// Option IPC commands

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use domain::option::{DataSource, OptionChain, OptionContract, OptionStrategy, OptionType, StrategyType};

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
    // Validate inputs
    if params.symbol.trim().is_empty() {
        return Err(AppError::Validation("Symbol cannot be empty".to_string()));
    }
    if params.workspace_id.trim().is_empty() {
        return Err(AppError::Validation(
            "Workspace ID cannot be empty".to_string(),
        ));
    }

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
    // Validate inputs
    if params.underlying_price <= 0.0 {
        return Err(AppError::Validation(
            "Underlying price must be positive".to_string(),
        ));
    }
    if params.strike <= 0.0 {
        return Err(AppError::Validation(
            "Strike price must be positive".to_string(),
        ));
    }
    if params.expiration_years <= 0.0 {
        return Err(AppError::Validation(
            "Time to expiration must be positive".to_string(),
        ));
    }
    if params.volatility <= 0.0 {
        return Err(AppError::Validation(
            "Volatility must be positive".to_string(),
        ));
    }

    let option_type = match params.option_type.as_str() {
        "call" => OptionType::Call,
        "put" => OptionType::Put,
        _ => {
            return Err(AppError::Validation(
                "Invalid option type: must be 'call' or 'put'".to_string(),
            ))
        }
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
    // Validate inputs
    if params.underlying_price <= 0.0 {
        return Err(AppError::Validation(
            "Underlying price must be positive".to_string(),
        ));
    }
    if params.strike <= 0.0 {
        return Err(AppError::Validation(
            "Strike price must be positive".to_string(),
        ));
    }
    if params.expiration_years <= 0.0 {
        return Err(AppError::Validation(
            "Time to expiration must be positive".to_string(),
        ));
    }
    if params.volatility <= 0.0 {
        return Err(AppError::Validation(
            "Volatility must be positive".to_string(),
        ));
    }

    let option_type = match params.option_type.as_str() {
        "call" => OptionType::Call,
        "put" => OptionType::Put,
        _ => {
            return Err(AppError::Validation(
                "Invalid option type: must be 'call' or 'put'".to_string(),
            ))
        }
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

/// Calculate implied volatility
#[tauri::command]
pub async fn calculate_implied_volatility(
    params: CalculateIVParams,
    state: State<'_, AppState>,
) -> Result<f64, AppError> {
    // Validate inputs
    if params.underlying_price <= 0.0 {
        return Err(AppError::Validation(
            "Underlying price must be positive".to_string(),
        ));
    }
    if params.strike <= 0.0 {
        return Err(AppError::Validation(
            "Strike price must be positive".to_string(),
        ));
    }
    if params.expiration_years <= 0.0 {
        return Err(AppError::Validation(
            "Time to expiration must be positive".to_string(),
        ));
    }
    if params.market_price <= 0.0 {
        return Err(AppError::Validation(
            "Market price must be positive".to_string(),
        ));
    }

    let option_type = match params.option_type.as_str() {
        "call" => OptionType::Call,
        "put" => OptionType::Put,
        _ => {
            return Err(AppError::Validation(
                "Invalid option type: must be 'call' or 'put'".to_string(),
            ))
        }
    };

    state.option_service.calculate_iv(
        option_type,
        params.underlying_price,
        params.strike,
        params.expiration_years,
        params.risk_free_rate,
        params.dividend_yield.unwrap_or(0.0),
        params.market_price,
    )
}

#[derive(Debug, Deserialize)]
pub struct CalculateIVParams {
    pub option_type: String,
    pub underlying_price: f64,
    pub strike: f64,
    pub expiration_years: f64,
    pub risk_free_rate: f64,
    pub dividend_yield: Option<f64>,
    pub market_price: f64,
}

// ============================================
// Option Chain CRUD Commands
// ============================================

/// Get an option chain by ID
#[tauri::command]
pub async fn get_option_chain(
    id: String,
    state: State<'_, AppState>,
) -> Result<OptionChain, AppError> {
    state.option_service.get_chain(&id).await
}

/// List all option chains for a workspace
#[tauri::command]
pub async fn list_option_chains(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<OptionChain>, AppError> {
    state.option_service.list_chains(&workspace_id).await
}

/// Delete an option chain
#[tauri::command]
pub async fn delete_option_chain(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.option_service.delete_chain(&id).await
}

// ============================================
// Option Contract CRUD Commands
// ============================================

/// Create a new option contract
#[tauri::command]
pub async fn create_option_contract(
    contract: CreateContractParams,
    state: State<'_, AppState>,
) -> Result<OptionContract, AppError> {
    let contract = domain::option::OptionContract {
        id: uuid::Uuid::new_v4().to_string(),
        workspace_id: contract.workspace_id,
        chain_id: contract.chain_id,
        symbol: contract.symbol,
        option_type: match contract.option_type.as_str() {
            "call" => OptionType::Call,
            "put" => OptionType::Put,
            _ => return Err(AppError::Validation("Invalid option type".to_string())),
        },
        strike: contract.strike,
        expiration: contract.expiration,
        contract_multiplier: contract.contract_multiplier.unwrap_or(100),
        bid: contract.bid.unwrap_or(0.0),
        ask: contract.ask.unwrap_or(0.0),
        last: contract.last,
        volume: contract.volume.unwrap_or(0),
        open_interest: contract.open_interest.unwrap_or(0),
        implied_volatility: contract.implied_volatility.unwrap_or(0.0),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    state.option_service.create_contract(&contract).await?;
    Ok(contract)
}

/// Get an option contract by ID
#[tauri::command]
pub async fn get_option_contract(
    id: String,
    state: State<'_, AppState>,
) -> Result<OptionContract, AppError> {
    state.option_service.get_contract(&id).await
}

/// List all option contracts for a chain
#[tauri::command]
pub async fn list_option_contracts(
    chain_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<OptionContract>, AppError> {
    state.option_service.list_contracts(&chain_id).await
}

/// Delete an option contract
#[tauri::command]
pub async fn delete_option_contract(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.option_service.delete_contract(&id).await
}

#[derive(Debug, Deserialize)]
pub struct CreateContractParams {
    pub workspace_id: String,
    pub chain_id: String,
    pub symbol: String,
    pub option_type: String,
    pub strike: f64,
    pub expiration: chrono::DateTime<chrono::Utc>,
    pub contract_multiplier: Option<u32>,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub last: Option<f64>,
    pub volume: Option<u64>,
    pub open_interest: Option<u64>,
    pub implied_volatility: Option<f64>,
}

// ============================================
// Option Strategy CRUD Commands
// ============================================

/// Create a new option strategy
#[tauri::command]
pub async fn create_option_strategy(
    params: CreateStrategyParams,
    state: State<'_, AppState>,
) -> Result<OptionStrategy, AppError> {
    let strategy = domain::option::OptionStrategy {
        id: uuid::Uuid::new_v4().to_string(),
        workspace_id: params.workspace_id,
        name: params.name,
        strategy_type: match params.strategy_type.as_str() {
            "long_call" => StrategyType::LongCall,
            "long_put" => StrategyType::LongPut,
            "covered_call" => StrategyType::CoveredCall,
            "protective_put" => StrategyType::ProtectivePut,
            "bull_call_spread" => StrategyType::BullCallSpread,
            "bear_put_spread" => StrategyType::BearPutSpread,
            "straddle" => StrategyType::Straddle,
            "strangle" => StrategyType::Strangle,
            "iron_condor" => StrategyType::IronCondor,
            "butterfly" => StrategyType::Butterfly,
            _ => StrategyType::Custom,
        },
        underlying: params.underlying,
        total_cost: params.total_cost.unwrap_or(0.0),
        max_profit: params.max_profit,
        max_loss: params.max_loss,
        break_even_points: params.break_even_points.unwrap_or_default(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    state.option_service.create_strategy(&strategy).await?;
    Ok(strategy)
}

/// Get an option strategy by ID
#[tauri::command]
pub async fn get_option_strategy(
    id: String,
    state: State<'_, AppState>,
) -> Result<OptionStrategy, AppError> {
    state.option_service.get_strategy(&id).await
}

/// List all option strategies for a workspace
#[tauri::command]
pub async fn list_option_strategies(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<OptionStrategy>, AppError> {
    state.option_service.list_strategies(&workspace_id).await
}

/// Update an option strategy
#[tauri::command]
pub async fn update_option_strategy(
    params: UpdateStrategyParams,
    state: State<'_, AppState>,
) -> Result<OptionStrategy, AppError> {
    let mut strategy = state.option_service.get_strategy(&params.id).await?;
    
    if let Some(name) = params.name {
        strategy.name = name;
    }
    if let Some(total_cost) = params.total_cost {
        strategy.total_cost = total_cost;
    }
    if let Some(max_profit) = params.max_profit {
        strategy.max_profit = Some(max_profit);
    }
    if let Some(max_loss) = params.max_loss {
        strategy.max_loss = Some(max_loss);
    }
    if let Some(break_even_points) = params.break_even_points {
        strategy.break_even_points = break_even_points;
    }
    strategy.updated_at = chrono::Utc::now();
    
    state.option_service.update_strategy(&strategy).await?;
    Ok(strategy)
}

/// Delete an option strategy
#[tauri::command]
pub async fn delete_option_strategy(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.option_service.delete_strategy(&id).await
}

#[derive(Debug, Deserialize)]
pub struct CreateStrategyParams {
    pub workspace_id: String,
    pub name: String,
    pub strategy_type: String,
    pub underlying: String,
    pub total_cost: Option<f64>,
    pub max_profit: Option<f64>,
    pub max_loss: Option<f64>,
    pub break_even_points: Option<Vec<f64>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStrategyParams {
    pub id: String,
    pub name: Option<String>,
    pub total_cost: Option<f64>,
    pub max_profit: Option<f64>,
    pub max_loss: Option<f64>,
    pub break_even_points: Option<Vec<f64>>,
}
