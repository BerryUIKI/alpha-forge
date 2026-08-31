use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::ChildStderr;

const MAX_STDERR_LINES: usize = 100;
const MAX_STDERR_BYTES: usize = 64 * 1024;

/// Execution diagnostics recorded during a supervised worker run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunDiagnostics {
    pub run_id: String,
    pub worker_id: String,
    pub worker_version: String,
    pub startup_latency_ms: u64,
    pub total_duration_ms: u64,
    pub frames_received: usize,
    pub frames_sent: usize,
    pub total_bytes_received: usize,
    pub total_bytes_sent: usize,
    pub graceful_exit: bool,
    pub exit_status: Option<String>,
    pub recent_stderr: Vec<String>,
}

/// Bounded stderr collector that reads from child stderr asynchronously.
#[derive(Clone, Default)]
pub struct StderrCollector {
    lines: Arc<Mutex<VecDeque<String>>>,
    total_bytes: Arc<Mutex<usize>>,
}

impl StderrCollector {
    pub fn new() -> Self {
        Self {
            lines: Arc::new(Mutex::new(VecDeque::new())),
            total_bytes: Arc::new(Mutex::new(0)),
        }
    }

    /// Spawns an async reader task to consume stderr until EOF.
    pub fn spawn_reader(self, stderr: ChildStderr) {
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                self.push_line(line);
            }
        });
    }

    pub fn push_line(&self, mut line: String) {
        // Redact simple sensitive patterns if encountered
        if line.contains("api_key") || line.contains("apiKey") || line.contains("sk-") {
            line = "[REDACTED SENSITIVE OUTPUT]".into();
        }

        let mut lines = self.lines.lock().unwrap();
        let mut total_bytes = self.total_bytes.lock().unwrap();

        let line_len = line.len();
        *total_bytes += line_len;

        lines.push_back(line);

        while lines.len() > MAX_STDERR_LINES || *total_bytes > MAX_STDERR_BYTES {
            if let Some(removed) = lines.pop_front() {
                *total_bytes = total_bytes.saturating_sub(removed.len());
            } else {
                break;
            }
        }
    }

    pub fn get_lines(&self) -> Vec<String> {
        let lines = self.lines.lock().unwrap();
        lines.iter().cloned().collect()
    }
}

/// Helper timer tracking duration across phases.
pub struct Stopwatch {
    start: Instant,
}

impl Stopwatch {
    pub fn start() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
}
