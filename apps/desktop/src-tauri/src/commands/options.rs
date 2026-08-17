// Option IPC commands

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::app::state::AppState;
use crate::error::AppError;
use crate::services::strategy_service::{
    CreateStrategyInput, CreateStrategyLegInput, StrategyWithLegs,
};
use domain::option::{
    DataSource, OptionChain, OptionContract, OptionType, PositionType, StrategyLeg, StrategyType,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChainResponse {
    pub id: String,
    pub workspace_id: String,
    pub symbol: String,
    pub underlying_price: f64,
    pub as_of: chrono::DateTime<chrono::Utc>,
    pub data_source: DataSource,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<OptionChain> for OptionChainResponse {
    fn from(chain: OptionChain) -> Self {
        Self {
            id: chain.id,
            workspace_id: chain.workspace_id,
            symbol: chain.symbol,
            underlying_price: chain.underlying_price,
            as_of: chain.as_of,
            data_source: chain.data_source,
            created_at: chain.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionContractResponse {
    pub id: String,
    pub workspace_id: String,
    pub chain_id: String,
    pub symbol: String,
    pub option_type: OptionType,
    pub strike: f64,
    pub expiration: chrono::DateTime<chrono::Utc>,
    pub contract_multiplier: u32,
    pub bid: f64,
    pub ask: f64,
    pub last: Option<f64>,
    pub volume: u64,
    pub open_interest: u64,
    pub implied_volatility: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<OptionContract> for OptionContractResponse {
    fn from(contract: OptionContract) -> Self {
        Self {
            id: contract.id,
            workspace_id: contract.workspace_id,
            chain_id: contract.chain_id,
            symbol: contract.symbol,
            option_type: contract.option_type,
            strike: contract.strike,
            expiration: contract.expiration,
            contract_multiplier: contract.contract_multiplier,
            bid: contract.bid,
            ask: contract.ask,
            last: contract.last,
            volume: contract.volume,
            open_interest: contract.open_interest,
            implied_volatility: contract.implied_volatility,
            created_at: contract.created_at,
            updated_at: contract.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionStrategyResponse {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub strategy_type: StrategyType,
    pub underlying: String,
    pub total_cost: f64,
    pub max_profit: Option<f64>,
    pub max_loss: Option<f64>,
    pub break_even_points: Vec<f64>,
    pub legs: Vec<StrategyLegResponse>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<StrategyWithLegs> for OptionStrategyResponse {
    fn from(details: StrategyWithLegs) -> Self {
        let strategy = details.strategy;
        Self {
            id: strategy.id,
            workspace_id: strategy.workspace_id,
            name: strategy.name,
            strategy_type: strategy.strategy_type,
            underlying: strategy.underlying,
            total_cost: strategy.total_cost,
            max_profit: strategy.max_profit,
            max_loss: strategy.max_loss,
            break_even_points: strategy.break_even_points,
            legs: details.legs.into_iter().map(Into::into).collect(),
            created_at: strategy.created_at,
            updated_at: strategy.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StrategyLegResponse {
    pub id: String,
    pub strategy_id: String,
    pub option_contract_id: String,
    pub quantity: i32,
    pub position_type: PositionType,
    pub premium: f64,
    pub strike: f64,
    pub expiration: chrono::DateTime<chrono::Utc>,
    pub option_type: OptionType,
}

impl From<StrategyLeg> for StrategyLegResponse {
    fn from(leg: StrategyLeg) -> Self {
        Self {
            id: leg.id,
            strategy_id: leg.strategy_id,
            option_contract_id: leg.option_contract_id,
            quantity: leg.quantity,
            position_type: leg.position_type,
            premium: leg.premium,
            strike: leg.strike,
            expiration: leg.expiration,
            option_type: leg.option_type,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchChainParams {
    pub symbol: String,
    pub workspace_id: String,
    pub provider: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
) -> Result<OptionChainResponse, AppError> {
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
        .map(Into::into)
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
#[serde(rename_all = "camelCase")]
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
// File-based Option Chain Import
// ============================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportOptionChainParams {
    pub workspace_id: String,
    /// Absolute path to the `.csv` or `.json` file selected by the user.
    pub file_path: String,
}

/// Import an option chain from a validated local file.
///
/// Rust owns the file read, schema validation, and persistence. React only
/// supplies the workspace ID and the selected file path. The endpoint limits
/// file size (10 MB), rejects path traversal, and reports partial imports.
#[tauri::command(rename_all = "camelCase")]
pub async fn import_option_chain_file(
    params: ImportOptionChainParams,
    state: State<'_, AppState>,
) -> Result<OptionChainResponse, AppError> {
    if params.workspace_id.trim().is_empty() {
        return Err(AppError::Validation(
            "Workspace ID cannot be empty".to_string(),
        ));
    }
    let file_path = std::path::Path::new(&params.file_path);

    let import = crate::providers::market_data::file_provider::import_option_chain_file(
        file_path,
        &params.workspace_id,
    )
    .map_err(|e| match e {
        AppError::Validation(_) | AppError::NotFound(_) => e,
        e => AppError::Internal(format!("File import failed: {}", e)),
    })?;

    let now = chrono::Utc::now();
    let chain = OptionChain {
        id: uuid::Uuid::new_v4().to_string(),
        workspace_id: params.workspace_id.clone(),
        symbol: import.symbol,
        underlying_price: import.underlying_price,
        as_of: now,
        data_source: DataSource::File,
        created_at: now,
    };

    state
        .option_service
        .persist_file_chain(
            chain.clone(),
            &import.contracts,
            import.rejected_count,
            import.rejection_detail,
        )
        .await?;

    Ok(chain.into())
}

// ============================================
// Option Chain CRUD Commands
// ============================================

/// Get an option chain by ID
#[tauri::command(rename_all = "camelCase")]
pub async fn get_option_chain(
    id: String,
    state: State<'_, AppState>,
) -> Result<OptionChainResponse, AppError> {
    state.option_service.get_chain(&id).await.map(Into::into)
}

/// List all option chains for a workspace
#[tauri::command(rename_all = "camelCase")]
pub async fn list_option_chains(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<OptionChainResponse>, AppError> {
    state
        .option_service
        .list_chains(&workspace_id)
        .await
        .map(|chains| chains.into_iter().map(Into::into).collect())
}

/// Delete an option chain
#[tauri::command(rename_all = "camelCase")]
pub async fn delete_option_chain(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    state.option_service.delete_chain(&id).await
}

// ============================================
// Option Contract CRUD Commands
// ============================================

/// Create a new option contract
#[tauri::command]
pub async fn create_option_contract(
    params: CreateContractParams,
    state: State<'_, AppState>,
) -> Result<OptionContractResponse, AppError> {
    let contract = domain::option::OptionContract {
        id: uuid::Uuid::new_v4().to_string(),
        workspace_id: params.workspace_id,
        chain_id: params.chain_id,
        symbol: params.symbol,
        option_type: match params.option_type.as_str() {
            "call" => OptionType::Call,
            "put" => OptionType::Put,
            _ => return Err(AppError::Validation("Invalid option type".to_string())),
        },
        strike: params.strike,
        expiration: params.expiration,
        contract_multiplier: params.contract_multiplier.unwrap_or(100),
        bid: params.bid.unwrap_or(0.0),
        ask: params.ask.unwrap_or(0.0),
        last: params.last,
        volume: params.volume.unwrap_or(0),
        open_interest: params.open_interest.unwrap_or(0),
        implied_volatility: params.implied_volatility.unwrap_or(0.0),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    state.option_service.create_contract(&contract).await?;
    Ok(contract.into())
}

/// Get an option contract by ID
#[tauri::command(rename_all = "camelCase")]
pub async fn get_option_contract(
    id: String,
    state: State<'_, AppState>,
) -> Result<OptionContractResponse, AppError> {
    state.option_service.get_contract(&id).await.map(Into::into)
}

/// List all option contracts for a chain
#[tauri::command(rename_all = "camelCase")]
pub async fn list_option_contracts(
    chain_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<OptionContractResponse>, AppError> {
    state
        .option_service
        .list_contracts(&chain_id)
        .await
        .map(|contracts| contracts.into_iter().map(Into::into).collect())
}

/// Delete an option contract
#[tauri::command(rename_all = "camelCase")]
pub async fn delete_option_contract(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.option_service.delete_contract(&id).await
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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
) -> Result<OptionStrategyResponse, AppError> {
    state
        .strategy_service
        .create_strategy(CreateStrategyInput {
            workspace_id: params.workspace_id,
            name: params.name,
            strategy_type: params.strategy_type,
            legs: params
                .legs
                .into_iter()
                .map(|leg| CreateStrategyLegInput {
                    contract_id: leg.contract_id,
                    quantity: leg.quantity,
                    position_type: leg.position_type,
                })
                .collect(),
        })
        .await
        .map(Into::into)
}

/// Get an option strategy by ID
#[tauri::command(rename_all = "camelCase")]
pub async fn get_option_strategy(
    id: String,
    state: State<'_, AppState>,
) -> Result<OptionStrategyResponse, AppError> {
    state
        .strategy_service
        .get_strategy(&id)
        .await
        .map(Into::into)
}

/// List all option strategies for a workspace
#[tauri::command(rename_all = "camelCase")]
pub async fn list_option_strategies(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<OptionStrategyResponse>, AppError> {
    state
        .strategy_service
        .list_strategies(&workspace_id)
        .await
        .map(|strategies| strategies.into_iter().map(Into::into).collect())
}

/// Update an option strategy
#[tauri::command]
pub async fn update_option_strategy(
    params: UpdateStrategyParams,
    state: State<'_, AppState>,
) -> Result<OptionStrategyResponse, AppError> {
    let mut details = state.strategy_service.get_strategy(&params.id).await?;

    if let Some(name) = params.name {
        details.strategy.name = name;
    }
    details.strategy.updated_at = chrono::Utc::now();

    state
        .option_service
        .update_strategy(&details.strategy)
        .await?;
    state
        .strategy_service
        .get_strategy(&details.strategy.id)
        .await
        .map(Into::into)
}

/// Delete an option strategy
#[tauri::command(rename_all = "camelCase")]
pub async fn delete_option_strategy(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state.strategy_service.delete_strategy(&id).await
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStrategyParams {
    pub workspace_id: String,
    pub name: String,
    pub strategy_type: StrategyType,
    pub legs: Vec<CreateStrategyLegParams>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStrategyLegParams {
    pub contract_id: String,
    pub quantity: i32,
    pub position_type: PositionType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStrategyParams {
    pub id: String,
    pub name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use domain::option::OptionStrategy;

    fn timestamp() -> chrono::DateTime<Utc> {
        Utc.with_ymd_and_hms(2024, 1, 2, 3, 4, 5)
            .single()
            .expect("valid fixture timestamp")
    }

    #[test]
    fn response_dtos_serialize_camel_case_without_changing_domain_models() {
        let chain = OptionChainResponse::from(OptionChain {
            id: "chain-1".into(),
            workspace_id: "workspace-1".into(),
            symbol: "AAPL".into(),
            underlying_price: 150.0,
            as_of: timestamp(),
            data_source: DataSource::Demo,
            created_at: timestamp(),
        });
        let contract = OptionContractResponse::from(OptionContract {
            id: "contract-1".into(),
            workspace_id: "workspace-1".into(),
            chain_id: "chain-1".into(),
            symbol: "AAPL".into(),
            option_type: OptionType::Call,
            strike: 150.0,
            expiration: timestamp(),
            contract_multiplier: 100,
            bid: 4.0,
            ask: 5.0,
            last: None,
            volume: 10,
            open_interest: 20,
            implied_volatility: 0.25,
            created_at: timestamp(),
            updated_at: timestamp(),
        });
        let strategy = OptionStrategyResponse::from(StrategyWithLegs {
            strategy: OptionStrategy {
                id: "strategy-1".into(),
                workspace_id: "workspace-1".into(),
                name: "Demo spread".into(),
                strategy_type: StrategyType::BullCallSpread,
                underlying: "AAPL".into(),
                total_cost: 100.0,
                max_profit: None,
                max_loss: Some(-100.0),
                break_even_points: vec![151.0],
                created_at: timestamp(),
                updated_at: timestamp(),
            },
            legs: Vec::new(),
        });

        let value = serde_json::json!({
            "chain": chain,
            "contract": contract,
            "strategy": strategy,
        });
        assert_eq!(value["chain"]["workspaceId"], "workspace-1");
        assert_eq!(value["chain"]["underlyingPrice"], 150.0);
        assert!(value["chain"].get("workspace_id").is_none());
        assert_eq!(value["contract"]["chainId"], "chain-1");
        assert!(value["contract"].get("chain_id").is_none());
        assert_eq!(value["strategy"]["strategyType"], "bull_call_spread");
        assert!(value["strategy"].get("maxProfit").is_some());
        assert!(value["strategy"].get("max_profit").is_none());
        assert!(value["strategy"]["maxProfit"].is_null());
        assert!(value["strategy"]["legs"].is_array());
    }

    #[test]
    fn request_dtos_accept_camel_case_and_reject_snake_case() {
        let params: FetchChainParams = serde_json::from_value(serde_json::json!({
            "symbol": "AAPL",
            "workspaceId": "workspace-1",
            "provider": "demo",
        }))
        .expect("camelCase request fixture should deserialize");
        assert_eq!(params.workspace_id, "workspace-1");

        let error = serde_json::from_value::<FetchChainParams>(serde_json::json!({
            "symbol": "AAPL",
            "workspace_id": "workspace-1",
        }))
        .expect_err("snake_case request fields must not be accepted");
        assert!(error.to_string().contains("workspaceId"));

        let strategy: CreateStrategyParams = serde_json::from_value(serde_json::json!({
            "workspaceId": "00000000-0000-4000-8000-000000000001",
            "name": "Call spread",
            "strategyType": "bull_call_spread",
            "legs": [{
                "contractId": "00000000-0000-4000-8000-000000000002",
                "quantity": 1,
                "positionType": "long"
            }]
        }))
        .expect("strategy request fixture should deserialize");
        assert_eq!(strategy.legs.len(), 1);
        assert_eq!(strategy.legs[0].position_type, PositionType::Long);
    }
}
