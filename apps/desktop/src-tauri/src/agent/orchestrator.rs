// Agent subprocess orchestrator.
//
// Governed by ADR-0010 and SUBPROCESS_ROADMAP.md (AW4).
// Manages the end-to-end execution loop between the desktop host,
// isolated worker subprocess, and provider/tool brokers.

use std::sync::Arc;
use std::time::Duration;
use tauri::AppHandle;
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

use crate::agent::events;
use crate::database::repositories::agent_task_repository::AgentTaskRepository;
use crate::error::AppError;
use agent_core::broker::{ProviderBroker, ToolBroker};
use agent_core::manager::SupervisorManager;
use agent_protocol::messages::*;
use domain::task::{AgentTask, TaskEventType, TaskStatus};

/// Orchestrator executing agent tasks in supervised subprocesses.
pub struct AgentOrchestrator {
    repo: Arc<Mutex<AgentTaskRepository>>,
    app: AppHandle,
    supervisor_manager: Arc<SupervisorManager>,
    provider_broker: Arc<ProviderBroker>,
    tool_broker: Arc<ToolBroker>,
}

impl AgentOrchestrator {
    pub fn new(
        repo: AgentTaskRepository,
        app: AppHandle,
        supervisor_manager: Arc<SupervisorManager>,
        provider_broker: Arc<ProviderBroker>,
        tool_broker: Arc<ToolBroker>,
    ) -> Self {
        Self {
            repo: Arc::new(Mutex::new(repo)),
            app,
            supervisor_manager,
            provider_broker,
            tool_broker,
        }
    }

