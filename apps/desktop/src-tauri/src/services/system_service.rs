// System service — handles system-level operations.

use crate::error::AppError;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::path::Path;
use std::time::Duration;
use tauri::Manager;

/// System information structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    pub app_name: String,
    pub app_version: String,
    pub platform: String,
    pub architecture: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseCheck {
    pub current_version: String,
    pub latest_version: String,
    pub release_url: String,
    pub update_available: bool,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
}

/// Service for system-level operations.
pub struct SystemService {
    app_handle: tauri::AppHandle,
    db_pool: SqlitePool,
}

impl SystemService {
    /// Creates a new system service.
    pub fn new(app_handle: tauri::AppHandle, db_pool: SqlitePool) -> Self {
        Self {
            app_handle,
            db_pool,
        }
    }

    /// Gets system information.
    pub fn get_info(&self) -> Result<SystemInfo, AppError> {
        let config = self.app_handle.config();

        Ok(SystemInfo {
            app_name: "Investment OS".to_string(),
            app_version: config
                .version
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
            platform: std::env::consts::OS.to_string(),
            architecture: std::env::consts::ARCH.to_string(),
        })
    }

    /// Performs health check.
    pub fn health_check(&self) -> Result<String, AppError> {
        // In a real application, this would check:
        // - Database connectivity
        // - Required services status
        // - Critical resources availability

        Ok("ok".to_string())
    }

    /// Gets application configuration directory.
    pub fn get_config_dir(&self) -> Result<std::path::PathBuf, AppError> {
        self.app_handle
            .path()
            .app_config_dir()
            .map_err(|e| AppError::Internal(format!("Failed to get config directory: {}", e)))
    }

    /// Gets application data directory.
    pub fn get_data_dir(&self) -> Result<std::path::PathBuf, AppError> {
        self.app_handle
            .path()
            .app_data_dir()
            .map_err(|e| AppError::Internal(format!("Failed to get data directory: {}", e)))
    }

    /// Exports a consistent SQLite backup selected through the native save dialog.
    pub async fn export_local_backup(&self) -> Result<Option<String>, AppError> {
        let destination = tauri::async_runtime::spawn_blocking(|| {
            rfd::FileDialog::new()
                .set_title("Export Investment OS local backup")
                .add_filter("SQLite database", &["db"])
                .set_file_name("investment-os-backup.db")
                .save_file()
        })
        .await
        .map_err(|_| AppError::Internal("backup file dialog did not complete".to_string()))?;

        let Some(destination) = destination else {
            return Ok(None);
        };
        if destination.exists() {
            return Err(AppError::Validation(
                "Choose a new backup filename; existing files are never overwritten".to_string(),
            ));
        }

        export_database_to(&self.db_pool, &destination).await?;
        Ok(Some(destination.to_string_lossy().to_string()))
    }

    /// Checks the public GitHub Releases feed without downloading or installing anything.
    pub async fn check_for_update(&self) -> Result<ReleaseCheck, AppError> {
        let current_version = self
            .app_handle
            .config()
            .version
            .clone()
            .unwrap_or_else(|| "0.0.0".to_string());
        let release = Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("InvestmentOS/0.1 release-check")
            .build()
            .map_err(|_| AppError::Internal("could not configure update check".to_string()))?
            .get("https://api.github.com/repos/BerryUIKI/alpha-forge/releases/latest")
            .send()
            .await
            .map_err(|_| {
                AppError::Validation(
                    "Could not reach GitHub Releases. Check your connection and try again."
                        .to_string(),
                )
            })?
            .error_for_status()
            .map_err(|_| {
                AppError::Validation("GitHub Releases did not return a latest release.".to_string())
            })?
            .json::<GitHubRelease>()
            .await
            .map_err(|_| {
                AppError::Validation(
                    "GitHub Releases returned an invalid release response.".to_string(),
                )
            })?;

        Ok(ReleaseCheck {
            update_available: is_newer_version(&release.tag_name, &current_version),
            current_version,
            latest_version: release.tag_name,
            release_url: validate_release_url(&release.html_url)?,
        })
    }
}

async fn export_database_to(pool: &SqlitePool, destination: &Path) -> Result<(), AppError> {
    let escaped_destination = destination.to_string_lossy().replace('\'', "''");
    sqlx::raw_sql(&format!("VACUUM INTO '{escaped_destination}'"))
        .execute(pool)
        .await
        .map_err(|_| {
            AppError::Internal(
                "Could not create the local backup. Choose a writable new filename and try again."
                    .to_string(),
            )
        })?;
    Ok(())
}

