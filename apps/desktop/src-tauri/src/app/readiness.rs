// Application readiness state.
//
// Managed before `setup` runs so a readiness probe works even while database
// and AppState initialization is still in flight or has failed. Events remain
// the fast path; the probe exists for panels mounted after startup.

use std::sync::{Arc, RwLock};

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AppReadinessStatus {
    Initializing,
    Ready,
    Failed,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppReadinessSnapshot {
    pub status: AppReadinessStatus,
    pub error_code: Option<String>,
}

impl Default for AppReadinessSnapshot {
    fn default() -> Self {
        Self {
            status: AppReadinessStatus::Initializing,
            error_code: None,
        }
    }
}

#[derive(Clone, Default)]
pub struct AppReadiness {
    state: Arc<RwLock<AppReadinessSnapshot>>,
}

impl AppReadiness {
    pub fn mark_ready(&self) {
        if let Ok(mut state) = self.state.write() {
            *state = AppReadinessSnapshot {
                status: AppReadinessStatus::Ready,
                error_code: None,
            };
        }
    }

    pub fn mark_failed(&self, error_code: &str) {
        if let Ok(mut state) = self.state.write() {
            *state = AppReadinessSnapshot {
                status: AppReadinessStatus::Failed,
                error_code: Some(error_code.to_string()),
            };
        }
    }

    pub fn snapshot(&self) -> AppReadinessSnapshot {
        self.state
            .read()
            .map(|state| state.clone())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::{AppReadiness, AppReadinessStatus};

    #[test]
    fn starts_initializing_and_tracks_terminal_states() {
        let readiness = AppReadiness::default();
        assert_eq!(
            readiness.snapshot().status,
            AppReadinessStatus::Initializing
        );

        readiness.mark_failed("DB_INIT_FAILED");
        let failed = readiness.snapshot();
        assert_eq!(failed.status, AppReadinessStatus::Failed);
        assert_eq!(failed.error_code.as_deref(), Some("DB_INIT_FAILED"));

        readiness.mark_ready();
        let ready = readiness.snapshot();
        assert_eq!(ready.status, AppReadinessStatus::Ready);
        assert!(ready.error_code.is_none());
    }

    #[test]
    fn serializes_with_camel_case_status() {
        let readiness = AppReadiness::default();
        let value = serde_json::to_value(readiness.snapshot()).expect("serializes");
        assert_eq!(value["status"], "initializing");
        assert!(value["errorCode"].is_null());
    }
}
