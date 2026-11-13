// End-to-end command tests against a real SQLite database.
//
// These tests exercise the same service wiring as the `alphaforge` binary with
// injected credentials and a deterministic research provider, so they cover the
// complete workspace/task/provider/artifact flows without network access.

use std::collections::HashMap;
use std::process::ExitCode;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use alpha_forge_lib::error::AppError;
use alpha_forge_lib::security::credentials::CredentialStore;
use alphaforge_cli::cli::{ArtifactCommand, ProviderCommand, TaskCommand, WorkspaceCommand};
use alphaforge_cli::commands;
use alphaforge_cli::context::Services;
use alphaforge_cli::output::OutputFormat;
use async_trait::async_trait;
use domain::artifact::{ArtifactType, CreateArtifactInput};
use domain::task::{TaskEventType, TaskStatus};
use provider_core::{
    ProviderError, ResearchCompletion, ResearchCompletionRequest, ResearchProvider,
};
use serde_json::Value;

#[derive(Default)]
struct FakeCredentialStore {
    values: Mutex<HashMap<String, String>>,
}

impl CredentialStore for FakeCredentialStore {
    fn set(&self, credential_name: &str, secret: &str) -> Result<(), AppError> {
        self.values
            .lock()
            .expect("credential store lock")
            .insert(credential_name.to_string(), secret.to_string());
        Ok(())
    }

    fn get(&self, credential_name: &str) -> Result<Option<String>, AppError> {
        Ok(self
            .values
            .lock()
            .expect("credential store lock")
            .get(credential_name)
            .cloned())
    }

