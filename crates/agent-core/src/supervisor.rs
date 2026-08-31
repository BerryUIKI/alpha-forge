use crate::diagnostics::{RunDiagnostics, StderrCollector, Stopwatch};
use crate::error::{SupervisorError, SupervisorResult};
use crate::launch::LaunchSpec;
use crate::manifest::WorkerManifest;
use agent_protocol::codec::async_codec::{AsyncFrameReader, AsyncFrameWriter};
use agent_protocol::envelope::{RawEnvelope, TypedEnvelope};
use agent_protocol::error::ProtocolError;
use agent_protocol::messages::*;
use agent_protocol::validator::SessionValidator;
use std::process::Stdio;
use std::time::Duration;
use tempfile::TempDir;
use tokio::io::BufReader;
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Active supervisor instance managing a single isolated worker subprocess.
pub struct WorkerSupervisor {
    run_id: String,
    manifest: WorkerManifest,
    spec: LaunchSpec,
    child: Option<Child>,
    writer: Option<AsyncFrameWriter<ChildStdin>>,
    reader: Option<AsyncFrameReader<BufReader<ChildStdout>>>,
    validator: SessionValidator,
    stderr_collector: StderrCollector,
    _temp_dir: Option<TempDir>,
    stopwatch: Stopwatch,
    startup_latency_ms: u64,
    frames_sent: usize,
    frames_received: usize,
}

impl WorkerSupervisor {
    pub fn new(
        run_id: impl Into<String>,
        manifest: WorkerManifest,
        spec: LaunchSpec,
        temp_dir: Option<TempDir>,
    ) -> Self {
        let run_id_str = run_id.into();
        let validator = SessionValidator::new(&run_id_str);
        Self {
            run_id: run_id_str,
            manifest,
            spec,
            child: None,
            writer: None,
            reader: None,
            validator,
            stderr_collector: StderrCollector::new(),
            _temp_dir: temp_dir,
            stopwatch: Stopwatch::start(),
            startup_latency_ms: 0,
            frames_sent: 0,
            frames_received: 0,
        }
    }

    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn manifest(&self) -> &WorkerManifest {
        &self.manifest
    }

    /// Spawns the subprocess directly with isolated environment and piped handles.
    pub async fn spawn(&mut self) -> SupervisorResult<()> {
        let mut cmd = Command::new(&self.spec.executable_path);
        cmd.args(&self.spec.args);
        cmd.current_dir(&self.spec.working_dir);

        // Sanitize environment: clear and set only allowlisted variables
        cmd.env_clear();
        for (k, v) in &self.spec.env {
            cmd.env(k, v);
        }

        // Configure pipes and kill-on-drop fallback
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        cmd.kill_on_drop(true);

        info!(
            run_id = %self.run_id,
            binary = %self.spec.executable_path.display(),
            "Spawning worker subprocess"
        );

        let mut child = cmd.spawn()?;

        let stdin = child.stdin.take().ok_or_else(|| {
            SupervisorError::Internal("Failed to capture worker stdin pipe".into())
        })?;
        let stdout = child.stdout.take().ok_or_else(|| {
            SupervisorError::Internal("Failed to capture worker stdout pipe".into())
        })?;
        let stderr = child.stderr.take().ok_or_else(|| {
            SupervisorError::Internal("Failed to capture worker stderr pipe".into())
        })?;

        // Start async stderr collector
        self.stderr_collector.clone().spawn_reader(stderr);

        let max_frame = self.spec.run_limits.max_frame_bytes.unwrap_or(1024 * 1024);
        let max_agg = self
            .spec
            .run_limits
            .max_aggregate_bytes
            .unwrap_or(16 * 1024 * 1024);

        self.writer = Some(AsyncFrameWriter::with_max_frame_bytes(stdin, max_frame));
        self.reader = Some(AsyncFrameReader::with_limits(
            BufReader::new(stdout),
            max_frame,
            max_agg,
        ));
        self.child = Some(child);

        Ok(())
    }

    /// Performs the full 4-step startup handshake within the configured timeout.
    pub async fn perform_handshake(
        &mut self,
        run_scope: RunScope,
        capabilities: Vec<String>,
        task_input: serde_json::Value,
        output_schema_id: &str,
    ) -> SupervisorResult<()> {
        let timeout_ms = self.spec.startup_timeout_ms;
        let handshake_future =
            self.handshake_inner(run_scope, capabilities, task_input, output_schema_id);

        match tokio::time::timeout(Duration::from_millis(timeout_ms), handshake_future).await {
            Ok(res) => res,
            Err(_) => {
                warn!(run_id = %self.run_id, timeout_ms, "Worker handshake timed out");
                let _ = self.kill().await;
                Err(SupervisorError::HandshakeTimeout { timeout_ms })
            }
        }
    }

