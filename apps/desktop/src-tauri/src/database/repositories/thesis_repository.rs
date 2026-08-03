// Thesis repository — handles persistence for investment theses and evidence.

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::error::AppError;
use domain::thesis::{
    AddEvidenceInput, CreateThesisInput, EvidenceDirection, InvestmentThesis,
    ThesisConfidenceSnapshot, ThesisEvidence, ThesisStatus, UpdateConfidenceInput,
};

pub struct ThesisRepository {
    pool: SqlitePool,
}

impl ThesisRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Create a new investment thesis.
    pub async fn create_thesis(
        &self,
        input: CreateThesisInput,
    ) -> Result<InvestmentThesis, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let confidence = input.confidence.unwrap_or(50).clamp(0, 100);

        sqlx::query(
            r#"
            INSERT INTO investment_theses
                (id, workspace_id, title, thesis, confidence, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&input.workspace_id)
        .bind(&input.title)
        .bind(&input.thesis)
        .bind(confidence)
        .bind("draft")
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create thesis: {}", e)))?;

        self.record_confidence_snapshot(&id, confidence).await?;

        Ok(InvestmentThesis {
            id,
            workspace_id: input.workspace_id,
            title: input.title,
            thesis: input.thesis,
            confidence,
            status: ThesisStatus::Draft,
            validation_date: None,
            outcome: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Get a thesis by ID.
    pub async fn get_thesis(&self, id: &str) -> Result<Option<InvestmentThesis>, AppError> {
        let row = sqlx::query_as::<_, ThesisRow>(
            r#"
            SELECT id, workspace_id, title, thesis, confidence, status,
                   validation_date, outcome, created_at, updated_at
            FROM investment_theses
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to get thesis: {}", e)))?;

        row.map(TryInto::try_into).transpose()
    }

    /// List all theses for a workspace.
    pub async fn list_by_workspace(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<InvestmentThesis>, AppError> {
        let rows = sqlx::query_as::<_, ThesisRow>(
            r#"
            SELECT id, workspace_id, title, thesis, confidence, status,
                   validation_date, outcome, created_at, updated_at
            FROM investment_theses
            WHERE workspace_id = ?
            ORDER BY updated_at DESC
            "#,
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list theses: {}", e)))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    /// Update thesis status.
    pub async fn update_status(&self, id: &str, status: ThesisStatus) -> Result<(), AppError> {
        let status_str = status.to_string();
        sqlx::query("UPDATE investment_theses SET status = ? WHERE id = ?")
            .bind(&status_str)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to update thesis status: {}", e)))?;
        Ok(())
    }

    /// Update thesis confidence.
    pub async fn update_confidence(&self, input: UpdateConfidenceInput) -> Result<(), AppError> {
        let confidence = input.confidence.clamp(0, 100);
        sqlx::query("UPDATE investment_theses SET confidence = ? WHERE id = ?")
            .bind(confidence)
            .bind(&input.thesis_id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                AppError::Internal(format!("Failed to update thesis confidence: {}", e))
            })?;
        self.record_confidence_snapshot(&input.thesis_id, confidence)
            .await?;
        Ok(())
    }

    /// Record validation outcome.
    pub async fn record_outcome(
        &self,
        id: &str,
        outcome: String,
        status: ThesisStatus,
    ) -> Result<(), AppError> {
        let now = Utc::now();
        let status_str = status.to_string();
        sqlx::query(
            "UPDATE investment_theses SET outcome = ?, validation_date = ?, status = ? WHERE id = ?",
        )
        .bind(&outcome)
        .bind(now.to_rfc3339())
        .bind(&status_str)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to record thesis outcome: {}", e)))?;
        Ok(())
    }

    /// Delete a thesis.
    pub async fn delete_thesis(&self, id: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM investment_theses WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to delete thesis: {}", e)))?;
        Ok(())
    }

    /// Add evidence to a thesis.
    pub async fn add_evidence(&self, input: AddEvidenceInput) -> Result<ThesisEvidence, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let direction_str = input.direction.to_string();

        sqlx::query(
            r#"
            INSERT INTO thesis_evidence (id, thesis_id, direction, evidence, source_id, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&input.thesis_id)
        .bind(&direction_str)
        .bind(&input.evidence)
        .bind(&input.source_id)
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to add evidence: {}", e)))?;

        Ok(ThesisEvidence {
            id,
            thesis_id: input.thesis_id,
            direction: input.direction,
            evidence: input.evidence,
            source_id: input.source_id,
            created_at: now,
        })
    }

    /// List all evidence for a thesis.
    pub async fn list_evidence(&self, thesis_id: &str) -> Result<Vec<ThesisEvidence>, AppError> {
        let rows = sqlx::query_as::<_, EvidenceRow>(
            r#"
            SELECT id, thesis_id, direction, evidence, source_id, created_at
            FROM thesis_evidence
            WHERE thesis_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(thesis_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list evidence: {}", e)))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    /// Delete evidence.
    pub async fn delete_evidence(&self, id: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM thesis_evidence WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to delete evidence: {}", e)))?;
        Ok(())
    }

    /// List confidence snapshots, newest first, for thesis review.
    pub async fn list_confidence_history(
        &self,
        thesis_id: &str,
    ) -> Result<Vec<ThesisConfidenceSnapshot>, AppError> {
        let rows = sqlx::query_as::<_, ConfidenceSnapshotRow>(
            "SELECT id, thesis_id, confidence, recorded_at FROM thesis_confidence_history WHERE thesis_id = ? ORDER BY recorded_at DESC",
        )
        .bind(thesis_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list confidence history: {e}")))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    async fn record_confidence_snapshot(
        &self,
        thesis_id: &str,
        confidence: i32,
    ) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO thesis_confidence_history (id, thesis_id, confidence, recorded_at) VALUES (?, ?, ?, ?)",
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(thesis_id)
        .bind(confidence)
        .bind(Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to record confidence history: {e}")))?;

        Ok(())
    }
}

// Database row types

#[derive(Debug, sqlx::FromRow)]
struct ThesisRow {
    id: String,
    workspace_id: String,
    title: String,
    thesis: String,
    confidence: i32,
    status: String,
    validation_date: Option<String>,
    outcome: Option<String>,
    created_at: String,
    updated_at: String,
}

impl TryFrom<ThesisRow> for InvestmentThesis {
    type Error = AppError;

    fn try_from(row: ThesisRow) -> Result<Self, Self::Error> {
        let status = match row.status.as_str() {
            "draft" => ThesisStatus::Draft,
            "active" => ThesisStatus::Active,
            "validating" => ThesisStatus::Validating,
            "validated" => ThesisStatus::Validated,
            "closed" => ThesisStatus::Closed,
            _ => {
                return Err(AppError::Internal(
                    "Invalid thesis status in database".to_string(),
                ))
            }
        };

        Ok(InvestmentThesis {
            id: row.id,
            workspace_id: row.workspace_id,
            title: row.title,
            thesis: row.thesis,
            confidence: row.confidence,
            status,
            validation_date: row
                .validation_date
                .map(|value| parse_timestamp(&value, "thesis validation"))
                .transpose()?,
            outcome: row.outcome,
            created_at: parse_timestamp(&row.created_at, "thesis creation")?,
            updated_at: parse_timestamp(&row.updated_at, "thesis update")?,
        })
    }
}

#[derive(Debug, sqlx::FromRow)]
struct EvidenceRow {
    id: String,
    thesis_id: String,
    direction: String,
    evidence: String,
    source_id: Option<String>,
    created_at: String,
}

#[derive(Debug, sqlx::FromRow)]
struct ConfidenceSnapshotRow {
    id: String,
    thesis_id: String,
    confidence: i32,
    recorded_at: String,
}

impl TryFrom<ConfidenceSnapshotRow> for ThesisConfidenceSnapshot {
    type Error = AppError;

    fn try_from(row: ConfidenceSnapshotRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            thesis_id: row.thesis_id,
            confidence: row.confidence,
            recorded_at: parse_timestamp(&row.recorded_at, "thesis confidence history")?,
        })
    }
}

impl TryFrom<EvidenceRow> for ThesisEvidence {
    type Error = AppError;

    fn try_from(row: EvidenceRow) -> Result<Self, Self::Error> {
        let direction = match row.direction.as_str() {
            "supporting" => EvidenceDirection::Supporting,
            "contradicting" => EvidenceDirection::Contradicting,
            _ => {
                return Err(AppError::Internal(
                    "Invalid evidence direction in database".to_string(),
                ))
            }
        };

        Ok(ThesisEvidence {
            id: row.id,
            thesis_id: row.thesis_id,
            direction,
            evidence: row.evidence,
            source_id: row.source_id,
            created_at: parse_timestamp(&row.created_at, "thesis evidence creation")?,
        })
    }
}

fn parse_timestamp(value: &str, field: &str) -> Result<DateTime<Utc>, AppError> {
    DateTime::parse_from_rfc3339(value)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .map_err(|_| AppError::Internal(format!("Invalid {field} timestamp in database")))
}