    fn delete(&self, credential_name: &str) -> Result<(), AppError> {
        self.values
            .lock()
            .expect("credential store lock")
            .remove(credential_name);
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum ProviderMode {
    Success,
    Failure,
    Slow,
}

struct FakeResearchProvider {
    mode: ProviderMode,
}

#[async_trait]
impl ResearchProvider for FakeResearchProvider {
    async fn complete_research(
        &self,
        _request: ResearchCompletionRequest,
    ) -> Result<ResearchCompletion, ProviderError> {
        match self.mode {
            ProviderMode::Success => Ok(ResearchCompletion {
                summary: "Demand is stable and evidence is mixed.".to_string(),
                claims: vec!["Order backlog is steady.".to_string()],
                evidence: vec!["Quarterly filing excerpt.".to_string()],
                risks: vec!["Demand may soften.".to_string()],
                confidence: 60,
            }),
            ProviderMode::Failure => Err(ProviderError::RequestFailed),
            ProviderMode::Slow => {
                tokio::time::sleep(Duration::from_secs(30)).await;
                Err(ProviderError::RequestFailed)
            }
        }
    }
}

struct TestContext {
    services: Services,
    credentials: Arc<FakeCredentialStore>,
    _directory: tempfile::TempDir,
}

async fn setup(mode: ProviderMode) -> TestContext {
    let directory = tempfile::tempdir().expect("temporary directory");
    let database_path = directory.path().join("alpha_forge.db");
    let credentials = Arc::new(FakeCredentialStore::default());
    let provider: Arc<dyn ResearchProvider> = Arc::new(FakeResearchProvider { mode });

    let services = Services::open_with(Some(&database_path), credentials.clone(), provider)
        .await
        .expect("services should open and migrate");

    TestContext {
        services,
        credentials,
        _directory: directory,
    }
}

async fn create_workspace(services: &Services, name: &str) -> String {
    let mut out = Vec::new();
    commands::workspace::run(
        services,
        WorkspaceCommand::Create {
            name: name.to_string(),
        },
        OutputFormat::Json,
        &mut out,
    )
    .await
    .expect("workspace creation should succeed");

    let value: Value = serde_json::from_slice(&out).expect("workspace JSON");
    value["id"].as_str().expect("workspace id").to_string()
}

async fn create_task(services: &Services, workspace_id: &str, title: &str) -> String {
    let mut out = Vec::new();
    commands::task::run(
        services,
        TaskCommand::Create {
            workspace: workspace_id.to_string(),
            title: title.to_string(),
            description: Some("Compare demand drivers.".to_string()),
        },
        OutputFormat::Json,
        &mut out,
    )
    .await
    .expect("task creation should succeed");

    let value: Value = serde_json::from_slice(&out).expect("task JSON");
    value["id"].as_str().expect("task id").to_string()
}

#[tokio::test]
async fn workspace_create_and_list_return_json_records() {
    let context = setup(ProviderMode::Success).await;

    let mut out = Vec::new();
    commands::workspace::run(
        &context.services,
        WorkspaceCommand::List,
        OutputFormat::Json,
        &mut out,
    )
    .await
    .expect("empty list should succeed");
    let empty: Value = serde_json::from_slice(&out).expect("list JSON");
    assert_eq!(empty, Value::Array(Vec::new()));

    let workspace_id = create_workspace(&context.services, "Alpha Research").await;

    let mut out = Vec::new();
    commands::workspace::run(
        &context.services,
        WorkspaceCommand::List,
        OutputFormat::Json,
        &mut out,
    )
    .await
    .expect("list should succeed");
    let workspaces: Value = serde_json::from_slice(&out).expect("list JSON");
    assert_eq!(workspaces[0]["id"], workspace_id);
    assert_eq!(workspaces[0]["name"], "Alpha Research");

    let mut text = Vec::new();
    commands::workspace::run(
        &context.services,
        WorkspaceCommand::List,
        OutputFormat::Text,
        &mut text,
    )
    .await
    .expect("text list should succeed");
    let text = String::from_utf8(text).expect("utf-8 output");
    assert!(text.contains("Alpha Research"));
}

#[tokio::test]
async fn task_create_requires_an_existing_workspace() {
    let context = setup(ProviderMode::Success).await;

    let error = commands::task::run(
        &context.services,
        TaskCommand::Create {
            workspace: "missing-workspace".to_string(),
            title: "Assess demand".to_string(),
            description: None,
        },
        OutputFormat::Text,
        &mut Vec::new(),
    )
    .await
    .expect_err("unknown workspace should be rejected");

    assert!(matches!(
        error,
        alphaforge_cli::error::CliError::App(AppError::Validation(_))
    ));
}

#[tokio::test]
async fn task_create_and_list_show_the_created_state() {
    let context = setup(ProviderMode::Success).await;
    let workspace_id = create_workspace(&context.services, "Alpha Research").await;
    let task_id = create_task(&context.services, &workspace_id, "Assess demand").await;

    let mut out = Vec::new();
    commands::task::run(
        &context.services,
        TaskCommand::List {
            workspace: workspace_id,
        },
        OutputFormat::Json,
        &mut out,
    )
    .await
    .expect("task list should succeed");

    let tasks: Value = serde_json::from_slice(&out).expect("task list JSON");
    assert_eq!(tasks[0]["id"], task_id);
    assert_eq!(tasks[0]["status"], "created");
}

#[tokio::test]
async fn task_start_completes_and_persists_structured_output() {
    let context = setup(ProviderMode::Success).await;
    let workspace_id = create_workspace(&context.services, "Alpha Research").await;
    let task_id = create_task(&context.services, &workspace_id, "Assess demand").await;

    let mut out = Vec::new();
    let exit_code = commands::task::run(
        &context.services,
        TaskCommand::Start {
            id: task_id.clone(),
            timeout: Some(30),
        },
        OutputFormat::Text,
        &mut out,
    )
    .await
    .expect("task start should succeed");

    assert_eq!(exit_code, ExitCode::SUCCESS);

    let text = String::from_utf8(out).expect("utf-8 output");
    assert!(text.contains("Task started."));
    assert!(text.contains("Demand is stable and evidence is mixed."));
    assert!(text.contains("completed"));

    let task = context
        .services
        .agent
        .get_task(&task_id)
        .await
        .expect("task lookup")
        .expect("task should exist");
    assert_eq!(task.status, TaskStatus::Completed);

    let events = context
        .services
        .agent
        .get_task_events(&task_id)
        .await
        .expect("events lookup");
    let completion = events
        .iter()
        .find(|event| event.event_type == TaskEventType::TaskCompleted)
        .expect("completion event should be persisted");
    assert!(completion
        .payload
        .as_deref()
        .expect("completion payload")
        .contains("Demand is stable"));

    // JSON mode returns one valid document containing the task and events.
    let second_task_id = create_task(&context.services, &workspace_id, "Assess supply").await;
    let mut json_out = Vec::new();
    let exit_code = commands::task::run(
        &context.services,
        TaskCommand::Start {
            id: second_task_id,
            timeout: Some(30),
        },
        OutputFormat::Json,
        &mut json_out,
    )
    .await
    .expect("JSON task start should succeed");
    assert_eq!(exit_code, ExitCode::SUCCESS);

    let value: Value = serde_json::from_slice(&json_out).expect("task start JSON");
    assert_eq!(value["task"]["status"], "completed");
    assert!(value["events"]
        .as_array()
        .expect("events array")
        .iter()
        .any(|event| event["event_type"] == "task_completed"));
}

#[tokio::test]
async fn task_start_failure_returns_exit_code_two() {
    let context = setup(ProviderMode::Failure).await;
    let workspace_id = create_workspace(&context.services, "Alpha Research").await;
    let task_id = create_task(&context.services, &workspace_id, "Assess demand").await;

    let mut out = Vec::new();
    let exit_code = commands::task::run(
        &context.services,
        TaskCommand::Start {
            id: task_id.clone(),
            timeout: Some(30),
        },
        OutputFormat::Text,
        &mut out,
    )
    .await
    .expect("provider failure should be recorded, not raised");

    assert_eq!(exit_code, ExitCode::from(2));

    let text = String::from_utf8(out).expect("utf-8 output");
    assert!(text.contains("request failed"));

    let task = context
        .services
        .agent
        .get_task(&task_id)
        .await
        .expect("task lookup")
        .expect("task should exist");
    assert_eq!(task.status, TaskStatus::Failed);
}

#[tokio::test]
async fn task_start_timeout_cancels_the_running_task() {
    let context = setup(ProviderMode::Slow).await;
    let workspace_id = create_workspace(&context.services, "Alpha Research").await;
    let task_id = create_task(&context.services, &workspace_id, "Assess demand").await;

    let mut out = Vec::new();
    let exit_code = commands::task::run(
        &context.services,
        TaskCommand::Start {
            id: task_id.clone(),
            timeout: Some(1),
        },
        OutputFormat::Text,
        &mut out,
    )
    .await
    .expect("timeout should cancel rather than fail the command");

    assert_eq!(exit_code, ExitCode::from(3));

    let text = String::from_utf8(out).expect("utf-8 output");
    assert!(text.contains("Timeout reached; cancelling task..."));

    let task = context
        .services
        .agent
        .get_task(&task_id)
        .await
        .expect("task lookup")
        .expect("task should exist");
    assert_eq!(task.status, TaskStatus::Cancelled);
}

#[tokio::test]
async fn task_cancel_moves_a_pending_task_to_cancelled() {
    let context = setup(ProviderMode::Success).await;
    let workspace_id = create_workspace(&context.services, "Alpha Research").await;
    let task_id = create_task(&context.services, &workspace_id, "Assess demand").await;

    let mut out = Vec::new();
    let exit_code = commands::task::run(
        &context.services,
        TaskCommand::Cancel {
            id: task_id.clone(),
        },
        OutputFormat::Json,
        &mut out,
    )
    .await
    .expect("cancel should succeed");

    assert_eq!(exit_code, ExitCode::SUCCESS);

    let task: Value = serde_json::from_slice(&out).expect("task JSON");
    assert_eq!(task["status"], "cancelled");
}

#[tokio::test]
async fn provider_status_reflects_keychain_state_and_clear_key_removes_it() {
    let context = setup(ProviderMode::Success).await;

    let mut out = Vec::new();
    commands::provider::run(
        &context.services,
        ProviderCommand::Status,
        OutputFormat::Json,
        &mut out,
    )
    .expect("status should succeed");
    let status: Value = serde_json::from_slice(&out).expect("status JSON");
    assert_eq!(status["provider"], "openai");
    assert_eq!(status["configured"], false);

    alpha_forge_lib::security::credentials::save_openai_api_key(&*context.credentials, "test-key")
        .expect("key should be saved");

    let mut out = Vec::new();
    commands::provider::run(
        &context.services,
        ProviderCommand::Status,
        OutputFormat::Text,
        &mut out,
    )
    .expect("status should succeed");
    let text = String::from_utf8(out).expect("utf-8 output");
    assert!(text.contains("openai: configured"));

    let mut out = Vec::new();
    commands::provider::run(
        &context.services,
        ProviderCommand::ClearKey,
        OutputFormat::Json,
        &mut out,
    )
    .expect("clear should succeed");
    let status: Value = serde_json::from_slice(&out).expect("status JSON");
    assert_eq!(status["configured"], false);
    assert!(
        alpha_forge_lib::security::credentials::load_openai_api_key(&*context.credentials)
            .expect("credential lookup")
            .is_none()
    );
}

#[tokio::test]
async fn artifact_list_supports_global_workspace_and_task_filters() {
    let context = setup(ProviderMode::Success).await;
    let workspace_id = create_workspace(&context.services, "Alpha Research").await;
    let other_workspace_id = create_workspace(&context.services, "Second Workspace").await;
    let task_id = create_task(&context.services, &workspace_id, "Assess demand").await;

    let input = CreateArtifactInput {
        workspace_id: workspace_id.clone(),
        task_id: Some(task_id.clone()),
        artifact_type: ArtifactType::ComparisonTable,
        input: serde_json::json!({ "companies": ["AAPL", "MSFT"] }),
    };
    let artifact = context
        .services
        .artifacts
        .create_artifact(input)
        .await
        .expect("artifact creation should succeed");

    let mut out = Vec::new();
    commands::artifact::run(
        &context.services,
        ArtifactCommand::List {
            workspace: None,
            task: None,
        },
        OutputFormat::Json,
        &mut out,
    )
    .await
    .expect("global artifact list should succeed");
    let artifacts: Value = serde_json::from_slice(&out).expect("artifact JSON");
    assert_eq!(artifacts.as_array().map(Vec::len), Some(1));
    assert_eq!(artifacts[0]["id"], artifact.id);

    let mut out = Vec::new();
    commands::artifact::run(
        &context.services,
        ArtifactCommand::List {
            workspace: Some(other_workspace_id),
            task: None,
        },
        OutputFormat::Json,
        &mut out,
    )
    .await
    .expect("workspace filter should succeed");
    let artifacts: Value = serde_json::from_slice(&out).expect("artifact JSON");
    assert_eq!(artifacts.as_array().map(Vec::len), Some(0));

    let mut out = Vec::new();
    commands::artifact::run(
        &context.services,
        ArtifactCommand::List {
            workspace: Some(workspace_id),
            task: Some(task_id),
        },
        OutputFormat::Json,
        &mut out,
    )
    .await
    .expect("task filter should succeed");
    let artifacts: Value = serde_json::from_slice(&out).expect("artifact JSON");
    assert_eq!(artifacts.as_array().map(Vec::len), Some(1));
}