    async fn handshake_inner(
        &mut self,
        run_scope: RunScope,
        capabilities: Vec<String>,
        task_input: serde_json::Value,
        output_schema_id: &str,
    ) -> SupervisorResult<()> {
        let handshake_timer = Stopwatch::start();

        // 1. Read worker.hello
        let raw_hello = self
            .read_raw_frame()
            .await?
            .ok_or(SupervisorError::Protocol(ProtocolError::UnexpectedEof))?;

        self.validator.validate_incoming(&raw_hello)?;
        let _typed_hello = raw_hello.to_typed()?;

        // 2. Send host.configure with secure nonce
        let nonce = Uuid::new_v4().to_string();
        self.validator.set_expected_nonce(&nonce);

        let configure = TypedEnvelope::from_payload(
            &self.run_id,
            MessagePayload::HostConfigure(HostConfigure {
                selected_version: 1,
                run_scope,
                limits: self.spec.run_limits.clone(),
                capabilities,
                nonce,
            }),
        );
        let raw_configure = configure.into_raw()?;
        self.validator.record_outgoing(&raw_configure)?;
        self.write_raw_frame(&raw_configure).await?;

        // 3. Read worker.ready
        let raw_ready = self
            .read_raw_frame()
            .await?
            .ok_or(SupervisorError::Protocol(ProtocolError::UnexpectedEof))?;

        self.validator.validate_incoming(&raw_ready)?;
        let _typed_ready = raw_ready.to_typed()?;

        // 4. Send host.start
        let start = TypedEnvelope::from_payload(
            &self.run_id,
            MessagePayload::HostStart(HostStart {
                task_input,
                output_schema_id: output_schema_id.to_string(),
                output_schema_version: 1,
            }),
        );
        let raw_start = start.into_raw()?;
        self.validator.record_outgoing(&raw_start)?;
        self.write_raw_frame(&raw_start).await?;

        self.startup_latency_ms = handshake_timer.elapsed_ms();
        debug!(
            run_id = %self.run_id,
            latency_ms = self.startup_latency_ms,
            "Worker handshake completed successfully"
        );

        Ok(())
    }

    /// Reads the next raw frame from the worker.
    pub async fn read_raw_frame(&mut self) -> SupervisorResult<Option<RawEnvelope>> {
        let reader = self
            .reader
            .as_mut()
            .ok_or_else(|| SupervisorError::Internal("Reader not initialized".into()))?;

        let frame = reader.read_frame().await?;
        if frame.is_some() {
            self.frames_received += 1;
        }
        Ok(frame)
    }

    /// Reads and validates the next typed message from the worker.
    pub async fn read_typed_frame(&mut self) -> SupervisorResult<Option<TypedEnvelope>> {
        if let Some(raw) = self.read_raw_frame().await? {
            self.validator.validate_incoming(&raw)?;
            let typed = raw.to_typed()?;
            Ok(Some(typed))
        } else {
            Ok(None)
        }
    }

    /// Writes a raw frame to the worker subprocess.
    pub async fn write_raw_frame(&mut self, envelope: &RawEnvelope) -> SupervisorResult<()> {
        let writer = self
            .writer
            .as_mut()
            .ok_or_else(|| SupervisorError::Internal("Writer not initialized".into()))?;

        writer.write_frame(envelope).await?;
        self.frames_sent += 1;
        Ok(())
    }

    /// Sends a typed message to the worker.
    pub async fn send_typed_message(&mut self, payload: MessagePayload) -> SupervisorResult<()> {
        let typed = TypedEnvelope::from_payload(&self.run_id, payload);
        let raw = typed.into_raw()?;
        self.validator.record_outgoing(&raw)?;
        self.write_raw_frame(&raw).await?;
        Ok(())
    }

    /// Gracefully requests task cancellation, waiting up to `grace_period_ms` before force-killing.
    pub async fn cancel_gracefully(&mut self, grace_period_ms: u64) -> SupervisorResult<()> {
        info!(run_id = %self.run_id, grace_period_ms, "Cancelling worker gracefully");

        // Send cancel message with a bounded timeout to avoid deadlocking if stdin pipe buffer is full
        let cancel = MessagePayload::RunCancel(RunCancel {
            reason: "User requested task cancellation".into(),
        });
        let _ =
            tokio::time::timeout(Duration::from_millis(250), self.send_typed_message(cancel)).await;

        if let Some(ref mut child) = self.child {
            match tokio::time::timeout(Duration::from_millis(grace_period_ms), child.wait()).await {
                Ok(Ok(status)) => {
                    debug!(run_id = %self.run_id, ?status, "Worker exited cleanly after cancellation");
                    return Ok(());
                }
                _ => {
                    warn!(run_id = %self.run_id, "Worker did not exit within grace period, force killing");
                    let _ = child.kill().await;
                    let _ = child.wait().await;
                }
            }
        }

        Ok(())
    }

    /// Force terminates the child process and waits for it to be reaped.
    pub async fn kill(&mut self) -> SupervisorResult<()> {
        if let Some(ref mut child) = self.child {
            let _ = child.kill().await;
            let _ = child.wait().await;
        }
        Ok(())
    }

    /// Collects final execution diagnostics for the run.
    pub fn collect_diagnostics(
        &self,
        graceful_exit: bool,
        exit_status: Option<String>,
    ) -> RunDiagnostics {
        let total_bytes_received = self
            .reader
            .as_ref()
            .map(|r| r.total_bytes_read())
            .unwrap_or(0);
        let total_bytes_sent = self
            .writer
            .as_ref()
            .map(|w| w.total_bytes_written())
            .unwrap_or(0);

        RunDiagnostics {
            run_id: self.run_id.clone(),
            worker_id: self.manifest.id.clone(),
            worker_version: self.manifest.version.clone(),
            startup_latency_ms: self.startup_latency_ms,
            total_duration_ms: self.stopwatch.elapsed_ms(),
            frames_received: self.frames_received,
            frames_sent: self.frames_sent,
            total_bytes_received,
            total_bytes_sent,
            graceful_exit,
            exit_status,
            recent_stderr: self.stderr_collector.get_lines(),
        }
    }
}
