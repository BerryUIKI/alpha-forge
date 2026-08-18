//! GooseAdapter trait and process supervisor
//!
//! Manages Goose sidecar lifecycle with security boundaries:
//! - Binary integrity verification
//! - Direct process spawn (no shell)
//! - Timeout and cancellation enforcement
//! - Output size limits
//! - Orphan process prevention

use std::path::PathBuf;
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::RwLock;
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

use crate::goose::config::{ExecutionBudget, GooseConfig};
use crate::goose::error::GooseError;
use crate::goose::output::StructuredResponse;
use crate::goose::recipe::Recipe;

/// Unique identifier for a Goose run
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RunId(pub u64);

impl RunId {
    pub fn new() -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed) as u64)
    }
}

impl Default for RunId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for RunId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "goose-run-{}", self.0)
    }
}

/// Status of a Goose run
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
    Timeout,
}

/// Progress event from Goose execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressEvent {
    pub run_id: RunId,
    pub status: RunStatus,
    pub message: String,
    pub turn: Option<u32>,
    pub tokens_used: Option<u64>,
}

/// Result of a successful Goose execution
#[derive(Debug, Clone)]
pub struct GooseResult {
    pub run_id: RunId,
    pub response: StructuredResponse,
    pub tokens_used: u64,
    pub duration: Duration,
    pub exit_code: i32,
}

/// Active process entry
struct ActiveProcess {
    #[allow(dead_code)]
    start_time: Instant,
    cancelled: Arc<AtomicBool>,
}

/// GooseAdapter manages Goose sidecar execution
pub struct GooseAdapter {
    config: GooseConfig,
    active_processes: RwLock<Vec<(RunId, ActiveProcess)>>,
}

impl GooseAdapter {
    /// Create a new GooseAdapter with the given configuration
    pub fn new(config: GooseConfig) -> Self {
        Self {
            config,
            active_processes: RwLock::new(Vec::new()),
        }
    }

    /// Verify binary integrity before execution
    pub async fn verify_binary(&self) -> Result<PathBuf, GooseError> {
        let path = &self.config.binary_path;

        if !path.exists() {
            return Err(GooseError::BinaryNotFound { path: path.clone() });
        }

        // Skip checksum verification if not configured (development mode)
        if self.config.binary_checksum.is_empty() {
            warn!("Binary checksum not configured, skipping integrity check");
            return Ok(path.clone());
        }

        // Calculate actual checksum
        let contents = tokio::fs::read(path).await.map_err(GooseError::Io)?;
        let mut hasher = Sha256::new();
        hasher.update(&contents);
        let actual = format!("{:x}", hasher.finalize());

        if actual != self.config.binary_checksum {
            return Err(GooseError::IntegrityCheckFailed {
                expected: self.config.binary_checksum.clone(),
                actual,
            });
        }

        debug!(path = ?path, "Binary integrity verified");
        Ok(path.clone())
    }

    /// Execute Goose with the given recipe and budget
    pub async fn execute(
        &self,
        run_id: RunId,
        recipe: &Recipe,
        budget: &ExecutionBudget,
    ) -> Result<GooseResult, GooseError> {
        // Verify binary before execution
        let binary_path = self.verify_binary().await?;

        // Validate recipe
        recipe.validate()?;

        // Create working directory
        let work_dir = &self.config.working_directory;
        tokio::fs::create_dir_all(work_dir)
            .await
            .map_err(GooseError::Io)?;

        // Write recipe to temp file
        let recipe_path = work_dir.join(format!("recipe-{}.yaml", run_id.0));
        recipe
            .write_to_file(&recipe_path)
            .await
            .map_err(|e| GooseError::Internal(format!("Failed to write recipe: {}", e)))?;

        // Build command (no shell)
        let mut cmd = Command::new(&binary_path);
        cmd.args(["run", "--recipe", &recipe_path.to_string_lossy()])
            .args(["--no-session"])
            .args(["--format", "json"])
            .current_dir(work_dir)
            .env("GOOSE_MODE", "auto")
            .env("GOOSE_CONTEXT_STRATEGY", "summarize")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        // Set max turns in environment
        cmd.env("GOOSE_MAX_TURNS", budget.max_turns.to_string());

        // Spawn process
        let mut child = cmd
            .spawn()
            .map_err(|e| GooseError::SpawnFailed { source: e })?;

        let pid = child.id().unwrap_or(0);
        info!(run_id = ?run_id, pid = pid, "Goose process started");

        // Track process state
        let cancelled = Arc::new(AtomicBool::new(false));
        let start_time = Instant::now();

        {
            let mut processes = self.active_processes.write().await;
            processes.push((
                run_id,
                ActiveProcess {
                    start_time,
                    cancelled: cancelled.clone(),
                },
            ));
        }

        // Wait for completion with timeout
        let result = self
            .wait_for_output(
                &mut child,
                run_id,
                budget.max_duration,
                cancelled.clone(),
                start_time,
            )
            .await;

        // Clean up process tracking
        {
            let mut processes = self.active_processes.write().await;
            processes.retain(|(id, _)| *id != run_id);
        }

        // Clean up recipe file
        if let Err(e) = tokio::fs::remove_file(&recipe_path).await {
            warn!("Failed to remove recipe file: {}", e);
        }

        result
    }

