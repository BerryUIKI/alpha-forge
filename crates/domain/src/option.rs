// Option domain models.
//
// Contains core entities for the Option Analysis Platform:
// OptionChain, OptionContract, Greeks, OptionStrategy, StrategyLeg, OptionPosition.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Type of option contract
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OptionType {
    Call,
    Put,
}

impl std::fmt::Display for OptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionType::Call => write!(f, "call"),
            OptionType::Put => write!(f, "put"),
        }
    }
}

/// Data source for option chain
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataSource {
    Live,
    Demo,
    File,
}

impl std::fmt::Display for DataSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataSource::Live => write!(f, "live"),
            DataSource::Demo => write!(f, "demo"),
            DataSource::File => write!(f, "file"),
        }
    }
}

/// Type of strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StrategyType {
    LongCall,
    LongPut,
    CoveredCall,
    ProtectivePut,
    BullCallSpread,
    BearPutSpread,
    Straddle,
    Strangle,
    IronCondor,
    Butterfly,
    Custom,
}

impl std::fmt::Display for StrategyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StrategyType::LongCall => write!(f, "long_call"),
            StrategyType::LongPut => write!(f, "long_put"),
            StrategyType::CoveredCall => write!(f, "covered_call"),
            StrategyType::ProtectivePut => write!(f, "protective_put"),
            StrategyType::BullCallSpread => write!(f, "bull_call_spread"),
            StrategyType::BearPutSpread => write!(f, "bear_put_spread"),
            StrategyType::Straddle => write!(f, "straddle"),
            StrategyType::Strangle => write!(f, "strangle"),
            StrategyType::IronCondor => write!(f, "iron_condor"),
            StrategyType::Butterfly => write!(f, "butterfly"),
            StrategyType::Custom => write!(f, "custom"),
        }
    }
}

/// Position type (long or short)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PositionType {
    Long,
    Short,
}

impl std::fmt::Display for PositionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PositionType::Long => write!(f, "long"),
            PositionType::Short => write!(f, "short"),
        }
    }
}

/// Pricing model used for calculations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PricingModel {
    BlackScholes,
    Binomial,
    FiniteDifference,
}

impl std::fmt::Display for PricingModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PricingModel::BlackScholes => write!(f, "black_scholes"),
            PricingModel::Binomial => write!(f, "binomial"),
            PricingModel::FiniteDifference => write!(f, "finite_difference"),
        }
    }
}

/// Option chain - represents all option contracts for a symbol at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionChain {
    pub id: String,
    pub workspace_id: String,
    pub symbol: String,
    pub underlying_price: f64,
    pub as_of: DateTime<Utc>,
    pub data_source: DataSource,
    pub created_at: DateTime<Utc>,
}

/// Option contract - a single option within a chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionContract {
    pub id: String,
    pub workspace_id: String,
    pub chain_id: String,
    pub symbol: String,
    pub option_type: OptionType,
    pub strike: f64,
    pub expiration: DateTime<Utc>,
    pub contract_multiplier: u32,
    pub bid: f64,
    pub ask: f64,
    pub last: Option<f64>,
    pub volume: u64,
    pub open_interest: u64,
    pub implied_volatility: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Greeks - risk sensitivities for an option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Greeks {
    pub id: String,
    pub option_contract_id: String,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
    pub iv: f64,
    pub calculated_at: DateTime<Utc>,
    pub calculation_model: PricingModel,
}

/// Option strategy - a multi-leg option position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionStrategy {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub strategy_type: StrategyType,
    pub underlying: String,
    pub total_cost: f64,
    pub max_profit: Option<f64>,
    pub max_loss: Option<f64>,
    pub break_even_points: Vec<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Strategy leg - a single option within a strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyLeg {
    pub id: String,
    pub strategy_id: String,
    pub option_contract_id: String,
    pub quantity: i32,
    pub position_type: PositionType,
    pub premium: f64,
    pub strike: f64,
    pub expiration: DateTime<Utc>,
    pub option_type: OptionType,
}

/// Option position - an actual position held in portfolio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionPosition {
    pub id: String,
    pub workspace_id: String,
    pub account_id: Option<String>,
    pub option_contract_id: String,
    pub quantity: i32,
    pub cost_basis: f64,
    pub opened_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

