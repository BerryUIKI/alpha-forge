use chrono::NaiveDate;
use rust_decimal::Decimal;

/// A cash dividend event from a market data provider.
#[derive(Debug, Clone)]
pub struct DividendEvent {
    pub date: NaiveDate,
    pub amount: Decimal,
    pub currency: Option<String>,
}
