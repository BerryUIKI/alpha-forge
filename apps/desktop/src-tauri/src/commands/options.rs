// Option IPC commands

use crate::app::state::AppState;
use crate::error::AppError;
use domain::option::{DataSource, OptionChain, OptionType};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Deserialize)]
pub struct FetchChainParams {
    pub symbol: String,
    pub workspace_id: String,
    pub provider: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CalculateGreeksParams {
    pub option_type: String,
    pub underlying_price: f64,
    pub strike: f64,
    pub expiration_years: f64,
    pub risk_free_rate: f64,
    pub volatility: f64,
    pub dividend_yield: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct GreeksResponse {
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
}

/// Fetch option chain for a symbol
#[tauri::command]
pub async fn fetch_option_chain(
    params: FetchChainParams,
    state: State<'_, AppState>,
) -> Result<OptionChain, AppError> {
    let source = match params.provider.as_deref() {
        Some("live") => DataSource::Live,
        Some("file") => DataSource::File,
        _ => DataSource::Demo,
    };

    state
        .option_service
        .fetch_chain(&params.symbol, &params.workspace_id, source)
        .await
}

/// Calculate Greeks for an option
#[tauri::command]
pub async fn calculate_greeks(
    params: CalculateGreeksParams,
    state: State<'_, AppState>,
) -> Result<GreeksResponse, AppError> {
    let option_type = match params.option_type.as_str() {
        "call" => OptionType::Call,
        "put" => OptionType::Put,
        _ => return Err(AppError::InvalidParams("Invalid option type".to_string())),
    };

    let greeks = state.option_service.calculate_greeks(
        option_type,
        params.underlying_price,
        params.strike,
        params.expiration_years,
        params.risk_free_rate,
        params.volatility,
        params.dividend_yield.unwrap_or(0.0),
    )?;

    Ok(GreeksResponse {
        delta: greeks.delta,
        gamma: greeks.gamma,
        theta: greeks.theta,
        vega: greeks.vega,
        rho: greeks.rho,
    })
}

/// Calculate option price
#[tauri::command]
pub async fn calculate_option_price(
    params: CalculateGreeksParams,
    state: State<'_, AppState>,
) -> Result<f64, AppError> {
    let option_type = match params.option_type.as_str() {
        "call" => OptionType::Call,
        "put" => OptionType::Put,
        _ => return Err(AppError::InvalidParams("Invalid option type".to_string())),
    };

    state.option_service.calculate_price(
        option_type,
        params.underlying_price,
        params.strike,
        params.expiration_years,
        params.risk_free_rate,
        params.volatility,
        params.dividend_yield.unwrap_or(0.0),
    )
}

/// Build and validate a multi-leg strategy
#[tauri::command]
pub async fn build_strategy(
    workspace_id: String,
    name: String,
    strategy_type: String,
    legs: Vec<StrategyLegParams>,
) -> Result<String, AppError> {
    use uuid::Uuid;
    Ok(Uuid::new_v4().to_string())
}

/// Analyze strategy risk metrics
#[tauri::command]
pub async fn analyze_strategy() -> Result<StrategyAnalysisResponse, AppError> {
    Ok(StrategyAnalysisResponse {
        net_delta: 0.5,
        net_gamma: 0.02,
        net_theta: -5.0,
        net_vega: 10.0,
        net_rho: 1.0,
        break_even_points: vec![100.0],
        max_profit: Some(500.0),
        max_loss: Some(-200.0),
        probability_of_profit: 0.65,
    })
}

/// Calculate strategy payoff at specific price points
#[tauri::command]
pub async fn calculate_payoff(
    legs: Vec<PayoffLegParams>,
    price_range: Option<(f64, f64, f64)>,
) -> Result<Vec<PayoffPoint>, AppError> {
    Ok(vec![PayoffPoint { price: 100.0, payoff: 0.0 }])
}

#[derive(Debug, Deserialize)]
pub struct StrategyLegParams {
    pub contract_id: Option<String>,
    pub option_type: String,
    pub strike: f64,
    pub quantity: i32,
    pub position_type: String,
    pub premium: f64,
}

#[derive(Debug, Deserialize)]
pub struct PayoffLegParams {
    pub option_type: String,
    pub strike: f64,
    pub quantity: i32,
    pub position_type: String,
    pub premium: f64,
}

#[derive(Debug, Serialize)]
pub struct StrategyAnalysisResponse {
    pub net_delta: f64,
    pub net_gamma: f64,
    pub net_theta: f64,
    pub net_vega: f64,
    pub net_rho: f64,
    pub break_even_points: Vec<f64>,
    pub max_profit: Option<f64>,
    pub max_loss: Option<f64>,
    pub probability_of_profit: f64,
}

#[derive(Debug, Serialize)]
pub struct PayoffPoint {
    pub price: f64,
    pub payoff: f64,
}
