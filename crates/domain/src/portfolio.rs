// Portfolio domain models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioAccount {
    pub id: String,
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
}
