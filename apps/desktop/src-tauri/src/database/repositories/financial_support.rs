// Shared row-parsing helpers for the financial repositories.
//
// Financial migrations (0015-0021) store money/quantity as TEXT decimal
// strings, dates as YYYY-MM-DD, and timestamps as RFC3339 UTC. These helpers
// convert raw DB rows into domain types with typed errors — no panics —
// following the pattern in `portfolio_repository.rs::parse_timestamp`.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;

use crate::error::AppError;

pub(crate) fn parse_decimal(value: &str, field: &str) -> Result<Decimal, AppError> {
    value
        .parse::<Decimal>()
        .map_err(|_| AppError::Internal(format!("invalid {field} decimal in database")))
}

pub(crate) fn parse_optional_decimal(
    value: Option<String>,
    field: &str,
) -> Result<Option<Decimal>, AppError> {
    value.map(|text| parse_decimal(&text, field)).transpose()
}

pub(crate) fn parse_date(value: &str, field: &str) -> Result<NaiveDate, AppError> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map_err(|_| AppError::Internal(format!("invalid {field} date in database")))
}

pub(crate) fn parse_optional_date(
    value: Option<String>,
    field: &str,
) -> Result<Option<NaiveDate>, AppError> {
    value.map(|text| parse_date(&text, field)).transpose()
}

pub(crate) fn parse_timestamp(value: &str, field: &str) -> Result<DateTime<Utc>, AppError> {
    DateTime::parse_from_rfc3339(value)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .map_err(|_| AppError::Internal(format!("invalid {field} timestamp in database")))
}

pub(crate) fn parse_json(
    value: Option<String>,
    field: &str,
) -> Result<Option<serde_json::Value>, AppError> {
    value
        .map(|text| {
            serde_json::from_str(&text)
                .map_err(|_| AppError::Internal(format!("invalid {field} json in database")))
        })
        .transpose()
}
