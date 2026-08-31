use crate::error::{SupervisorError, SupervisorResult};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Metadata describing an approved and allowlisted worker executable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerManifest {
    pub id: String,
    pub version: String,
    pub executable_name: String,
    pub protocol_versions: Vec<u32>,
    pub capabilities: Vec<String>,
    pub sha256_digest: Option<String>,
    pub is_packaged: bool,
}

impl WorkerManifest {
    pub fn native_worker() -> Self {
        Self {
            id: "alphaforge-native-worker".into(),
            version: "0.1.0".into(),
            executable_name: "alphaforge-agent-worker".into(),
            protocol_versions: vec![1],
            capabilities: vec!["provider.broker".into(), "tool.broker".into()],
            sha256_digest: None,
            is_packaged: true,
        }
    }

    pub fn fixture_worker() -> Self {
        Self {
            id: "alphaforge-fixture-worker".into(),
            version: "0.1.0".into(),
            executable_name: "alphaforge-agent-worker".into(),
            protocol_versions: vec![1],
            capabilities: vec!["provider.broker".into(), "tool.broker".into()],
            sha256_digest: None,
            is_packaged: false,
        }
    }
}

/// In-memory registry of approved worker manifests with secure path resolution.
#[derive(Debug, Clone, Default)]
pub struct WorkerRegistry {
    manifests: HashMap<String, WorkerManifest>,
    search_paths: Vec<PathBuf>,
}

impl WorkerRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            manifests: HashMap::new(),
            search_paths: Vec::new(),
        };
        registry.register(WorkerManifest::native_worker());
        registry.register(WorkerManifest::fixture_worker());
        registry
    }

    pub fn register(&mut self, manifest: WorkerManifest) {
        self.manifests.insert(manifest.id.clone(), manifest);
    }

    pub fn get(&self, id: &str) -> Option<&WorkerManifest> {
        self.manifests.get(id)
    }

    pub fn add_search_path(&mut self, path: impl Into<PathBuf>) {
        self.search_paths.push(path.into());
    }

    /// Resolves the absolute verified path to the worker executable.
    pub fn resolve_executable(
        &self,
        manifest_id: &str,
    ) -> SupervisorResult<(WorkerManifest, PathBuf)> {
        let manifest = self
            .get(manifest_id)
            .ok_or_else(|| SupervisorError::BinaryNotFound(manifest_id.to_string()))?
            .clone();

        // 1. Search candidate directories
        let mut candidates = Vec::new();

        // Check explicit search paths
        for base in &self.search_paths {
            candidates.push(base.join(&manifest.executable_name));
            #[cfg(windows)]
            candidates.push(base.join(format!("{}.exe", manifest.executable_name)));
        }

        // Check current executable directory (for packaged Tauri apps)
        if let Ok(current_exe) = std::env::current_exe() {
            if let Some(parent) = current_exe.parent() {
                candidates.push(parent.join(&manifest.executable_name));
                #[cfg(windows)]
                candidates.push(parent.join(format!("{}.exe", manifest.executable_name)));
            }
        }

        // Check target/debug or target/release build outputs in development
        if let Ok(cargo_bin) = std::env::var("CARGO_BIN_EXE_alphaforge-agent-worker") {
            candidates.push(PathBuf::from(cargo_bin));
        }

        // Find first existing valid file
        let resolved_path = candidates
            .into_iter()
            .find(|p| p.is_file())
            .ok_or_else(|| SupervisorError::BinaryNotFound(manifest.executable_name.clone()))?;

        let canonical_path = resolved_path.canonicalize().map_err(|e| {
            SupervisorError::InvalidBinaryPath(format!("{}: {}", resolved_path.display(), e))
        })?;

        // 2. Integrity check if digest is configured
        if let Some(ref expected_digest) = manifest.sha256_digest {
            verify_file_digest(&canonical_path, expected_digest)?;
        }

        Ok((manifest, canonical_path))
    }
}

fn verify_file_digest(path: &Path, expected_hex: &str) -> SupervisorResult<()> {
    let bytes = std::fs::read(path).map_err(SupervisorError::SpawnFailed)?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let digest = hasher.finalize();
    let actual_hex = format!("{:x}", digest);

    if !actual_hex.eq_ignore_ascii_case(expected_hex) {
        return Err(SupervisorError::IntegrityMismatch {
            expected: expected_hex.to_string(),
            actual: actual_hex,
        });
    }

    Ok(())
}
