// Thesis management Tauri commands — M5 Investment Knowledge System.

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use domain::thesis::{
    AddEvidenceInput, CreateThesisInput, EvidenceDirection, InvestmentThesis,
    ThesisConfidenceSnapshot, ThesisEvidence, ThesisStatus, UpdateConfidenceInput,
};

/// DTO for InvestmentThesis with camelCase serialization for the IPC boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvestmentThesisDto {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub thesis: String,
    pub confidence: i32,
    pub status: ThesisStatus,
    pub validation_date: Option<String>,
    pub outcome: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<InvestmentThesis> for InvestmentThesisDto {
    fn from(thesis: InvestmentThesis) -> Self {
        Self {
            id: thesis.id,
            workspace_id: thesis.workspace_id,
            title: thesis.title,
            thesis: thesis.thesis,
            confidence: thesis.confidence,
            status: thesis.status,
            validation_date: thesis.validation_date.map(|dt| dt.to_rfc3339()),
            outcome: thesis.outcome,
            created_at: thesis.created_at.to_rfc3339(),
            updated_at: thesis.updated_at.to_rfc3339(),
        }
    }
}

/// DTO for ThesisEvidence with camelCase serialization for the IPC boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThesisEvidenceDto {
    pub id: String,
    pub thesis_id: String,
    pub direction: EvidenceDirection,
    pub evidence: String,
    pub source_id: Option<String>,
    pub created_at: String,
}

impl From<ThesisEvidence> for ThesisEvidenceDto {
    fn from(evidence: ThesisEvidence) -> Self {
        Self {
            id: evidence.id,
            thesis_id: evidence.thesis_id,
            direction: evidence.direction,
            evidence: evidence.evidence,
            source_id: evidence.source_id,
            created_at: evidence.created_at.to_rfc3339(),
        }
    }
}

/// DTO for ThesisConfidenceSnapshot with camelCase serialization for the IPC boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThesisConfidenceSnapshotDto {
    pub id: String,
    pub thesis_id: String,
    pub confidence: i32,
    pub recorded_at: String,
}

impl From<ThesisConfidenceSnapshot> for ThesisConfidenceSnapshotDto {
    fn from(snap: ThesisConfidenceSnapshot) -> Self {
        Self {
            id: snap.id,
            thesis_id: snap.thesis_id,
            confidence: snap.confidence,
            recorded_at: snap.recorded_at.to_rfc3339(),
        }
    }
}

// Thesis CRUD commands

#[tauri::command]
pub async fn create_thesis(
    workspace_id: String,
    title: String,
    thesis: String,
    confidence: Option<i32>,
    state: State<'_, AppState>,
) -> Result<InvestmentThesisDto, AppError> {
    state
        .thesis_service
        .create_thesis(CreateThesisInput {
            workspace_id,
            title,
            thesis,
            confidence,
        })
        .await
        .map(InvestmentThesisDto::from)
}

