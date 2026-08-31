// Proposal repository — handles proposal persistence in SQLite.

use crate::error::AppError;
use domain::proposal::{Proposal, ProposalStatus, ProposalType};
use sqlx::SqlitePool;

pub struct ProposalRepository {
    pool: SqlitePool,
}

impl ProposalRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_proposal(&self, proposal: &Proposal) -> Result<(), AppError> {
        let payload_str = serde_json::to_string(&proposal.payload)
            .map_err(|e| AppError::Validation(format!("Invalid proposal payload: {}", e)))?;
        let proposal_type_str = serde_json::to_value(proposal.proposal_type)
            .map_err(|e| AppError::Validation(format!("Invalid proposal type: {}", e)))?
            .as_str()
            .unwrap_or("evidence_candidate")
            .to_string();
        let status_str = serde_json::to_value(proposal.status)
            .map_err(|e| AppError::Validation(format!("Invalid proposal status: {}", e)))?
            .as_str()
            .unwrap_or("pending")
            .to_string();

        sqlx::query(
            r#"INSERT INTO proposals (
                id, workspace_id, run_id, proposal_type, title, summary, payload, status, created_at, reviewed_at, resulting_entity_id
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&proposal.id)
        .bind(&proposal.workspace_id)
        .bind(&proposal.run_id)
        .bind(&proposal_type_str)
        .bind(&proposal.title)
        .bind(&proposal.summary)
        .bind(&payload_str)
        .bind(&status_str)
        .bind(&proposal.created_at)
        .bind(&proposal.reviewed_at)
        .bind(&proposal.resulting_entity_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create proposal: {}", e)))?;

        Ok(())
    }

    pub async fn get_proposal(&self, id: &str) -> Result<Option<Proposal>, AppError> {
        let row = sqlx::query_as::<_, ProposalRow>(
            r#"SELECT id, workspace_id, run_id, proposal_type, title, summary, payload, status, created_at, reviewed_at, resulting_entity_id
            FROM proposals WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to get proposal: {}", e)))?;

        row.map(ProposalRow::into_domain).transpose()
    }

    pub async fn list_by_workspace(
        &self,
        workspace_id: &str,
        status: Option<ProposalStatus>,
    ) -> Result<Vec<Proposal>, AppError> {
        let rows = if let Some(st) = status {
            let status_str = serde_json::to_value(st)
                .map_err(|e| AppError::Validation(format!("Invalid status: {}", e)))?
                .as_str()
                .unwrap_or("pending")
                .to_string();

            sqlx::query_as::<_, ProposalRow>(
                r#"SELECT id, workspace_id, run_id, proposal_type, title, summary, payload, status, created_at, reviewed_at, resulting_entity_id
                FROM proposals WHERE workspace_id = ? AND status = ? ORDER BY created_at DESC"#,
            )
            .bind(workspace_id)
            .bind(status_str)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to list proposals: {}", e)))?
        } else {
            sqlx::query_as::<_, ProposalRow>(
                r#"SELECT id, workspace_id, run_id, proposal_type, title, summary, payload, status, created_at, reviewed_at, resulting_entity_id
                FROM proposals WHERE workspace_id = ? ORDER BY created_at DESC"#,
            )
            .bind(workspace_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to list proposals: {}", e)))?
        };

        rows.into_iter().map(ProposalRow::into_domain).collect()
    }

    pub async fn update_status(
        &self,
        id: &str,
        status: ProposalStatus,
        reviewed_at: Option<&str>,
        resulting_entity_id: Option<&str>,
    ) -> Result<(), AppError> {
        let status_str = serde_json::to_value(status)
            .map_err(|e| AppError::Validation(format!("Invalid status: {}", e)))?
            .as_str()
            .unwrap_or("pending")
            .to_string();

        sqlx::query(
            r#"UPDATE proposals SET status = ?, reviewed_at = ?, resulting_entity_id = ? WHERE id = ?"#,
        )
        .bind(status_str)
        .bind(reviewed_at)
        .bind(resulting_entity_id)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update proposal status: {}", e)))?;

        Ok(())
    }

    pub async fn delete_proposal(&self, id: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM proposals WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to delete proposal: {}", e)))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct ProposalRow {
    id: String,
    workspace_id: String,
    run_id: String,
    proposal_type: String,
    title: String,
    summary: String,
    payload: String,
    status: String,
    created_at: String,
    reviewed_at: Option<String>,
    resulting_entity_id: Option<String>,
}

impl ProposalRow {
    fn into_domain(self) -> Result<Proposal, AppError> {
        let payload: serde_json::Value = serde_json::from_str(&self.payload)
            .map_err(|e| AppError::Internal(format!("Corrupt proposal payload JSON: {}", e)))?;
        let proposal_type: ProposalType =
            serde_json::from_value(serde_json::Value::String(self.proposal_type))
                .map_err(|e| AppError::Internal(format!("Invalid stored proposal_type: {}", e)))?;
        let status: ProposalStatus = serde_json::from_value(serde_json::Value::String(self.status))
            .map_err(|e| AppError::Internal(format!("Invalid stored proposal status: {}", e)))?;

        Ok(Proposal {
            id: self.id,
            workspace_id: self.workspace_id,
            run_id: self.run_id,
            proposal_type,
            title: self.title,
            summary: self.summary,
            payload,
            status,
            created_at: self.created_at,
            reviewed_at: self.reviewed_at,
            resulting_entity_id: self.resulting_entity_id,
        })
    }
}