fn is_newer_version(latest: &str, current: &str) -> bool {
    let parse = |value: &str| {
        value
            .trim_start_matches('v')
            .split('.')
            .map(str::parse::<u32>)
            .collect::<Result<Vec<_>, _>>()
            .ok()
            .filter(|parts| parts.len() == 3)
            .map(|parts| (parts[0], parts[1], parts[2]))
    };
    matches!((parse(latest), parse(current)), (Some(latest), Some(current)) if latest > current)
}

fn validate_release_url(input: &str) -> Result<String, AppError> {
    let url = Url::parse(input).map_err(|_| {
        AppError::Validation("GitHub Releases returned an invalid release URL.".to_string())
    })?;
    let is_official_release = url.scheme() == "https"
        && url
            .host_str()
            .is_some_and(|host| host.eq_ignore_ascii_case("github.com"))
        && url.port().is_none()
        && url.username().is_empty()
        && url.password().is_none()
        && url.path().starts_with("/BerryUIKI/alpha-forge/releases/");

    if !is_official_release {
        return Err(AppError::Validation(
            "GitHub Releases returned an untrusted release URL.".to_string(),
        ));
    }

    Ok(url.to_string())
}

#[cfg(test)]
mod tests {
    use super::{export_database_to, is_newer_version, validate_release_url, SystemInfo};
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn serializes_system_info_with_camel_case_contract() {
        let info = SystemInfo {
            app_name: "Investment OS".to_string(),
            app_version: "0.1.0".to_string(),
            platform: "windows".to_string(),
            architecture: "x86_64".to_string(),
        };

        assert_eq!(
            serde_json::to_value(info).expect("system info should serialize"),
            serde_json::json!({
                "appName": "Investment OS",
                "appVersion": "0.1.0",
                "platform": "windows",
                "architecture": "x86_64",
            })
        );
    }

    #[test]
    fn compares_release_versions_without_accepting_invalid_tags() {
        assert!(is_newer_version("v0.2.0", "0.1.9"));
        assert!(!is_newer_version("v0.1.0", "0.1.0"));
        assert!(!is_newer_version("latest", "0.1.0"));
    }

    #[test]
    fn accepts_only_official_https_release_pages() {
        assert_eq!(
            validate_release_url("https://github.com/BerryUIKI/alpha-forge/releases/tag/v0.2.0")
                .unwrap(),
            "https://github.com/BerryUIKI/alpha-forge/releases/tag/v0.2.0"
        );
        for url in [
            "http://github.com/BerryUIKI/alpha-forge/releases/tag/v0.2.0",
            "https://github.com.evil.example/BerryUIKI/alpha-forge/releases/tag/v0.2.0",
            "https://github.com/other/project/releases/tag/v0.2.0",
            "https://user:password@github.com/BerryUIKI/alpha-forge/releases/tag/v0.2.0",
        ] {
            assert!(
                validate_release_url(url).is_err(),
                "{url} should be rejected"
            );
        }
    }

    #[tokio::test]
    async fn exports_a_consistent_database_copy() {
        let unique_suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after epoch")
            .as_nanos();
        let source = std::env::temp_dir().join(format!("investment-os-source-{unique_suffix}.db"));
        let destination =
            std::env::temp_dir().join(format!("investment-os-backup-{unique_suffix}.db"));
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(
                SqliteConnectOptions::new()
                    .filename(&source)
                    .create_if_missing(true),
            )
            .await
            .expect("test database should open");
        sqlx::query("CREATE TABLE note (id INTEGER PRIMARY KEY, body TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("test schema should be created");
        sqlx::query("INSERT INTO note (body) VALUES ('preserved')")
            .execute(&pool)
            .await
            .expect("test row should be inserted");

        export_database_to(&pool, &destination)
            .await
            .expect("backup should be exported");
        assert!(
            destination.is_file(),
            "backup should create a database file"
        );

        let backup = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(
                SqliteConnectOptions::new()
                    .filename(&destination)
                    .read_only(true),
            )
            .await
            .expect("backup database should open");
        let body: String = sqlx::query_scalar("SELECT body FROM note WHERE id = 1")
            .fetch_one(&backup)
            .await
            .expect("backup should preserve rows");
        assert_eq!(body, "preserved");

        backup.close().await;
        pool.close().await;
        std::fs::remove_file(destination).expect("temporary backup should be removable");
        std::fs::remove_file(source).expect("temporary source should be removable");
    }
}
