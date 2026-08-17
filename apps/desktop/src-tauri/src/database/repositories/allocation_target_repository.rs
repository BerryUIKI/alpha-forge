// Financial repositories — allocation targets + weights + constraints.
//
// SQLx persistence for `allocation_targets`, `allocation_target_weights`, and
// `allocation_target_constraints` (migration 0020). Targets express a desired
// portfolio composition against one taxonomy; weights pin categories to basis
// points; constraints gate buy/sell/trade actions during rebalancing. The
// database enforces that every weight's taxonomy matches its owning target
// (see the triggers in migration 0020), so the repository surfaces those
// violations as typed validation errors instead of raw SQL failures.

use chrono::Utc;
use sqlx::SqlitePool;

use crate::database::repositories::financial_support::{
    parse_decimal, parse_json, parse_timestamp,
};
use crate::error::AppError;
use domain::financial::{
    AllocationTarget, AllocationTargetConstraint, AllocationTargetConstraintInput,
    AllocationTargetWeight, AllocationTargetWeightInput, ConstraintAction, ConstraintEffect,
    ConstraintSubjectType, CreateAllocationTargetInput, ScopeType,
};

pub struct AllocationTargetRepository {
    pool: SqlitePool,
}

impl AllocationTargetRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        input: CreateAllocationTargetInput,
    ) -> Result<AllocationTarget, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO allocation_targets
                (id, name, scope_type, scope_id, taxonomy_id, trigger_type, drift_band_bps,
                 rebalance_goal, min_trade_amount, whole_shares_only, allow_sells,
                 max_turnover_bps, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&input.name)
        .bind(input.scope_type.to_string())
        .bind(&input.scope_id)
        .bind(&input.taxonomy_id)
        .bind(&input.trigger_type)
        .bind(input.drift_band_bps)
        .bind(&input.rebalance_goal)
        .bind(input.min_trade_amount.to_string())
        .bind(input.whole_shares_only)
        .bind(input.allow_sells)
        .bind(input.max_turnover_bps)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to create allocation target: {e}")))?;

        Ok(AllocationTarget {
            id,
            name: input.name,
            scope_type: input.scope_type,
            scope_id: input.scope_id,
            taxonomy_id: input.taxonomy_id,
            trigger_type: input.trigger_type,
            drift_band_bps: input.drift_band_bps,
            rebalance_goal: input.rebalance_goal,
            min_trade_amount: input.min_trade_amount,
            whole_shares_only: input.whole_shares_only,
            allow_sells: input.allow_sells,
            max_turnover_bps: input.max_turnover_bps,
            created_at: now,
            updated_at: now,
            archived_at: None,
        })
    }

    pub async fn get(&self, id: &str) -> Result<Option<AllocationTarget>, AppError> {
        let row = sqlx::query_as::<_, TargetRow>(
            "SELECT id, name, scope_type, scope_id, taxonomy_id, trigger_type, drift_band_bps,
                    rebalance_goal, min_trade_amount, whole_shares_only, allow_sells,
                    max_turnover_bps, created_at, updated_at, archived_at
             FROM allocation_targets WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to get allocation target: {e}")))?;

        row.map(TryInto::try_into).transpose()
    }

    pub async fn list(&self, include_archived: bool) -> Result<Vec<AllocationTarget>, AppError> {
        let rows = if include_archived {
            sqlx::query_as::<_, TargetRow>(
                "SELECT id, name, scope_type, scope_id, taxonomy_id, trigger_type, drift_band_bps,
                        rebalance_goal, min_trade_amount, whole_shares_only, allow_sells,
                        max_turnover_bps, created_at, updated_at, archived_at
                 FROM allocation_targets ORDER BY created_at",
            )
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, TargetRow>(
                "SELECT id, name, scope_type, scope_id, taxonomy_id, trigger_type, drift_band_bps,
                        rebalance_goal, min_trade_amount, whole_shares_only, allow_sells,
                        max_turnover_bps, created_at, updated_at, archived_at
                 FROM allocation_targets WHERE archived_at IS NULL ORDER BY created_at",
            )
            .fetch_all(&self.pool)
            .await
        }
        .map_err(|e| AppError::Internal(format!("failed to list allocation targets: {e}")))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    pub async fn archive(&self, id: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE allocation_targets SET archived_at = ? WHERE id = ?")
            .bind(Utc::now().to_rfc3339())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("failed to archive allocation target: {e}")))?;
        Ok(())
    }

    pub async fn add_weight(
        &self,
        input: AllocationTargetWeightInput,
    ) -> Result<AllocationTargetWeight, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO allocation_target_weights
                (id, target_id, taxonomy_id, category_id, target_bps, is_locked, is_required,
                 created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&input.target_id)
        .bind(&input.taxonomy_id)
        .bind(&input.category_id)
        .bind(input.target_bps)
        .bind(input.is_locked)
        .bind(input.is_required)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string()
                .contains("must match allocation_targets.taxonomy_id")
            {
                AppError::Validation(format!(
                    "weight taxonomy {} does not match target {} taxonomy",
                    input.taxonomy_id, input.target_id
                ))
            } else {
                AppError::Internal(format!("failed to add allocation weight: {e}"))
            }
        })?;

        Ok(AllocationTargetWeight {
            id,
            target_id: input.target_id,
            taxonomy_id: input.taxonomy_id,
            category_id: input.category_id,
            target_bps: input.target_bps,
            is_locked: input.is_locked,
            is_required: input.is_required,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn list_weights(
        &self,
        target_id: &str,
    ) -> Result<Vec<AllocationTargetWeight>, AppError> {
        let rows = sqlx::query_as::<_, WeightRow>(
            "SELECT id, target_id, taxonomy_id, category_id, target_bps, is_locked, is_required,
                    created_at, updated_at
             FROM allocation_target_weights WHERE target_id = ? ORDER BY created_at",
        )
        .bind(target_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to list allocation weights: {e}")))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    pub async fn add_constraint(
        &self,
        input: AllocationTargetConstraintInput,
    ) -> Result<AllocationTargetConstraint, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO allocation_target_constraints
                (id, target_id, subject_type, subject_id, action, effect, reason, metadata_json,
                 created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&input.target_id)
        .bind(input.subject_type.to_string())
        .bind(&input.subject_id)
        .bind(input.action.to_string())
        .bind(input.effect.to_string())
        .bind(&input.reason)
        .bind(serde_json::to_string(&input.metadata_json).ok())
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to add allocation constraint: {e}")))?;

        Ok(AllocationTargetConstraint {
            id,
            target_id: input.target_id,
            subject_type: input.subject_type,
            subject_id: input.subject_id,
            action: input.action,
            effect: input.effect,
            reason: input.reason,
            metadata_json: input.metadata_json,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn list_constraints(
        &self,
        target_id: &str,
    ) -> Result<Vec<AllocationTargetConstraint>, AppError> {
        let rows = sqlx::query_as::<_, ConstraintRow>(
            "SELECT id, target_id, subject_type, subject_id, action, effect, reason, metadata_json,
                    created_at, updated_at
             FROM allocation_target_constraints WHERE target_id = ? ORDER BY created_at",
        )
        .bind(target_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to list allocation constraints: {e}")))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }
}

#[derive(sqlx::FromRow)]
struct TargetRow {
    id: String,
    name: String,
    scope_type: String,
    scope_id: Option<String>,
    taxonomy_id: String,
    trigger_type: String,
    drift_band_bps: i32,
    rebalance_goal: String,
    min_trade_amount: String,
    whole_shares_only: bool,
    allow_sells: bool,
    max_turnover_bps: Option<i32>,
    created_at: String,
    updated_at: String,
    archived_at: Option<String>,
}

impl TryFrom<TargetRow> for AllocationTarget {
    type Error = AppError;

    fn try_from(row: TargetRow) -> Result<Self, Self::Error> {
        let scope_type = ScopeType::parse(&row.scope_type).ok_or_else(|| {
            AppError::Internal(format!(
                "invalid scope_type in database: {}",
                row.scope_type
            ))
        })?;

        Ok(Self {
            id: row.id,
            name: row.name,
            scope_type,
            scope_id: row.scope_id,
            taxonomy_id: row.taxonomy_id,
            trigger_type: row.trigger_type,
            drift_band_bps: row.drift_band_bps,
            rebalance_goal: row.rebalance_goal,
            min_trade_amount: parse_decimal(&row.min_trade_amount, "target min trade amount")?,
            whole_shares_only: row.whole_shares_only,
            allow_sells: row.allow_sells,
            max_turnover_bps: row.max_turnover_bps,
            created_at: parse_timestamp(&row.created_at, "target creation")?,
            updated_at: parse_timestamp(&row.updated_at, "target update")?,
            archived_at: row.archived_at,
        })
    }
}

#[derive(sqlx::FromRow)]
struct WeightRow {
    id: String,
    target_id: String,
    taxonomy_id: String,
    category_id: String,
    target_bps: i32,
    is_locked: bool,
    is_required: bool,
    created_at: String,
    updated_at: String,
}

impl TryFrom<WeightRow> for AllocationTargetWeight {
    type Error = AppError;

    fn try_from(row: WeightRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            target_id: row.target_id,
            taxonomy_id: row.taxonomy_id,
            category_id: row.category_id,
            target_bps: row.target_bps,
            is_locked: row.is_locked,
            is_required: row.is_required,
            created_at: parse_timestamp(&row.created_at, "weight creation")?,
            updated_at: parse_timestamp(&row.updated_at, "weight update")?,
        })
    }
}

#[derive(sqlx::FromRow)]
struct ConstraintRow {
    id: String,
    target_id: String,
    subject_type: String,
    subject_id: String,
    action: String,
    effect: String,
    reason: Option<String>,
    metadata_json: Option<String>,
    created_at: String,
    updated_at: String,
}

impl TryFrom<ConstraintRow> for AllocationTargetConstraint {
    type Error = AppError;

    fn try_from(row: ConstraintRow) -> Result<Self, Self::Error> {
        let subject_type = ConstraintSubjectType::parse(&row.subject_type).ok_or_else(|| {
            AppError::Internal(format!(
                "invalid subject_type in database: {}",
                row.subject_type
            ))
        })?;
        let action = ConstraintAction::parse(&row.action).ok_or_else(|| {
            AppError::Internal(format!("invalid action in database: {}", row.action))
        })?;
        let effect = ConstraintEffect::parse(&row.effect).ok_or_else(|| {
            AppError::Internal(format!("invalid effect in database: {}", row.effect))
        })?;

        Ok(Self {
            id: row.id,
            target_id: row.target_id,
            subject_type,
            subject_id: row.subject_id,
            action,
            effect,
            reason: row.reason,
            metadata_json: parse_json(row.metadata_json, "constraint metadata")?,
            created_at: parse_timestamp(&row.created_at, "constraint creation")?,
            updated_at: parse_timestamp(&row.updated_at, "constraint update")?,
        })
    }
}
