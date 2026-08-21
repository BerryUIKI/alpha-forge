// Portfolio Tauri commands — Phase 1 placeholder & Phase 2.4 normalized IPC DTOs.

use crate::app::state::AppState;
use crate::error::AppError;
use domain::portfolio::{
    ConcentrationRisk, ConcentrationSeverity, CreatePortfolioAccountInput,
    CreatePortfolioThemeLinkInput, CreatePositionInput, PortfolioAccount, PortfolioAllocation,
    PortfolioReview, PortfolioTransaction, Position, ThemeExposure, ThesisAlignment,
    TransactionType,
};
use serde::{Deserialize, Serialize};
use tauri::State;

// ── DTOs ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioAccountDto {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub account_type: String,
    pub currency: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<PortfolioAccount> for PortfolioAccountDto {
    fn from(account: PortfolioAccount) -> Self {
        Self {
            id: account.id,
            workspace_id: account.workspace_id,
            name: account.name,
            account_type: account.account_type,
            currency: account.currency,
            created_at: account.created_at.to_rfc3339(),
            updated_at: account.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionDto {
    pub id: String,
    pub account_id: String,
    pub symbol: String,
    pub quantity: f64,
    pub cost_basis: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Position> for PositionDto {
    fn from(position: Position) -> Self {
        Self {
            id: position.id,
            account_id: position.account_id,
            symbol: position.symbol,
            quantity: position.quantity,
            cost_basis: position.cost_basis,
            created_at: position.created_at.to_rfc3339(),
            updated_at: position.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioTransactionDto {
    pub id: String,
    pub account_id: String,
    pub symbol: String,
    pub transaction_type: TransactionType,
    pub quantity: f64,
    pub price: f64,
    pub executed_at: String,
    pub created_at: String,
}

impl From<PortfolioTransaction> for PortfolioTransactionDto {
    fn from(tx: PortfolioTransaction) -> Self {
        Self {
            id: tx.id,
            account_id: tx.account_id,
            symbol: tx.symbol,
            transaction_type: tx.transaction_type,
            quantity: tx.quantity,
            price: tx.price,
            executed_at: tx.executed_at.to_rfc3339(),
            created_at: tx.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioAllocationDto {
    pub symbol: String,
    pub allocated_cost: f64,
    pub weight_percent: f64,
    pub account_count: usize,
}

impl From<PortfolioAllocation> for PortfolioAllocationDto {
    fn from(a: PortfolioAllocation) -> Self {
        Self {
            symbol: a.symbol,
            allocated_cost: a.allocated_cost,
            weight_percent: a.weight_percent,
            account_count: a.account_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConcentrationRiskDto {
    pub symbol: String,
    pub weight_percent: f64,
    pub severity: ConcentrationSeverity,
    pub message: String,
}

impl From<ConcentrationRisk> for ConcentrationRiskDto {
    fn from(r: ConcentrationRisk) -> Self {
        Self {
            symbol: r.symbol,
            weight_percent: r.weight_percent,
            severity: r.severity,
            message: r.message,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeExposureDto {
    pub entity_id: String,
    pub theme_name: String,
    pub allocated_cost: f64,
    pub weight_percent: f64,
}

impl From<ThemeExposure> for ThemeExposureDto {
    fn from(t: ThemeExposure) -> Self {
        Self {
            entity_id: t.entity_id,
            theme_name: t.theme_name,
            allocated_cost: t.allocated_cost,
            weight_percent: t.weight_percent,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThesisAlignmentDto {
    pub symbol: String,
    pub thesis_id: String,
    pub thesis_title: String,
    pub confidence: i32,
    pub status: String,
}

impl From<ThesisAlignment> for ThesisAlignmentDto {
    fn from(t: ThesisAlignment) -> Self {
        Self {
            symbol: t.symbol,
            thesis_id: t.thesis_id,
            thesis_title: t.thesis_title,
            confidence: t.confidence,
            status: t.status,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioReviewDto {
    pub generated_at: String,
    pub concentration_risks: Vec<ConcentrationRiskDto>,
    pub unaligned_symbols: Vec<String>,
}

impl From<PortfolioReview> for PortfolioReviewDto {
    fn from(r: PortfolioReview) -> Self {
        Self {
            generated_at: r.generated_at.to_rfc3339(),
            concentration_risks: r
                .concentration_risks
                .into_iter()
                .map(ConcentrationRiskDto::from)
                .collect(),
            unaligned_symbols: r.unaligned_symbols,
        }
    }
}

// ── Commands ────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_portfolio_account(
    workspace_id: String,
    name: String,
    account_type: String,
    currency: String,
    state: State<'_, AppState>,
) -> Result<PortfolioAccountDto, AppError> {
    state
        .portfolio_service
        .create_account(CreatePortfolioAccountInput {
            workspace_id,
            name,
            account_type,
            currency,
        })
        .await
        .map(PortfolioAccountDto::from)
}

#[tauri::command]
pub async fn list_portfolio_accounts(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<PortfolioAccountDto>, AppError> {
    state
        .portfolio_service
        .list_accounts(&workspace_id)
        .await
        .map(|accounts| {
            accounts
                .into_iter()
                .map(PortfolioAccountDto::from)
                .collect()
        })
}

#[tauri::command]
pub async fn create_portfolio_position(
    account_id: String,
    symbol: String,
    quantity: f64,
    cost_basis: Option<f64>,
    state: State<'_, AppState>,
) -> Result<PositionDto, AppError> {
    state
        .portfolio_service
        .create_position(CreatePositionInput {
            account_id,
            symbol,
            quantity,
            cost_basis,
        })
        .await
        .map(PositionDto::from)
}

#[tauri::command]
pub async fn list_portfolio_positions(
    account_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<PositionDto>, AppError> {
    state
        .portfolio_service
        .list_positions(&account_id)
        .await
        .map(|positions| positions.into_iter().map(PositionDto::from).collect())
}

#[tauri::command]
pub async fn import_portfolio_transactions_csv(
    account_id: String,
    csv_text: String,
    state: State<'_, AppState>,
) -> Result<Vec<PortfolioTransactionDto>, AppError> {
    state
        .portfolio_service
        .import_transactions_csv(&account_id, &csv_text)
        .await
        .map(|txs| txs.into_iter().map(PortfolioTransactionDto::from).collect())
}

#[tauri::command]
pub async fn list_portfolio_transactions(
    account_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<PortfolioTransactionDto>, AppError> {
    state
        .portfolio_service
        .list_transactions(&account_id)
        .await
        .map(|txs| txs.into_iter().map(PortfolioTransactionDto::from).collect())
}

#[tauri::command]
pub async fn get_portfolio_allocation(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<PortfolioAllocationDto>, AppError> {
    state
        .portfolio_service
        .allocation_by_workspace(&workspace_id)
        .await
        .map(|allocations| {
            allocations
                .into_iter()
                .map(PortfolioAllocationDto::from)
                .collect()
        })
}

#[tauri::command]
pub async fn get_portfolio_concentration_risks(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ConcentrationRiskDto>, AppError> {
    state
        .portfolio_service
        .concentration_risks(&workspace_id)
        .await
        .map(|risks| risks.into_iter().map(ConcentrationRiskDto::from).collect())
}

#[tauri::command]
pub async fn link_portfolio_theme(
    workspace_id: String,
    symbol: String,
    entity_id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state
        .portfolio_service
        .link_theme(CreatePortfolioThemeLinkInput {
            workspace_id,
            symbol,
            entity_id,
        })
        .await
}

#[tauri::command]
pub async fn get_portfolio_theme_exposure(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ThemeExposureDto>, AppError> {
    state
        .portfolio_service
        .theme_exposure(&workspace_id)
        .await
        .map(|exposures| exposures.into_iter().map(ThemeExposureDto::from).collect())
}

#[tauri::command]
pub async fn get_portfolio_thesis_alignment(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ThesisAlignmentDto>, AppError> {
    state
        .portfolio_service
        .thesis_alignment(&workspace_id)
        .await
        .map(|alignments| {
            alignments
                .into_iter()
                .map(ThesisAlignmentDto::from)
                .collect()
        })
}

#[tauri::command]
pub async fn generate_portfolio_review(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<PortfolioReviewDto, AppError> {
    state
        .portfolio_service
        .review(&workspace_id)
        .await
        .map(PortfolioReviewDto::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_portfolio_account_dto_serialization() {
        let dto = PortfolioAccountDto {
            id: "acc-1".to_string(),
            workspace_id: "ws-1".to_string(),
            name: "Main Brokerage".to_string(),
            account_type: "securities".to_string(),
            currency: "USD".to_string(),
            created_at: "2026-08-21T00:00:00Z".to_string(),
            updated_at: "2026-08-21T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&dto).expect("failed to serialize");
        assert!(json.contains("\"workspaceId\":\"ws-1\""));
        assert!(json.contains("\"accountType\":\"securities\""));
        assert!(json.contains("\"createdAt\":\"2026-08-21T00:00:00Z\""));

        let deserialized: PortfolioAccountDto =
            serde_json::from_str(&json).expect("failed to deserialize");
        assert_eq!(deserialized, dto);
    }

    #[test]
    fn test_position_dto_serialization() {
        let dto = PositionDto {
            id: "pos-1".to_string(),
            account_id: "acc-1".to_string(),
            symbol: "AAPL".to_string(),
            quantity: 100.0,
            cost_basis: Some(15000.0),
            created_at: "2026-08-21T00:00:00Z".to_string(),
            updated_at: "2026-08-21T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&dto).expect("failed to serialize");
        assert!(json.contains("\"accountId\":\"acc-1\""));
        assert!(json.contains("\"costBasis\":15000.0"));

        let deserialized: PositionDto = serde_json::from_str(&json).expect("failed to deserialize");
        assert_eq!(deserialized, dto);
    }

    #[test]
    fn test_portfolio_transaction_dto_serialization() {
        let dto = PortfolioTransactionDto {
            id: "tx-1".to_string(),
            account_id: "acc-1".to_string(),
            symbol: "AAPL".to_string(),
            transaction_type: TransactionType::Buy,
            quantity: 50.0,
            price: 150.0,
            executed_at: "2026-08-21T00:00:00Z".to_string(),
            created_at: "2026-08-21T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&dto).expect("failed to serialize");
        assert!(json.contains("\"accountId\":\"acc-1\""));
        assert!(json.contains("\"transactionType\":\"buy\""));
        assert!(json.contains("\"executedAt\":\"2026-08-21T00:00:00Z\""));

        let deserialized: PortfolioTransactionDto =
            serde_json::from_str(&json).expect("failed to deserialize");
        assert_eq!(deserialized, dto);
    }

    #[test]
    fn test_portfolio_review_dto_serialization() {
        let dto = PortfolioReviewDto {
            generated_at: "2026-08-21T00:00:00Z".to_string(),
            concentration_risks: vec![ConcentrationRiskDto {
                symbol: "AAPL".to_string(),
                weight_percent: 35.0,
                severity: ConcentrationSeverity::High,
                message: "High concentration in AAPL".to_string(),
            }],
            unaligned_symbols: vec!["NVDA".to_string()],
        };

        let json = serde_json::to_string(&dto).expect("failed to serialize");
        assert!(json.contains("\"generatedAt\":\"2026-08-21T00:00:00Z\""));
        assert!(json.contains("\"concentrationRisks\":["));
        assert!(json.contains("\"unalignedSymbols\":[\"NVDA\"]"));

        let deserialized: PortfolioReviewDto =
            serde_json::from_str(&json).expect("failed to deserialize");
        assert_eq!(deserialized, dto);
    }
}
