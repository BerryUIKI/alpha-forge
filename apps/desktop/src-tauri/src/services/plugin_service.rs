use crate::database::repositories::plugin_repository::{InstalledPlugin, PluginRepository};
use crate::error::AppError;
use crate::plugins::loader::{load_bundled_manifests, validate_bundled_payload};
use domain::artifact::ArtifactType;

#[derive(Debug)]
pub struct PluginArtifactRequest {
    pub artifact_type: ArtifactType,
    pub payload: serde_json::Value,
}

pub struct PluginService {
    repository: PluginRepository,
}

impl PluginService {
    pub fn new(repository: PluginRepository) -> Self {
        Self { repository }
    }

    pub async fn sync_bundled_plugins(&self) -> Result<(), AppError> {
        for manifest in load_bundled_manifests()? {
            self.repository.upsert_internal(&manifest).await?;
        }
        Ok(())
    }

    pub async fn list_plugins(&self) -> Result<Vec<InstalledPlugin>, AppError> {
        self.repository.list().await
    }

    pub async fn set_enabled(&self, plugin_id: &str, enabled: bool) -> Result<(), AppError> {
        if plugin_id.trim().is_empty() {
            return Err(AppError::Validation("Plugin id is required".to_string()));
        }
        self.repository.set_enabled(plugin_id, enabled).await
    }

    pub async fn prepare_artifact(
        &self,
        plugin_id: &str,
        payload: serde_json::Value,
    ) -> Result<PluginArtifactRequest, AppError> {
        let plugin = self
            .repository
            .list()
            .await?
            .into_iter()
            .find(|plugin| plugin.manifest.id == plugin_id)
            .ok_or_else(|| AppError::NotFound("Plugin not found".to_string()))?;
        if !plugin.enabled {
            return Err(AppError::PermissionDenied("Plugin is disabled".to_string()));
        }
        validate_bundled_payload(plugin_id, &payload)?;
        let artifact_type = match plugin_id {
            "company-comparison" => ArtifactType::ComparisonTable,
            "valuation-model" => ArtifactType::ValuationModel,
            "industry-map" => ArtifactType::IndustryMap,
            "portfolio-risk" => ArtifactType::RiskDashboard,
            "research-timeline" => ArtifactType::Timeline,
            "earnings-analyzer" => ArtifactType::EarningsAnalysis,
            "macro-dashboard" => ArtifactType::MacroDashboard,
            _ => {
                return Err(AppError::NotFound(
                    "Internal plugin is not supported".to_string(),
                ))
            }
        };
        Ok(PluginArtifactRequest {
            artifact_type,
            payload,
        })
    }
}
