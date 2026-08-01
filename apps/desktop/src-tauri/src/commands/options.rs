// Option IPC commands

use tauri::State;
use crate::app::state::AppState;
use crate::error::AppError;
use domain::option::{OptionChain, OptionType, DataSource};
use serde::{Deserialize, Serialize};

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
    
    state.option_service.fetch_chain(&params.symbol, &params.workspace_id, source).await
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