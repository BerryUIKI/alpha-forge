use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::error::AppError;
use domain::portfolio::{
    CreatePortfolioAccountInput, CreatePortfolioThemeLinkInput, CreatePortfolioTransactionInput,
    CreatePositionInput, PortfolioAccount, PortfolioTransaction, Position, ThemeExposure,
    ThesisAlignment, TransactionType,
};

pub struct PortfolioRepository {
    pool: SqlitePool,
}

impl PortfolioRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_account(
        &self,
        input: CreatePortfolioAccountInput,
    ) -> Result<PortfolioAccount, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO portfolio_accounts (id, workspace_id, name, account_type, currency, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&input.workspace_id)
        .bind(&input.name)
        .bind(&input.account_type)
        .bind(&input.currency)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|error| AppError::Internal(format!("Failed to create portfolio account: {error}")))?;

        Ok(PortfolioAccount {
            id,
            workspace_id: input.workspace_id,
            name: input.name,
            account_type: input.account_type,
            currency: input.currency,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn list_accounts(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<PortfolioAccount>, AppError> {
        let rows = sqlx::query_as::<_, AccountRow>("SELECT id, workspace_id, name, account_type, currency, created_at, updated_at FROM portfolio_accounts WHERE workspace_id = ? ORDER BY name")
            .bind(workspace_id).fetch_all(&self.pool).await
            .map_err(|error| AppError::Internal(format!("Failed to list portfolio accounts: {error}")))?;
        rows.into_iter().map(TryInto::try_into).collect()
    }

    pub async fn get_account(&self, id: &str) -> Result<Option<PortfolioAccount>, AppError> {
        let row = sqlx::query_as::<_, AccountRow>("SELECT id, workspace_id, name, account_type, currency, created_at, updated_at FROM portfolio_accounts WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await
            .map_err(|error| AppError::Internal(format!("Failed to get portfolio account: {error}")))?;
        row.map(TryInto::try_into).transpose()
    }

    pub async fn create_position(&self, input: CreatePositionInput) -> Result<Position, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        sqlx::query("INSERT INTO positions (id, account_id, symbol, quantity, cost_basis, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(&id).bind(&input.account_id).bind(&input.symbol).bind(input.quantity).bind(input.cost_basis)
            .bind(now.to_rfc3339()).bind(now.to_rfc3339()).execute(&self.pool).await
            .map_err(|error| AppError::Internal(format!("Failed to create position: {error}")))?;
        Ok(Position {
            id,
            account_id: input.account_id,
            symbol: input.symbol,
            quantity: input.quantity,
            cost_basis: input.cost_basis,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn list_positions(&self, account_id: &str) -> Result<Vec<Position>, AppError> {
        let rows = sqlx::query_as::<_, PositionRow>("SELECT id, account_id, symbol, quantity, cost_basis, created_at, updated_at FROM positions WHERE account_id = ? ORDER BY symbol")
            .bind(account_id).fetch_all(&self.pool).await
            .map_err(|error| AppError::Internal(format!("Failed to list positions: {error}")))?;
        rows.into_iter().map(TryInto::try_into).collect()
    }

    pub async fn list_positions_by_workspace(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<Position>, AppError> {
        let rows = sqlx::query_as::<_, PositionRow>("SELECT p.id, p.account_id, p.symbol, p.quantity, p.cost_basis, p.created_at, p.updated_at FROM positions p JOIN portfolio_accounts a ON a.id = p.account_id WHERE a.workspace_id = ? ORDER BY p.symbol")
            .bind(workspace_id).fetch_all(&self.pool).await
            .map_err(|error| AppError::Internal(format!("Failed to list workspace positions: {error}")))?;
        rows.into_iter().map(TryInto::try_into).collect()
    }
    pub async fn create_theme_link(
        &self,
        input: CreatePortfolioThemeLinkInput,
    ) -> Result<(), AppError> {
        sqlx::query("INSERT OR IGNORE INTO portfolio_theme_links (workspace_id, symbol, entity_id, created_at) VALUES (?, ?, ?, ?)").bind(input.workspace_id).bind(input.symbol).bind(input.entity_id).bind(Utc::now().to_rfc3339()).execute(&self.pool).await.map_err(|e| AppError::Internal(format!("Failed to link portfolio theme: {e}")))?;
        Ok(())
    }
    pub async fn theme_exposure(&self, workspace_id: &str) -> Result<Vec<ThemeExposure>, AppError> {
        let rows = sqlx::query_as::<_, ThemeExposureRow>("SELECT l.entity_id, e.name AS theme_name, SUM(ABS(p.quantity) * COALESCE(p.cost_basis, 0)) AS allocated_cost FROM portfolio_theme_links l JOIN knowledge_entities e ON e.id = l.entity_id JOIN positions p ON p.symbol = l.symbol JOIN portfolio_accounts a ON a.id = p.account_id WHERE l.workspace_id = ? AND a.workspace_id = ? GROUP BY l.entity_id, e.name ORDER BY allocated_cost DESC").bind(workspace_id).bind(workspace_id).fetch_all(&self.pool).await.map_err(|e| AppError::Internal(format!("Failed to calculate theme exposure: {e}")))?;
        let total: f64 = rows.iter().map(|row| row.allocated_cost).sum();
        Ok(rows
            .into_iter()
            .map(|row| ThemeExposure {
                entity_id: row.entity_id,
                theme_name: row.theme_name,
                allocated_cost: row.allocated_cost,
                weight_percent: if total > 0.0 {
                    row.allocated_cost / total * 100.0
                } else {
                    0.0
                },
            })
            .collect())
    }
    pub async fn thesis_alignment(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<ThesisAlignment>, AppError> {
        let rows = sqlx::query_as::<_, ThesisAlignmentRow>("SELECT DISTINCT p.symbol, t.id AS thesis_id, t.title AS thesis_title, t.confidence, t.status FROM positions p JOIN portfolio_accounts a ON a.id = p.account_id JOIN investment_theses t ON lower(t.title || ' ' || t.thesis) LIKE '%' || lower(p.symbol) || '%' WHERE a.workspace_id = ? AND t.workspace_id = ? ORDER BY p.symbol, t.updated_at DESC").bind(workspace_id).bind(workspace_id).fetch_all(&self.pool).await.map_err(|e| AppError::Internal(format!("Failed to check thesis alignment: {e}")))?;
        Ok(rows
            .into_iter()
            .map(|row| ThesisAlignment {
                symbol: row.symbol,
                thesis_id: row.thesis_id,
                thesis_title: row.thesis_title,
                confidence: row.confidence,
                status: row.status,
            })
            .collect())
    }

    pub async fn import_transactions(
        &self,
        inputs: Vec<CreatePortfolioTransactionInput>,
    ) -> Result<Vec<PortfolioTransaction>, AppError> {
        let mut database_transaction = self.pool.begin().await.map_err(|error| {
            AppError::Internal(format!("Failed to start transaction import: {error}"))
        })?;
        let created_at = Utc::now();
        let mut transactions = Vec::with_capacity(inputs.len());
        for input in inputs {
            let id = uuid::Uuid::new_v4().to_string();
            sqlx::query("INSERT INTO transactions (id, account_id, symbol, transaction_type, quantity, price, executed_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(&id).bind(&input.account_id).bind(&input.symbol).bind(input.transaction_type.to_string()).bind(input.quantity).bind(input.price).bind(input.executed_at.to_rfc3339()).bind(created_at.to_rfc3339()).execute(&mut *database_transaction).await
                .map_err(|error| AppError::Internal(format!("Failed to import transaction: {error}")))?;
            transactions.push(PortfolioTransaction {
                id,
                account_id: input.account_id,
                symbol: input.symbol,
                transaction_type: input.transaction_type,
                quantity: input.quantity,
                price: input.price,
                executed_at: input.executed_at,
                created_at,
            });
        }
        database_transaction.commit().await.map_err(|error| {
            AppError::Internal(format!("Failed to finish transaction import: {error}"))
        })?;
        Ok(transactions)
    }

    pub async fn list_transactions(
        &self,
        account_id: &str,
    ) -> Result<Vec<PortfolioTransaction>, AppError> {
        let rows = sqlx::query_as::<_, TransactionRow>("SELECT id, account_id, symbol, transaction_type, quantity, price, executed_at, created_at FROM transactions WHERE account_id = ? ORDER BY executed_at DESC")
            .bind(account_id).fetch_all(&self.pool).await.map_err(|error| AppError::Internal(format!("Failed to list transactions: {error}")))?;
        rows.into_iter().map(TryInto::try_into).collect()
    }
}

#[derive(sqlx::FromRow)]
struct AccountRow {
    id: String,
    workspace_id: String,
    name: String,
    account_type: String,
    currency: String,
    created_at: String,
    updated_at: String,
}

impl TryFrom<AccountRow> for PortfolioAccount {
    type Error = AppError;

    fn try_from(row: AccountRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            workspace_id: row.workspace_id,
            name: row.name,
            account_type: row.account_type,
            currency: row.currency,
            created_at: parse_timestamp(&row.created_at, "portfolio account creation")?,
            updated_at: parse_timestamp(&row.updated_at, "portfolio account update")?,
        })
    }
}

#[derive(sqlx::FromRow)]
struct PositionRow {
    id: String,
    account_id: String,
    symbol: String,
    quantity: f64,
    cost_basis: Option<f64>,
    created_at: String,
    updated_at: String,
}

#[derive(sqlx::FromRow)]
struct TransactionRow {
    id: String,
    account_id: String,
    symbol: String,
    transaction_type: String,
    quantity: f64,
    price: f64,
    executed_at: String,
    created_at: String,
}
#[derive(sqlx::FromRow)]
struct ThemeExposureRow {
    entity_id: String,
    theme_name: String,
    allocated_cost: f64,
}
#[derive(sqlx::FromRow)]
struct ThesisAlignmentRow {
    symbol: String,
    thesis_id: String,
    thesis_title: String,
    confidence: i32,
    status: String,
}

impl TryFrom<TransactionRow> for PortfolioTransaction {
    type Error = AppError;
    fn try_from(row: TransactionRow) -> Result<Self, Self::Error> {
        let transaction_type = TransactionType::parse(&row.transaction_type).ok_or_else(|| {
            AppError::Internal("Invalid transaction type in database".to_string())
        })?;
        Ok(Self {
            id: row.id,
            account_id: row.account_id,
            symbol: row.symbol,
            transaction_type,
            quantity: row.quantity,
            price: row.price,
            executed_at: parse_timestamp(&row.executed_at, "transaction execution")?,
            created_at: parse_timestamp(&row.created_at, "transaction creation")?,
        })
    }
}

impl TryFrom<PositionRow> for Position {
    type Error = AppError;

    fn try_from(row: PositionRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            account_id: row.account_id,
            symbol: row.symbol,
            quantity: row.quantity,
            cost_basis: row.cost_basis,
            created_at: parse_timestamp(&row.created_at, "position creation")?,
            updated_at: parse_timestamp(&row.updated_at, "position update")?,
        })
    }
}

fn parse_timestamp(value: &str, field: &str) -> Result<DateTime<Utc>, AppError> {
    DateTime::parse_from_rfc3339(value)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .map_err(|_| AppError::Internal(format!("Invalid {field} timestamp in database")))
}
