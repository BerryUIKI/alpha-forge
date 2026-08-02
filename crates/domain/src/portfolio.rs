// Portfolio domain models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioAccount {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub account_type: String,
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub id: String,
    pub account_id: String,
    pub symbol: String,
    pub quantity: f64,
    pub cost_basis: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePortfolioAccountInput {
    pub workspace_id: String,
    pub name: String,
    pub account_type: String,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePositionInput {
    pub account_id: String,
    pub symbol: String,
    pub quantity: f64,
    pub cost_basis: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioTransaction {
    pub id: String,
    pub account_id: String,
    pub symbol: String,
    pub transaction_type: TransactionType,
    pub quantity: f64,
    pub price: f64,
    pub executed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType { Buy, Sell }

impl TransactionType {
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() { "buy" => Some(Self::Buy), "sell" => Some(Self::Sell), _ => None }
    }
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { match self { Self::Buy => write!(formatter, "buy"), Self::Sell => write!(formatter, "sell") } }
}

#[derive(Debug, Clone)]
pub struct CreatePortfolioTransactionInput {
    pub account_id: String,
    pub symbol: String,
    pub transaction_type: TransactionType,
    pub quantity: f64,
    pub price: f64,
    pub executed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioAllocation {
    pub symbol: String,
    pub allocated_cost: f64,
    pub weight_percent: f64,
    pub account_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcentrationRisk {
    pub symbol: String,
    pub weight_percent: f64,
    pub severity: ConcentrationSeverity,
    pub message: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConcentrationSeverity { Moderate, High }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeExposure { pub entity_id: String, pub theme_name: String, pub allocated_cost: f64, pub weight_percent: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePortfolioThemeLinkInput { pub workspace_id: String, pub symbol: String, pub entity_id: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThesisAlignment { pub symbol: String, pub thesis_id: String, pub thesis_title: String, pub confidence: i32, pub status: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioReview { pub generated_at: DateTime<Utc>, pub concentration_risks: Vec<ConcentrationRisk>, pub unaligned_symbols: Vec<String> }
