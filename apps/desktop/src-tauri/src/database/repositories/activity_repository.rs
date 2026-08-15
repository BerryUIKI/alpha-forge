// Financial repositories — import runs + activities.
//
// SQLx persistence for `import_runs` and `activities` (migration 0017).
// `activities` is the canonical transaction ledger; `idempotency_key` makes
// re-imports of the same source record a no-op.

use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use sqlx::Row;
use sqlx::SqlitePool;

use crate::database::repositories::financial_support::{
    parse_date, parse_json, parse_optional_date, parse_optional_decimal, parse_timestamp,
};
use crate::error::AppError;
use crate::services::income_service::IncomeActivityRow;
use domain::financial::{
    Activity, ActivityStatus, ActivityType, CreateActivityInput, CreateImportRunInput, ImportRun,
};

pub struct ImportRunRepository {
    pool: SqlitePool,
}

impl ImportRunRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateImportRunInput) -> Result<ImportRun, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO import_runs
                (id, account_id, source_system, run_type, mode, status, started_at, review_mode,
                 created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&input.account_id)
        .bind(&input.source_system)
        .bind(&input.run_type)
        .bind(&input.mode)
        .bind(&input.status)
        .bind(now.to_rfc3339())
        .bind(&input.review_mode)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to create import run: {e}")))?;

        Ok(ImportRun {
            id,
            account_id: input.account_id,
            source_system: input.source_system,
            run_type: input.run_type,
            mode: input.mode,
            status: input.status,
            started_at: now.to_rfc3339(),
            finished_at: None,
            review_mode: input.review_mode,
            applied_at: None,
            checkpoint_in: None,
            checkpoint_out: None,
            summary: None,
            warnings: None,
            error: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn list_by_account(&self, account_id: &str) -> Result<Vec<ImportRun>, AppError> {
        let rows = sqlx::query_as::<_, ImportRunRow>(
            "SELECT id, account_id, source_system, run_type, mode, status, started_at,
                    finished_at, review_mode, applied_at, checkpoint_in, checkpoint_out,
                    summary, warnings, error, created_at, updated_at
             FROM import_runs WHERE account_id = ? ORDER BY started_at DESC",
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to list import runs: {e}")))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }
}

pub struct ActivityRepository {
    pool: SqlitePool,
}

impl ActivityRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateActivityInput) -> Result<Activity, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO activities
                (id, account_id, asset_id, activity_type, activity_type_override, source_type,
                 subtype, status, activity_date, settlement_date, quantity, unit_price, amount,
                 fee, tax, currency, fx_rate, notes, metadata, source_system, source_record_id,
                 source_group_id, idempotency_key, import_run_id, is_user_modified, needs_review,
                 created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 0, ?, ?)",
        )
        .bind(&id)
        .bind(&input.account_id)
        .bind(&input.asset_id)
        .bind(input.activity_type.to_string())
        .bind(&input.activity_type_override)
        .bind(&input.source_type)
        .bind(&input.subtype)
        .bind(input.status.to_string())
        .bind(input.activity_date.to_string())
        .bind(input.settlement_date.map(|d| d.to_string()))
        .bind(input.quantity.map(|d| d.to_string()))
        .bind(input.unit_price.map(|d| d.to_string()))
        .bind(input.amount.map(|d| d.to_string()))
        .bind(input.fee.map(|d| d.to_string()))
        .bind(input.tax.map(|d| d.to_string()))
        .bind(&input.currency)
        .bind(input.fx_rate.map(|d| d.to_string()))
        .bind(&input.notes)
        .bind(json_or_null(&input.metadata))
        .bind(&input.source_system)
        .bind(&input.source_record_id)
        .bind(&input.source_group_id)
        .bind(&input.idempotency_key)
        .bind(&input.import_run_id)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to create activity: {e}")))?;

        let row = sqlx::query_as::<_, ActivityRow>(
            "SELECT id, account_id, asset_id, activity_type, activity_type_override, source_type,
                    subtype, status, activity_date, settlement_date, quantity, unit_price, amount,
                    fee, tax, currency, fx_rate, notes, metadata, source_system, source_record_id,
                    source_group_id, idempotency_key, import_run_id, is_user_modified, needs_review,
                    created_at, updated_at
             FROM activities WHERE id = ?",
        )
        .bind(&id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to read created activity: {e}")))?;

        row.try_into()
    }

    pub async fn get(&self, id: &str) -> Result<Option<Activity>, AppError> {
        let row = sqlx::query_as::<_, ActivityRow>(
            "SELECT id, account_id, asset_id, activity_type, activity_type_override, source_type,
                    subtype, status, activity_date, settlement_date, quantity, unit_price, amount,
                    fee, tax, currency, fx_rate, notes, metadata, source_system, source_record_id,
                    source_group_id, idempotency_key, import_run_id, is_user_modified, needs_review,
                    created_at, updated_at
             FROM activities WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to get activity: {e}")))?;

        row.map(TryInto::try_into).transpose()
    }

    pub async fn list_by_account(&self, account_id: &str) -> Result<Vec<Activity>, AppError> {
        let rows = sqlx::query_as::<_, ActivityRow>(
            "SELECT id, account_id, asset_id, activity_type, activity_type_override, source_type,
                    subtype, status, activity_date, settlement_date, quantity, unit_price, amount,
                    fee, tax, currency, fx_rate, notes, metadata, source_system, source_record_id,
                    source_group_id, idempotency_key, import_run_id, is_user_modified, needs_review,
                    created_at, updated_at
             FROM activities WHERE account_id = ? ORDER BY activity_date, created_at",
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to list activities: {e}")))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    pub async fn list_by_asset(&self, asset_id: &str) -> Result<Vec<Activity>, AppError> {
        let rows = sqlx::query_as::<_, ActivityRow>(
            "SELECT id, account_id, asset_id, activity_type, activity_type_override, source_type,
                    subtype, status, activity_date, settlement_date, quantity, unit_price, amount,
                    fee, tax, currency, fx_rate, notes, metadata, source_system, source_record_id,
                    source_group_id, idempotency_key, import_run_id, is_user_modified, needs_review,
                    created_at, updated_at
             FROM activities WHERE asset_id = ? ORDER BY activity_date, created_at",
        )
        .bind(asset_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to list asset activities: {e}")))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    /// Get income activities grouped by month for the given accounts.
    pub async fn get_income_activities(
        &self,
        account_ids: Option<&[String]>,
    ) -> Result<Vec<IncomeActivityRow>, AppError> {
        let mut sql = String::from(
            "SELECT strftime('%Y-%m', a.activity_date) as month_key, a.activity_type as income_type,
             COALESCE(a.asset_id, 'CASH') as asset_id,
             COALESCE(ast.kind, 'CASH') as asset_kind,
             COALESCE(ast.display_code, 'CASH') as symbol,
             COALESCE(ast.name, 'Cash') as symbol_name,
             a.currency, COALESCE(a.amount, '0') as amount,
             a.account_id, acc.name as account_name
             FROM activities a
             LEFT JOIN assets ast ON a.asset_id = ast.id
             INNER JOIN accounts acc ON a.account_id = acc.id
             WHERE a.activity_type IN ('DIVIDEND', 'INTEREST')",
        );

        if let Some(ids) = account_ids {
            if !ids.is_empty() {
                let placeholders: Vec<String> = ids
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format!("?{}", i + 1))
                    .collect();
                sql.push_str(&format!(
                    " AND a.account_id IN ({})",
                    placeholders.join(",")
                ));
                let mut query = sqlx::query(&sql);
                for id in ids {
                    query = query.bind(id);
                }
                let rows = query.fetch_all(&self.pool).await.map_err(|e| {
                    AppError::Internal(format!("income activity query failed: {e}"))
                })?;
                return parse_income_rows(rows);
            }
        }

        let rows = sqlx::query(&sql)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("income activity query failed: {e}")))?;
        parse_income_rows(rows)
    }

    /// Get the date of the first activity for the given accounts.
    pub async fn get_first_activity_date(
        &self,
        account_ids: Option<&[String]>,
    ) -> Result<Option<NaiveDate>, AppError> {
        let ids = match account_ids {
            Some(ids) if !ids.is_empty() => ids,
            _ => return Ok(None),
        };
        let placeholders: Vec<String> = ids
            .iter()
            .enumerate()
            .map(|(i, _)| format!("?{}", i + 1))
            .collect();
        let sql = format!(
            "SELECT MIN(a.activity_date) as first_date FROM activities a WHERE a.account_id IN ({})",
            placeholders.join(",")
        );
        let mut query = sqlx::query(&sql);
        for id in ids {
            query = query.bind(id);
        }
        let row = query
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("failed to read first activity date: {e}")))?;
        let date_str: Option<String> = row.try_get("first_date").map_err(|e| {
            AppError::Internal(format!("failed to read first activity date column: {e}"))
        })?;
        date_str
            .map(|d| {
                NaiveDate::parse_from_str(&d, "%Y-%m-%d")
                    .map_err(|e| AppError::Internal(format!("invalid date: {e}")))
            })
            .transpose()
    }

    /// Get the overall first activity date.
    pub async fn get_first_activity_date_overall(&self) -> Result<Option<NaiveDate>, AppError> {
        let row = sqlx::query("SELECT MIN(activity_date) as first_date FROM activities")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("failed to read first activity date: {e}")))?;
        let date_str: Option<String> = row.try_get("first_date").map_err(|e| {
            AppError::Internal(format!("failed to read first activity date column: {e}"))
        })?;
        date_str
            .map(|d| {
                NaiveDate::parse_from_str(&d, "%Y-%m-%d")
                    .map_err(|e| AppError::Internal(format!("invalid date: {e}")))
            })
            .transpose()
    }
}