/// Greeks snapshot - historical Greeks tracking for positions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreeksSnapshot {
    pub id: String,
    pub workspace_id: String,
    pub position_id: String,
    pub snapshot_date: DateTime<Utc>,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_type_serialization() {
        let call = OptionType::Call;
        let json = serde_json::to_string(&call).unwrap();
        assert_eq!(json, r#""call""#);

        let parsed: OptionType = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, OptionType::Call);
    }

    #[test]
    fn test_data_source_display() {
        assert_eq!(format!("{}", DataSource::Live), "live");
        assert_eq!(format!("{}", DataSource::Demo), "demo");
        assert_eq!(format!("{}", DataSource::File), "file");
    }

    #[test]
    fn test_strategy_type_serialization() {
        let strategy = StrategyType::BullCallSpread;
        let json = serde_json::to_string(&strategy).unwrap();
        assert_eq!(json, r#""bull_call_spread""#);
    }

    #[test]
    fn test_option_chain_creation() {
        let chain = OptionChain {
            id: "chain-123".to_string(),
            workspace_id: "ws-1".to_string(),
            symbol: "AAPL".to_string(),
            underlying_price: 150.0,
            as_of: Utc::now(),
            data_source: DataSource::Demo,
            created_at: Utc::now(),
        };

        assert_eq!(chain.symbol, "AAPL");
        assert_eq!(chain.underlying_price, 150.0);
    }

    #[test]
    fn test_option_contract_creation() {
        let contract = OptionContract {
            id: "contract-123".to_string(),
            workspace_id: "ws-1".to_string(),
            chain_id: "chain-123".to_string(),
            symbol: "AAPL".to_string(),
            option_type: OptionType::Call,
            strike: 150.0,
            expiration: Utc::now(),
            contract_multiplier: 100,
            bid: 5.20,
            ask: 5.30,
            last: Some(5.25),
            volume: 1000,
            open_interest: 5000,
            implied_volatility: 0.25,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(contract.option_type, OptionType::Call);
        assert_eq!(contract.strike, 150.0);
    }

    #[test]
    fn test_greeks_creation() {
        let greeks = Greeks {
            id: "greeks-123".to_string(),
            option_contract_id: "contract-123".to_string(),
            delta: 0.52,
            gamma: 0.08,
            theta: -0.05,
            vega: 0.15,
            rho: 0.02,
            iv: 0.25,
            calculated_at: Utc::now(),
            calculation_model: PricingModel::BlackScholes,
        };

        assert!((greeks.delta - 0.52).abs() < 0.001);
        assert!((greeks.gamma - 0.08).abs() < 0.001);
    }

    #[test]
    fn test_option_strategy_creation() {
        let strategy = OptionStrategy {
            id: "strategy-123".to_string(),
            workspace_id: "ws-1".to_string(),
            name: "Bull Call Spread".to_string(),
            strategy_type: StrategyType::BullCallSpread,
            underlying: "AAPL".to_string(),
            total_cost: -250.0,
            max_profit: Some(250.0),
            max_loss: Some(-250.0),
            break_even_points: vec![152.5],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(strategy.strategy_type, StrategyType::BullCallSpread);
        assert_eq!(strategy.break_even_points.len(), 1);
    }

    #[test]
    fn test_strategy_leg_creation() {
        let leg = StrategyLeg {
            id: "leg-123".to_string(),
            strategy_id: "strategy-123".to_string(),
            option_contract_id: "contract-123".to_string(),
            quantity: 1,
            position_type: PositionType::Long,
            premium: 5.20,
            strike: 150.0,
            expiration: Utc::now(),
            option_type: OptionType::Call,
        };

        assert_eq!(leg.quantity, 1);
        assert_eq!(leg.position_type, PositionType::Long);
    }

    #[test]
    fn test_option_position_creation() {
        let position = OptionPosition {
            id: "position-123".to_string(),
            workspace_id: "ws-1".to_string(),
            account_id: Some("account-123".to_string()),
            option_contract_id: "contract-123".to_string(),
            quantity: 2,
            cost_basis: 1040.0,
            opened_at: Utc::now(),
            closed_at: None,
            notes: Some("Test position".to_string()),
        };

        assert_eq!(position.quantity, 2);
        assert!(position.closed_at.is_none());
    }

    #[test]
    fn test_greeks_snapshot_creation() {
        let snapshot = GreeksSnapshot {
            id: "snapshot-123".to_string(),
            workspace_id: "ws-1".to_string(),
            position_id: "position-123".to_string(),
            snapshot_date: Utc::now(),
            delta: 0.50,
            gamma: 0.08,
            theta: -0.05,
            vega: 0.15,
            rho: 0.02,
            created_at: Utc::now(),
        };

        assert!((snapshot.delta - 0.50).abs() < 0.001);
    }
}
