// Portfolio Tauri commands — Phase 1 placeholder.

use crate::app::state::AppState;
use crate::error::AppError;
use domain::portfolio::{
    ConcentrationRisk, CreatePortfolioAccountInput, CreatePortfolioThemeLinkInput,
    CreatePositionInput, PortfolioAccount, PortfolioAllocation, PortfolioReview,
    PortfolioTransaction, Position, ThemeExposure, ThesisAlignment,
};
use tauri::State;

#[tauri::command]
pub async fn create_portfolio_account(
    workspace_id: String,
    name: String,
    account_type: String,
    currency: String,
    state: State<'_, AppState>,
) -> Result<PortfolioAccount, AppError> {
    state
        .portfolio_service
        .create_account(CreatePortfolioAccountInput {
            workspace_id,
            name,
            account_type,
            currency,
        })
        .await
}
#[tauri::command]
pub async fn list_portfolio_accounts(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<PortfolioAccount>, AppError> {
    state.portfolio_service.list_accounts(&workspace_id).await
}
#[tauri::command]
pub async fn create_portfolio_position(
    account_id: String,
    symbol: String,
    quantity: f64,
    cost_basis: Option<f64>,
    state: State<'_, AppState>,
) -> Result<Position, AppError> {
    state
        .portfolio_service
        .create_position(CreatePositionInput {
            account_id,
            symbol,
            quantity,
            cost_basis,
        })
        .await
}
#[tauri::command]
pub async fn list_portfolio_positions(
    account_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Position>, AppError> {
    state.portfolio_service.list_positions(&account_id).await
}

#[tauri::command]
pub async fn import_portfolio_transactions_csv(
    account_id: String,
    csv_text: String,
    state: State<'_, AppState>,
) -> Result<Vec<PortfolioTransaction>, AppError> {
    state
        .portfolio_service
        .import_transactions_csv(&account_id, &csv_text)
        .await
}

#[tauri::command]
pub async fn list_portfolio_transactions(
    account_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<PortfolioTransaction>, AppError> {
    state.portfolio_service.list_transactions(&account_id).await
}

#[tauri::command]
pub async fn get_portfolio_allocation(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<PortfolioAllocation>, AppError> {
    state
        .portfolio_service
        .allocation_by_workspace(&workspace_id)
        .await
}

#[tauri::command]
pub async fn get_portfolio_concentration_risks(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ConcentrationRisk>, AppError> {
    state
        .portfolio_service
        .concentration_risks(&workspace_id)
        .await
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
) -> Result<Vec<ThemeExposure>, AppError> {
    state.portfolio_service.theme_exposure(&workspace_id).await
}
#[tauri::command]
pub async fn get_portfolio_thesis_alignment(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ThesisAlignment>, AppError> {
    state
        .portfolio_service
        .thesis_alignment(&workspace_id)
        .await
}
#[tauri::command]
pub async fn generate_portfolio_review(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<PortfolioReview, AppError> {
    state.portfolio_service.review(&workspace_id).await
}