fn parse_income_rows(
    rows: Vec<sqlx::sqlite::SqliteRow>,
) -> Result<Vec<IncomeActivityRow>, AppError> {
    fn read<T: for<'r> sqlx::Decode<'r, sqlx::Sqlite> + sqlx::Type<sqlx::Sqlite>>(
        row: &sqlx::sqlite::SqliteRow,
        column: &str,
    ) -> Result<T, AppError> {
        row.try_get(column)
            .map_err(|e| AppError::Internal(format!("failed to read column {column}: {e}")))
    }

    rows.into_iter()
        .map(|row| {
            let amount_text: String = read(&row, "amount")?;
            Ok(IncomeActivityRow {
                month_key: read(&row, "month_key")?,
                income_type: read(&row, "income_type")?,
                asset_id: read(&row, "asset_id")?,
                asset_kind: read(&row, "asset_kind")?,
                symbol: read(&row, "symbol")?,
                symbol_name: read(&row, "symbol_name")?,
                currency: read(&row, "currency")?,
                amount: amount_text
                    .parse::<Decimal>()
                    .map_err(|e| AppError::Internal(format!("invalid amount: {e}")))?,
                account_id: read(&row, "account_id")?,
                account_name: read(&row, "account_name")?,
            })
        })
        .collect()
}