    /// Starts and supervises an agent task execution within an isolated subprocess.
    pub async fn execute_task(
        &self,
        task: AgentTask,
        manifest_id: &str,
        timeout: Duration,
    ) -> Result<(), AppError> {
        let task_id = task.id.clone();
        let workspace_id = task.workspace_id.clone();

        info!(task_id = %task_id, manifest_id, "Orchestrating agent subprocess execution");

        Self::record_progress(
            &self.repo,
            &self.app,
            &task_id,
            "Spawning isolated worker subprocess...",
        )
        .await?;

        // 1. Spawn worker subprocess
        let supervisor_arc = match self
            .supervisor_manager
            .spawn_worker(&task_id, manifest_id)
            .await
        {
            Ok(sup) => sup,
            Err(err) => {
                let msg = format!("Failed to spawn worker subprocess: {}", err);
                error!(task_id = %task_id, %err, "Worker spawn failed");
                return Self::record_failure(&self.repo, &self.app, &task_id, &msg).await;
            }
        };

        // 2. Perform handshake
        let run_scope = RunScope {
            workspace_id,
            task_id: task_id.clone(),
            session_id: None,
        };

        let task_input = serde_json::json!({
            "title": task.title,
            "description": task.description.as_deref().unwrap_or("")
        });

        {
            let mut supervisor = supervisor_arc.lock().await;
            if let Err(err) = supervisor
                .perform_handshake(
                    run_scope.clone(),
                    vec!["provider.broker".into(), "tool.broker".into()],
                    task_input,
                    "alphaforge.research.completion",
                )
                .await
            {
                let msg = format!("Worker handshake failed: {}", err);
                warn!(task_id = %task_id, %err, "Handshake failure");
                return Self::record_failure(&self.repo, &self.app, &task_id, &msg).await;
            }
        }

        Self::record_progress(
            &self.repo,
            &self.app,
            &task_id,
            "Worker handshake verified; executing research run...",
        )
        .await?;

        // 3. Runtime execution message loop
        let loop_future = async {
            let mut completed = false;
            loop {
                let frame_opt = {
                    let mut supervisor = supervisor_arc.lock().await;
                    supervisor.read_typed_frame().await
                };

                let typed = match frame_opt {
                    Ok(Some(frame)) => frame,
                    Ok(None) => {
                        debug!(task_id = %task_id, "Worker closed communication channel");
                        break;
                    }
                    Err(err) => {
                        warn!(task_id = %task_id, %err, "Worker protocol error during execution");
                        return Err(AppError::Internal(format!(
                            "Worker protocol error: {}",
                            err
                        )));
                    }
                };

                match typed.payload {
                    MessagePayload::RunProgress(prog) => {
                        Self::record_progress(&self.repo, &self.app, &task_id, &prog.message)
                            .await?;
                    }
                    MessagePayload::ProviderRequest(req) => {
                        debug!(task_id = %task_id, req_id = %req.request_id, "Dispatching brokered model call");
                        let res = self
                            .provider_broker
                            .handle_request(&req)
                            .await
                            .map_err(|e| {
                                AppError::Internal(format!("Provider broker error: {}", e))
                            })?;
                        let mut supervisor = supervisor_arc.lock().await;
                        supervisor
                            .send_typed_message(MessagePayload::ProviderResponse(res))
                            .await
                            .map_err(|e| {
                                AppError::Internal(format!(
                                    "Failed to return provider response: {}",
                                    e
                                ))
                            })?;
                    }
                    MessagePayload::ToolRequest(req) => {
                        debug!(task_id = %task_id, tool = %req.tool_name, "Dispatching brokered tool call");
                        let res = self
                            .tool_broker
                            .handle_request(&run_scope, &req)
                            .await
                            .map_err(|e| AppError::Internal(format!("Tool broker error: {}", e)))?;
                        let mut supervisor = supervisor_arc.lock().await;
                        supervisor
                            .send_typed_message(MessagePayload::ToolResponse(res))
                            .await
                            .map_err(|e| {
                                AppError::Internal(format!("Failed to return tool response: {}", e))
                            })?;
                    }
                    MessagePayload::ProposalCreated(prop) => {
                        info!(task_id = %task_id, title = %prop.title, "Worker created human proposal");
                        // Proposal recorded in task progress
                        let prop_msg = format!("Proposed {}: {}", prop.proposal_type, prop.title);
                        Self::record_progress(&self.repo, &self.app, &task_id, &prop_msg).await?;
                    }
                    MessagePayload::RunResult(res) => {
                        let output = serde_json::to_string(&res.result).unwrap_or_default();
                        {
                            let repo_guard = self.repo.lock().await;
                            repo_guard
                                .update_status(&task_id, TaskStatus::Completed)
                                .await?;
                            repo_guard
                                .create_event(
                                    &task_id,
                                    TaskEventType::TaskCompleted,
                                    Some(output.clone()),
                                )
                                .await?;
                        }
                        events::emit_completion(&self.app, &task_id, Some(&output));
                        completed = true;
                        break;
                    }
                    MessagePayload::RunFailure(fail) => {
                        let fail_msg = format!("Worker failed [{}]: {}", fail.code, fail.message);
                        Self::record_failure(&self.repo, &self.app, &task_id, &fail_msg).await?;
                        completed = true;
                        break;
                    }
                    _ => {}
                }
            }

            if !completed {
                warn!(task_id = %task_id, "Worker subprocess terminated unexpectedly without terminal status");
                return Err(AppError::Internal(
                    "Worker subprocess terminated unexpectedly without reporting a final status"
                        .into(),
                ));
            }

            Ok::<(), AppError>(())
        };

        // Enforce execution timeout
        let result = match tokio::time::timeout(timeout, loop_future).await {
            Ok(Ok(())) => Ok(()),
            Ok(Err(err)) => {
                let mut supervisor = supervisor_arc.lock().await;
                let _ = supervisor.cancel_gracefully(1000).await;
                Self::record_failure(&self.repo, &self.app, &task_id, &err.to_string()).await
            }
            Err(_) => {
                warn!(task_id = %task_id, "Worker execution timed out");
                let mut supervisor = supervisor_arc.lock().await;
                let _ = supervisor.cancel_gracefully(1000).await;
                Self::record_failure(
                    &self.repo,
                    &self.app,
                    &task_id,
                    "The research task timed out before completing.",
                )
                .await
            }
        };

        // Always unregister worker upon completion to release concurrency slots
        self.supervisor_manager.unregister_worker(&task_id);

        result
    }

    async fn record_progress(
        repo: &Arc<Mutex<AgentTaskRepository>>,
        app: &AppHandle,
        task_id: &str,
        message: &str,
    ) -> Result<(), AppError> {
        let repo_guard = repo.lock().await;
        repo_guard
            .create_event(
                task_id,
                TaskEventType::TaskProgress,
                Some(message.to_string()),
            )
            .await?;
        events::emit_progress(app, task_id, message);
        Ok(())
    }

    async fn record_failure(
        repo: &Arc<Mutex<AgentTaskRepository>>,
        app: &AppHandle,
        task_id: &str,
        message: &str,
    ) -> Result<(), AppError> {
        let repo_guard = repo.lock().await;
        repo_guard
            .update_status(task_id, TaskStatus::Failed)
            .await?;
        repo_guard
            .create_event(
                task_id,
                TaskEventType::TaskFailed,
                Some(message.to_string()),
            )
            .await?;
        events::emit_failure(app, task_id, message);
        Ok(())
    }
}
