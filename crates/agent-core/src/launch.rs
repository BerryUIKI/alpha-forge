use agent_protocol::messages::RunLimits;
use std::collections::HashMap;
use std::path::PathBuf;

/// Allowlisted environment variables safe to inherit by worker subprocesses.
const SAFE_ENV_ALLOWLIST: &[&str] = &[
    "PATH",
    "HOME",
    "USER",
    "LOGNAME",
    "TMPDIR",
    "TMP",
    "TEMP",
    "SYSTEMROOT",
    "SYSTEMDRIVE",
    "COMSPEC",
    "PATHEXT",
    "RUST_LOG",
    "RUST_BACKTRACE",
];

/// Specification for launching an isolated worker subprocess.
#[derive(Debug, Clone)]
pub struct LaunchSpec {
    pub executable_path: PathBuf,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub working_dir: PathBuf,
    pub startup_timeout_ms: u64,
    pub run_limits: RunLimits,
}

impl LaunchSpec {
    pub fn new(executable_path: PathBuf, working_dir: PathBuf) -> Self {
        let mut env = HashMap::new();

        // Populate from safe allowlist only
        for &key in SAFE_ENV_ALLOWLIST {
            if let Ok(val) = std::env::var(key) {
                env.insert(key.to_string(), val);
            }
        }

        Self {
            executable_path,
            args: Vec::new(),
            env,
            working_dir,
            startup_timeout_ms: 10_000,
            run_limits: RunLimits::default(),
        }
    }

    pub fn with_arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn with_args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for arg in args {
            self.args.push(arg.into());
        }
        self
    }

    pub fn with_env_var(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.env.insert(key.into(), val.into());
        self
    }

    pub fn with_startup_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.startup_timeout_ms = timeout_ms;
        self
    }

    pub fn with_limits(mut self, limits: RunLimits) -> Self {
        self.run_limits = limits;
        self
    }
}