    /// Wait for process output with timeout and cancellation
    async fn wait_for_output(
        &self,
        child: &mut tokio::process::Child,
        run_id: RunId,
        max_duration: Duration,
        cancelled: Arc<AtomicBool>,
        start_time: Instant,
    ) -> Result<GooseResult, GooseError> {
        // Capture stdout and stderr
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        let mut output_buffer = Vec::new();

        // Read stderr in background
        let stderr_handle = if let Some(stderr) = stderr {
            let mut reader = BufReader::new(stderr).lines();
            Some(tokio::spawn(async move {
                let mut lines = Vec::new();
                while let Ok(Some(line)) = reader.next_line().await {
                    lines.push(redact_sensitive(&line));
                }
                lines
            }))
        } else {
            None
        };

        // Read stdout with size limit
        if let Some(mut stdout) = stdout {
            let mut buf = [0u8; 4096];
            loop {
                // Check cancellation
                if cancelled.load(Ordering::Relaxed) {
                    child.kill().await.ok();
                    return Err(GooseError::Cancelled);
                }

                match timeout(Duration::from_millis(100), stdout.read(&mut buf)).await {
                    Ok(Ok(0)) => break, // EOF
                    Ok(Ok(n)) => {
                        output_buffer.extend_from_slice(&buf[..n]);

                        // Check size limit
                        if output_buffer.len() > self.config.max_output_bytes {
                            child.kill().await.ok();
                            return Err(GooseError::OutputSizeExceeded {
                                size: output_buffer.len(),
                                limit: self.config.max_output_bytes,
                            });
                        }
                    }
                    Ok(Err(e)) => {
                        return Err(GooseError::Io(e));
                    }
                    Err(_) => continue, // timeout, check cancellation
                }

                // Check global timeout
                if start_time.elapsed() > max_duration {
                    child.kill().await.ok();
                    return Err(GooseError::Timeout {
                        timeout_ms: max_duration.as_millis() as u64,
                    });
                }
            }
        }

        // Wait for process to exit with timeout
        let wait_result = timeout(max_duration.saturating_sub(start_time.elapsed()), async {
            child.wait().await
        })
        .await;

        let status = match wait_result {
            Ok(Ok(status)) => status,
            Ok(Err(e)) => return Err(GooseError::Io(e)),
            Err(_) => {
                child.kill().await.ok();
                return Err(GooseError::Timeout {
                    timeout_ms: max_duration.as_millis() as u64,
                });
            }
        };

        let exit_code = status.code().unwrap_or(-1);

        // Collect stderr
        if let Some(handle) = stderr_handle {
            if let Ok(lines) = handle.await {
                for line in lines {
                    error!(run_id = ?run_id, stderr = %line);
                }
            }
        }

        if exit_code != 0 {
            return Err(GooseError::ExitCode { code: exit_code });
        }

        // Parse and validate output
        let response: StructuredResponse = serde_json::from_slice(&output_buffer)
            .map_err(|e| GooseError::OutputParseError { source: e })?;

        response.validate()?;

        let duration = start_time.elapsed();
        info!(
            run_id = ?run_id,
            duration_ms = duration.as_millis(),
            exit_code = exit_code,
            "Goose execution completed"
        );

        Ok(GooseResult {
            run_id,
            response,
            tokens_used: 0, // TODO: extract from response or logs
            duration,
            exit_code,
        })
    }

    /// Cancel a running Goose execution
    pub async fn cancel(&self, run_id: RunId) -> Result<(), GooseError> {
        let processes = self.active_processes.read().await;

        if let Some((_, state)) = processes.iter().find(|(id, _)| *id == run_id) {
            state.cancelled.store(true, Ordering::Relaxed);
            debug!(run_id = ?run_id, "Cancellation requested");
            Ok(())
        } else {
            Err(GooseError::Internal("Run not found".into()))
        }
    }

    /// Clean up all active processes (for shutdown)
    pub async fn shutdown(&self) {
        let mut processes = self.active_processes.write().await;

        for (run_id, state) in processes.iter_mut() {
            state.cancelled.store(true, Ordering::Relaxed);
            debug!(run_id = ?run_id, "Marking for cancellation during shutdown");
        }

        processes.clear();
        info!("All Goose processes marked for termination");
    }
}

/// Redact sensitive information from strings before logging
fn redact_sensitive(s: &str) -> String {
    // Redact common secret patterns
    let mut result = s.to_string();

    // Redact API keys
    if result.contains("key") || result.contains("token") || result.contains("secret") {
        result = "[REDACTED]".to_string();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_id_is_unique() {
        let id1 = RunId::new();
        let id2 = RunId::new();
        assert_ne!(id1, id2);
    }

    #[tokio::test]
    async fn binary_not_found_error() {
        let config = GooseConfig {
            binary_path: PathBuf::from("/nonexistent/goose"),
            ..Default::default()
        };

        let adapter = GooseAdapter::new(config);
        let result = adapter.verify_binary().await;
        assert!(matches!(result, Err(GooseError::BinaryNotFound { .. })));
    }
}
