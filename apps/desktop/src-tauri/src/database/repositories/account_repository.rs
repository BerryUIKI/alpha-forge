// Financial repositories — platforms + accounts.
//
// SQLx persistence for the financial `platforms` and `accounts` tables
// (migration 0015). Follows `portfolio_repository.rs` conventions: thin
// methods, private `FromRow` structs, `TryFrom<Row> -> domain model` with
// typed errors.

use chrono::Utc;
use sqlx::SqlitePool;

use crate::database::repositories::financial_support::{parse_json, parse_timestamp};
use crate::error::AppError;
use domain::financial::{CreateAccountInput, CreatePlatformInput, FinancialAccount, Platform};

pub struct PlatformRepository {
    pool: SqlitePool,
}

impl PlatformRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreatePlatformInput) -> Result<Platform, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO platforms (id, name, url, kind, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&input.name)
        .bind(&input.url)
        .bind(&input.kind)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to create platform: {e}")))?;

        Ok(Platform {
            id,
            name: input.name,
            url: input.url,
            external_id: None,
            kind: input.kind,
            website_url: None,
            logo_url: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn list(&self) -> Result<Vec<Platform>, AppError> {
        let rows = sqlx::query_as::<_, PlatformRow>(
            "SELECT id, name, url, external_id, kind, website_url, logo_url, created_at, updated_at
             FROM platforms ORDER BY name",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to list platforms: {e}")))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    pub async fn get(&self, id: &str) -> Result<Option<Platform>, AppError> {
        let row = sqlx::query_as::<_, PlatformRow>(
            "SELECT id, name, url, external_id, kind, website_url, logo_url, created_at, updated_at
             FROM platforms WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to get platform: {e}")))?;

        row.map(TryInto::try_into).transpose()
    }
}

pub struct AccountRepository {
    pool: SqlitePool,
}

impl AccountRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateAccountInput) -> Result<FinancialAccount, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO accounts
                (id, workspace_id, name, account_type, group_name, currency, is_default,
                 platform_id, account_number, tracking_mode, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&input.workspace_id)
        .bind(&input.name)
        .bind(input.account_type.to_string())
        .bind(&input.group_name)
        .bind(&input.currency)
        .bind(input.is_default)
        .bind(&input.platform_id)
        .bind(&input.account_number)
        .bind(input.tracking_mode.to_string())
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to create financial account: {e}")))?;

        Ok(FinancialAccount {
            id,
            workspace_id: input.workspace_id,
            name: input.name,
            account_type: input.account_type,
            group_name: input.group_name,
            currency: input.currency,
            is_default: input.is_default,
            is_active: true,
            platform_id: input.platform_id,
            account_number: input.account_number,
            meta: None,
            provider: None,
            provider_account_id: None,
            is_archived: false,
            tracking_mode: input.tracking_mode,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn list(&self) -> Result<Vec<FinancialAccount>, AppError> {
        let rows = sqlx::query_as::<_, AccountRow>(
            "SELECT id, workspace_id, name, account_type, group_name, currency, is_default,
                    is_active, platform_id, account_number, meta, provider, provider_account_id,
                    is_archived, tracking_mode, created_at, updated_at
             FROM accounts ORDER BY name",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to list financial accounts: {e}")))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    pub async fn list_by_workspace(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<FinancialAccount>, AppError> {
        let rows = sqlx::query_as::<_, AccountRow>(
            "SELECT id, workspace_id, name, account_type, group_name, currency, is_default,
                    is_active, platform_id, account_number, meta, provider, provider_account_id,
                    is_archived, tracking_mode, created_at, updated_at
             FROM accounts WHERE workspace_id = ? ORDER BY name",
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to list workspace accounts: {e}")))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    pub async fn get(&self, id: &str) -> Result<Option<FinancialAccount>, AppError> {
        let row = sqlx::query_as::<_, AccountRow>(
            "SELECT id, workspace_id, name, account_type, group_name, currency, is_default,
                    is_active, platform_id, account_number, meta, provider, provider_account_id,
                    is_archived, tracking_mode, created_at, updated_at
             FROM accounts WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to get financial account: {e}")))?;

        row.map(TryInto::try_into).transpose()
    }
}

#[derive(sqlx::FromRow)]
struct PlatformRow {
    id: String,
    name: Option<String>,
    url: String,
    external_id: Option<String>,
    kind: String,
    website_url: Option<String>,
    logo_url: Option<String>,
    created_at: String,
    updated_at: String,
}

impl TryFrom<PlatformRow> for Platform {
    type Error = AppError;

    fn try_from(row: PlatformRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            name: row.name,
            url: row.url,
            external_id: row.external_id,
            kind: row.kind,
            website_url: row.website_url,
            logo_url: row.logo_url,
            created_at: parse_timestamp(&row.created_at, "platform creation")?,
            updated_at: parse_timestamp(&row.updated_at, "platform update")?,
        })
    }
}

#[derive(sqlx::FromRow)]
struct AccountRow {
    id: String,
    workspace_id: Option<String>,
    name: String,
    account_type: String,
    group_name: Option<String>,
    currency: String,
    is_default: bool,
    is_active: bool,
    platform_id: Option<String>,
    account_number: Option<String>,
    meta: Option<String>,
    provider: Option<String>,
    provider_account_id: Option<String>,
    is_archived: bool,
    tracking_mode: String,
    created_at: String,
    updated_at: String,
}

impl TryFrom<AccountRow> for FinancialAccount {
    type Error = AppError;

    fn try_from(row: AccountRow) -> Result<Self, Self::Error> {
        let account_type =
            domain::financial::AccountType::parse(&row.account_type).ok_or_else(|| {
                AppError::Internal(format!(
                    "invalid account_type in database: {}",
                    row.account_type
                ))
            })?;
        let tracking_mode =
            domain::financial::TrackingMode::parse(&row.tracking_mode).ok_or_else(|| {
                AppError::Internal(format!(
                    "invalid tracking_mode in database: {}",
                    row.tracking_mode
                ))
            })?;

        Ok(Self {
            id: row.id,
            workspace_id: row.workspace_id,
            name: row.name,
            account_type,
            group_name: row.group_name,
            currency: row.currency,
            is_default: row.is_default,
            is_active: row.is_active,
            platform_id: row.platform_id,
            account_number: row.account_number,
            meta: parse_json(row.meta, "account meta")?,
            provider: row.provider,
            provider_account_id: row.provider_account_id,
            is_archived: row.is_archived,
            tracking_mode,
            created_at: parse_timestamp(&row.created_at, "account creation")?,
            updated_at: parse_timestamp(&row.updated_at, "account update")?,
        })
    }
}