#[tauri::command]
pub async fn get_thesis(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<InvestmentThesisDto>, AppError> {
    state
        .thesis_service
        .get_thesis(&id)
        .await
        .map(|opt| opt.map(InvestmentThesisDto::from))
}

#[tauri::command]
pub async fn list_theses(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<InvestmentThesisDto>, AppError> {
    state
        .thesis_service
        .list_theses(&workspace_id)
        .await
        .map(|theses| theses.into_iter().map(InvestmentThesisDto::from).collect())
}

#[tauri::command]
pub async fn activate_thesis(
    id: String,
    state: State<'_, AppState>,
) -> Result<InvestmentThesisDto, AppError> {
    state
        .thesis_service
        .activate_thesis(&id)
        .await
        .map(InvestmentThesisDto::from)
}

#[tauri::command]
pub async fn start_thesis_validation(
    id: String,
    state: State<'_, AppState>,
) -> Result<InvestmentThesisDto, AppError> {
    state
        .thesis_service
        .start_validation(&id)
        .await
        .map(InvestmentThesisDto::from)
}

#[tauri::command]
pub async fn complete_thesis_validation(
    id: String,
    outcome: String,
    validated: bool,
    state: State<'_, AppState>,
) -> Result<InvestmentThesisDto, AppError> {
    state
        .thesis_service
        .complete_validation(&id, outcome, validated)
        .await
        .map(InvestmentThesisDto::from)
}

#[tauri::command]
pub async fn update_thesis_confidence(
    thesis_id: String,
    confidence: i32,
    state: State<'_, AppState>,
) -> Result<InvestmentThesisDto, AppError> {
    state
        .thesis_service
        .update_confidence(UpdateConfidenceInput {
            thesis_id,
            confidence,
        })
        .await
        .map(InvestmentThesisDto::from)
}

#[tauri::command]
pub async fn close_thesis(
    id: String,
    state: State<'_, AppState>,
) -> Result<InvestmentThesisDto, AppError> {
    state
        .thesis_service
        .close_thesis(&id)
        .await
        .map(InvestmentThesisDto::from)
}

#[tauri::command]
pub async fn delete_thesis(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    state.thesis_service.delete_thesis(&id).await
}

// Evidence commands

#[tauri::command]
pub async fn add_thesis_evidence(
    thesis_id: String,
    direction: String,
    evidence: String,
    source_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<ThesisEvidenceDto, AppError> {
    let dir = match direction.as_str() {
        "supporting" => EvidenceDirection::Supporting,
        "contradicting" => EvidenceDirection::Contradicting,
        _ => {
            return Err(AppError::Validation(
                "Evidence direction must be 'supporting' or 'contradicting'".to_string(),
            ));
        }
    };

    state
        .thesis_service
        .add_evidence(AddEvidenceInput {
            thesis_id,
            direction: dir,
            evidence,
            source_id,
        })
        .await
        .map(ThesisEvidenceDto::from)
}

#[tauri::command]
pub async fn list_thesis_evidence(
    thesis_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ThesisEvidenceDto>, AppError> {
    state
        .thesis_service
        .list_evidence(&thesis_id)
        .await
        .map(|evidence| evidence.into_iter().map(ThesisEvidenceDto::from).collect())
}

#[tauri::command]
pub async fn delete_thesis_evidence(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.thesis_service.delete_evidence(&id).await
}

#[tauri::command]
pub async fn list_thesis_confidence_history(
    thesis_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ThesisConfidenceSnapshotDto>, AppError> {
    state
        .thesis_service
        .list_confidence_history(&thesis_id)
        .await
        .map(|snaps| {
            snaps
                .into_iter()
                .map(ThesisConfidenceSnapshotDto::from)
                .collect()
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_thesis_dtos_camel_case_serialization() {
        let now = Utc::now();
        let thesis = InvestmentThesis {
            id: "th-1".to_string(),
            workspace_id: "ws-1".to_string(),
            title: "Growth thesis".to_string(),
            thesis: "Revenue accelerating".to_string(),
            confidence: 85,
            status: ThesisStatus::Active,
            validation_date: Some(now),
            outcome: Some("Success".to_string()),
            created_at: now,
            updated_at: now,
        };
        let thesis_dto = InvestmentThesisDto::from(thesis);
        let thesis_json = serde_json::to_string(&thesis_dto).expect("thesis serialization");
        assert!(thesis_json.contains("\"workspaceId\":\"ws-1\""));
        assert!(thesis_json.contains("\"validationDate\":"));
        assert!(thesis_json.contains("\"createdAt\":"));
        assert!(!thesis_json.contains("\"workspace_id\":"));
        assert!(!thesis_json.contains("\"validation_date\":"));

        let evidence = ThesisEvidence {
            id: "ev-1".to_string(),
            thesis_id: "th-1".to_string(),
            direction: EvidenceDirection::Supporting,
            evidence: "Q3 beat".to_string(),
            source_id: Some("src-1".to_string()),
            created_at: now,
        };
        let evidence_dto = ThesisEvidenceDto::from(evidence);
        let evidence_json = serde_json::to_string(&evidence_dto).expect("evidence serialization");
        assert!(evidence_json.contains("\"thesisId\":\"th-1\""));
        assert!(evidence_json.contains("\"sourceId\":\"src-1\""));
        assert!(evidence_json.contains("\"direction\":\"supporting\""));
        assert!(!evidence_json.contains("\"thesis_id\":"));
        assert!(!evidence_json.contains("\"source_id\":"));

        let snapshot = ThesisConfidenceSnapshot {
            id: "snap-1".to_string(),
            thesis_id: "th-1".to_string(),
            confidence: 85,
            recorded_at: now,
        };
        let snap_dto = ThesisConfidenceSnapshotDto::from(snapshot);
        let snap_json = serde_json::to_string(&snap_dto).expect("snapshot serialization");
        assert!(snap_json.contains("\"thesisId\":\"th-1\""));
        assert!(snap_json.contains("\"recordedAt\":"));
        assert!(!snap_json.contains("\"thesis_id\":"));
        assert!(!snap_json.contains("\"recorded_at\":"));
    }
}