#[derive(sqlx::FromRow)]
struct ImportRunRow {
    id: String,
    account_id: String,
    source_system: String,
    run_type: String,
    mode: String,
    status: String,
    started_at: String,
    finished_at: Option<String>,
    review_mode: String,
    applied_at: Option<String>,
    checkpoint_in: Option<String>,
    checkpoint_out: Option<String>,
    summary: Option<String>,
    warnings: Option<String>,
    error: Option<String>,
    created_at: String,
    updated_at: String,
}

impl TryFrom<ImportRunRow> for ImportRun {
    type Error = AppError;

    fn try_from(row: ImportRunRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            account_id: row.account_id,
            source_system: row.source_system,
            run_type: row.run_type,
            mode: row.mode,
            status: row.status,
            started_at: row.started_at,
            finished_at: row.finished_at,
            review_mode: row.review_mode,
            applied_at: row.applied_at,
            checkpoint_in: row.checkpoint_in,
            checkpoint_out: row.checkpoint_out,
            summary: row.summary,
            warnings: row.warnings,
            error: row.error,
            created_at: parse_timestamp(&row.created_at, "import run creation")?,
            updated_at: parse_timestamp(&row.updated_at, "import run update")?,
        })
    }
}

#[derive(sqlx::FromRow)]
struct ActivityRow {
    id: String,
    account_id: String,
    asset_id: Option<String>,
    activity_type: String,
    activity_type_override: Option<String>,
    source_type: Option<String>,
    subtype: Option<String>,
    status: String,
    activity_date: String,
    settlement_date: Option<String>,
    quantity: Option<String>,
    unit_price: Option<String>,
    amount: Option<String>,
    fee: Option<String>,
    tax: Option<String>,
    currency: String,
    fx_rate: Option<String>,
    notes: Option<String>,
    metadata: Option<String>,
    source_system: Option<String>,
    source_record_id: Option<String>,
    source_group_id: Option<String>,
    idempotency_key: Option<String>,
    import_run_id: Option<String>,
    is_user_modified: bool,
    needs_review: bool,
    created_at: String,
    updated_at: String,
}

impl TryFrom<ActivityRow> for Activity {
    type Error = AppError;

    fn try_from(row: ActivityRow) -> Result<Self, Self::Error> {
        let activity_type = ActivityType::parse(&row.activity_type).ok_or_else(|| {
            AppError::Internal(format!(
                "invalid activity_type in database: {}",
                row.activity_type
            ))
        })?;
        let status = ActivityStatus::parse(&row.status).ok_or_else(|| {
            AppError::Internal(format!(
                "invalid activity status in database: {}",
                row.status
            ))
        })?;

        Ok(Self {
            id: row.id,
            account_id: row.account_id,
            asset_id: row.asset_id,
            activity_type,
            activity_type_override: row.activity_type_override,
            source_type: row.source_type,
            subtype: row.subtype,
            status,
            activity_date: parse_date(&row.activity_date, "activity date")?,
            settlement_date: parse_optional_date(row.settlement_date, "settlement date")?,
            quantity: parse_optional_decimal(row.quantity, "activity quantity")?,
            unit_price: parse_optional_decimal(row.unit_price, "activity unit price")?,
            amount: parse_optional_decimal(row.amount, "activity amount")?,
            fee: parse_optional_decimal(row.fee, "activity fee")?,
            tax: parse_optional_decimal(row.tax, "activity tax")?,
            currency: row.currency,
            fx_rate: parse_optional_decimal(row.fx_rate, "activity fx rate")?,
            notes: row.notes,
            metadata: parse_json(row.metadata, "activity metadata")?,
            source_system: row.source_system,
            source_record_id: row.source_record_id,
            source_group_id: row.source_group_id,
            idempotency_key: row.idempotency_key,
            import_run_id: row.import_run_id,
            is_user_modified: row.is_user_modified,
            needs_review: row.needs_review,
            created_at: parse_timestamp(&row.created_at, "activity creation")?,
            updated_at: parse_timestamp(&row.updated_at, "activity update")?,
        })
    }
}

fn json_or_null(value: &Option<serde_json::Value>) -> Option<String> {
    value.as_ref().map(serde_json::Value::to_string)
}
