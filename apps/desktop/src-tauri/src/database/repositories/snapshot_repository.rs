// Financial repositories — holdings snapshots + snapshot positions.
//
// SQLx persistence for `holdings_snapshots` and `snapshot_positions`
// (migration 0019). A snapshot and its positions are written atomically in one
// transaction; the relational `snapshot_positions` rows are authoritative and
// the snapshot's `positions` JSON column is a serialized mirror kept for
// forward compatibility with tools that only read the JSON payload.

use chrono::Utc;
use serde_json::json;
use sqlx::SqlitePool;

use crate::database::repositories::financial_support::{
    parse_date, parse_decimal, parse_json, parse_optional_decimal, parse_timestamp,
};
use crate::error::AppError;
use domain::financial::{
    CreateSnapshotInput, HoldingSnapshot, HoldingSnapshotSource, SnapshotPosition,
};

pub struct SnapshotRepository {
    pool: SqlitePool,
}

impl SnapshotRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateSnapshotInput) -> Result<HoldingSnapshot, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let mut database_transaction = self.pool.begin().await.map_err(|e| {
            AppError::Internal(format!("failed to start snapshot transaction: {e}"))
        })?;

        sqlx::query(
            "INSERT INTO holdings_snapshots
                (id, account_id, snapshot_date, currency, positions, cash_balances, cost_basis,
                 net_contribution, net_contribution_base, cash_total_account_currency,
                 cash_total_base_currency, source, calculated_at)
             VALUES (?, ?, ?, ?, '{}', ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&input.account_id)
        .bind(input.snapshot_date.to_string())
        .bind(&input.currency)
        .bind(serde_json::to_string(&input.cash_balances).map_err(|e| {
            AppError::Internal(format!("failed to encode snapshot cash balances: {e}"))
        })?)
        .bind(input.cost_basis.to_string())
        .bind(input.net_contribution.to_string())
        .bind(input.net_contribution_base.to_string())
        .bind(input.cash_total_account_currency.to_string())
        .bind(input.cash_total_base_currency.to_string())
        .bind(input.source.to_string())
        .bind(now.to_rfc3339())
        .execute(&mut *database_transaction)
        .await
        .map_err(|e| AppError::Internal(format!("failed to create snapshot: {e}")))?;

        let mut positions = Vec::with_capacity(input.positions.len());
        for position in input.positions {
            let row = sqlx::query_as::<_, PositionRow>(
                "INSERT INTO snapshot_positions
                    (snapshot_id, asset_id, quantity, average_cost, total_cost_basis, currency,
                     contract_multiplier, inception_date, is_alternative, cost_basis_base,
                     cost_basis_account, created_at, last_updated)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                 RETURNING id, snapshot_id, asset_id, quantity, average_cost, total_cost_basis,
                           currency, contract_multiplier, inception_date, is_alternative,
                           cost_basis_base, cost_basis_account, created_at, last_updated",
            )
            .bind(&id)
            .bind(&position.asset_id)
            .bind(position.quantity.to_string())
            .bind(position.average_cost.to_string())
            .bind(position.total_cost_basis.to_string())
            .bind(&position.currency)
            .bind(position.contract_multiplier.to_string())
            .bind(position.inception_date.to_string())
            .bind(position.is_alternative)
            .bind(position.cost_basis_base.map(|d| d.to_string()))
            .bind(position.cost_basis_account.map(|d| d.to_string()))
            .bind(now.to_rfc3339())
            .bind(now.to_rfc3339())
            .fetch_one(&mut *database_transaction)
            .await
            .map_err(|e| AppError::Internal(format!("failed to create snapshot position: {e}")))?;

            positions.push(row.try_into()?);
        }

        // Mirror the inserted positions into the snapshot's JSON column.
        let mirror: Vec<serde_json::Value> = positions
            .iter()
            .map(|position: &SnapshotPosition| {
                json!({
                    "id": position.id,
                    "asset_id": position.asset_id,
                    "quantity": position.quantity.to_string(),
                    "average_cost": position.average_cost.to_string(),
                    "total_cost_basis": position.total_cost_basis.to_string(),
                    "currency": position.currency,
                    "contract_multiplier": position.contract_multiplier.to_string(),
                    "inception_date": position.inception_date.to_string(),
                    "is_alternative": position.is_alternative,
                    "cost_basis_base": position.cost_basis_base.map(|d| d.to_string()),
                    "cost_basis_account": position.cost_basis_account.map(|d| d.to_string()),
                })
            })
            .collect();
        sqlx::query("UPDATE holdings_snapshots SET positions = ? WHERE id = ?")
            .bind(serde_json::to_string(&mirror).map_err(|e| {
                AppError::Internal(format!("failed to encode snapshot positions mirror: {e}"))
            })?)
            .bind(&id)
            .execute(&mut *database_transaction)
            .await
            .map_err(|e| AppError::Internal(format!("failed to update snapshot mirror: {e}")))?;

        database_transaction.commit().await.map_err(|e| {
            AppError::Internal(format!("failed to commit snapshot transaction: {e}"))
        })?;

        let snapshot = sqlx::query_as::<_, SnapshotRow>(
            "SELECT id, account_id, snapshot_date, currency, cash_balances, cost_basis,
                    net_contribution, net_contribution_base, cash_total_account_currency,
                    cash_total_base_currency, source, calculated_at
             FROM holdings_snapshots WHERE id = ?",
        )
        .bind(&id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to read created snapshot: {e}")))?;

        SnapshotRow::into_domain(snapshot, positions)
    }

    pub async fn get(&self, id: &str) -> Result<Option<HoldingSnapshot>, AppError> {
        let snapshot = sqlx::query_as::<_, SnapshotRow>(
            "SELECT id, account_id, snapshot_date, currency, cash_balances, cost_basis,
                    net_contribution, net_contribution_base, cash_total_account_currency,
                    cash_total_base_currency, source, calculated_at
             FROM holdings_snapshots WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to get snapshot: {e}")))?;

        let Some(snapshot) = snapshot else {
            return Ok(None);
        };

        let positions = self.load_positions(id).await?;
        snapshot.into_domain(positions).map(Some)
    }

    pub async fn list_by_account(
        &self,
        account_id: &str,
    ) -> Result<Vec<HoldingSnapshot>, AppError> {
        let snapshots = sqlx::query_as::<_, SnapshotRow>(
            "SELECT id, account_id, snapshot_date, currency, cash_balances, cost_basis,
                    net_contribution, net_contribution_base, cash_total_account_currency,
                    cash_total_base_currency, source, calculated_at
             FROM holdings_snapshots WHERE account_id = ? ORDER BY snapshot_date, calculated_at",
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to list snapshots: {e}")))?;

        let mut result = Vec::with_capacity(snapshots.len());
        for snapshot in snapshots {
            let positions = self.load_positions(&snapshot.id).await?;
            result.push(snapshot.into_domain(positions)?);
        }
        Ok(result)
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM holdings_snapshots WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("failed to delete snapshot: {e}")))?;
        Ok(())
    }

    async fn load_positions(&self, snapshot_id: &str) -> Result<Vec<SnapshotPosition>, AppError> {
        let rows = sqlx::query_as::<_, PositionRow>(
            "SELECT id, snapshot_id, asset_id, quantity, average_cost, total_cost_basis, currency,
                    contract_multiplier, inception_date, is_alternative, cost_basis_base,
                    cost_basis_account, created_at, last_updated
             FROM snapshot_positions WHERE snapshot_id = ? ORDER BY id",
        )
        .bind(snapshot_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to load snapshot positions: {e}")))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }
}

#[derive(sqlx::FromRow)]
struct SnapshotRow {
    id: String,
    account_id: String,
    snapshot_date: String,
    currency: String,
    cash_balances: Option<String>,
    cost_basis: String,
    net_contribution: String,
    net_contribution_base: String,
    cash_total_account_currency: String,
    cash_total_base_currency: String,
    source: String,
    calculated_at: String,
}

impl SnapshotRow {
    fn into_domain(self, positions: Vec<SnapshotPosition>) -> Result<HoldingSnapshot, AppError> {
        let source = HoldingSnapshotSource::parse(&self.source).ok_or_else(|| {
            AppError::Internal(format!(
                "invalid snapshot source in database: {}",
                self.source
            ))
        })?;

        Ok(HoldingSnapshot {
            id: self.id,
            account_id: self.account_id,
            snapshot_date: parse_date(&self.snapshot_date, "snapshot date")?,
            currency: self.currency,
            positions,
            cash_balances: parse_json(self.cash_balances, "snapshot cash balances")?
                .unwrap_or(serde_json::Value::Object(Default::default())),
            cost_basis: parse_decimal(&self.cost_basis, "snapshot cost basis")?,
            net_contribution: parse_decimal(&self.net_contribution, "snapshot net contribution")?,
            net_contribution_base: parse_decimal(
                &self.net_contribution_base,
                "snapshot net contribution base",
            )?,
            cash_total_account_currency: parse_decimal(
                &self.cash_total_account_currency,
                "snapshot cash total account currency",
            )?,
            cash_total_base_currency: parse_decimal(
                &self.cash_total_base_currency,
                "snapshot cash total base currency",
            )?,
            source,
            calculated_at: parse_timestamp(&self.calculated_at, "snapshot calculation")?,
        })
    }
}

#[derive(sqlx::FromRow)]
struct PositionRow {
    id: i64,
    snapshot_id: String,
    asset_id: String,
    quantity: String,
    average_cost: String,
    total_cost_basis: String,
    currency: String,
    contract_multiplier: String,
    inception_date: String,
    is_alternative: bool,
    cost_basis_base: Option<String>,
    cost_basis_account: Option<String>,
    created_at: String,
    last_updated: String,
}

impl TryFrom<PositionRow> for SnapshotPosition {
    type Error = AppError;

    fn try_from(row: PositionRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            snapshot_id: row.snapshot_id,
            asset_id: row.asset_id,
            quantity: parse_decimal(&row.quantity, "snapshot position quantity")?,
            average_cost: parse_decimal(&row.average_cost, "snapshot position average cost")?,
            total_cost_basis: parse_decimal(
                &row.total_cost_basis,
                "snapshot position total cost basis",
            )?,
            currency: row.currency,
            contract_multiplier: parse_decimal(
                &row.contract_multiplier,
                "snapshot position contract multiplier",
            )?,
            inception_date: parse_date(&row.inception_date, "snapshot position inception date")?,
            is_alternative: row.is_alternative,
            cost_basis_base: parse_optional_decimal(
                row.cost_basis_base,
                "snapshot position cost basis base",
            )?,
            cost_basis_account: parse_optional_decimal(
                row.cost_basis_account,
                "snapshot position cost basis account",
            )?,
            created_at: parse_timestamp(&row.created_at, "snapshot position creation")?,
            last_updated: parse_timestamp(&row.last_updated, "snapshot position update")?,
        })
    }
}
