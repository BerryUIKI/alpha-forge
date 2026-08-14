// Tests for the daily-account-valuation financial repository.

use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::database::repositories::account_repository::AccountRepository;
use crate::database::repositories::test_support::setup_test_db;
use crate::database::repositories::valuation_repository::ValuationRepository;
use domain::financial::{
    AccountType, BasisStatus, CreateAccountInput, ExternalFlowSource, TrackingMode,
    UpsertValuationInput, ValuationStatus,
};

fn dec(value: &str) -> Decimal {
    Decimal::from_str_exact(value).expect("valid decimal")
}

fn valuation_input(account_id: &str, day: NaiveDate) -> UpsertValuationInput {
    UpsertValuationInput {
        account_id: account_id.to_string(),
        valuation_date: day,
        account_currency: "USD".to_string(),
        base_currency: "USD".to_string(),
        fx_rate_to_base: dec("1"),
        cash_balance: dec("1000"),
        investment_market_value: dec("2415.30"),
        total_value: dec("3415.30"),
        cost_basis: dec("2415.30"),
        net_contribution: dec("3415.30"),
        cash_balance_base: dec("1000"),
        investment_market_value_base: dec("2415.30"),
        total_value_base: dec("3415.30"),
        cost_basis_base: dec("2415.30"),
        net_contribution_base: dec("3415.30"),
        external_inflow_base: dec("0"),
        external_outflow_base: dec("0"),
        performance_eligible_value_base: dec("3415.30"),
        external_flow_source: ExternalFlowSource::NoFlow,
        value_status: ValuationStatus::Complete,
        basis_status: BasisStatus::Complete,
    }
}

#[tokio::test]
async fn valuation_repository_upserts_one_row_per_account_date() {
    let pool = setup_test_db().await;
    let account_repo = AccountRepository::new(pool.clone());
    let account = account_repo
        .create(CreateAccountInput {
            workspace_id: None,
            name: "Valuation Account".to_string(),
            account_type: AccountType::Securities,
            group_name: None,
            currency: "USD".to_string(),
            is_default: false,
            platform_id: None,
            account_number: None,
            tracking_mode: TrackingMode::Transactions,
        })
        .await
        .expect("Failed to create account");
    let account_id = account.id;
    let repo = ValuationRepository::new(pool.clone());

    let day = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
    let first = repo
        .upsert(valuation_input(&account_id, day))
        .await
        .expect("Failed to upsert valuation");
    assert_eq!(first.total_value, dec("3415.30"));
    assert_eq!(first.value_status, ValuationStatus::Complete);

    // Re-running the same date must update in place, not duplicate.
    let mut updated = valuation_input(&account_id, day);
    updated.investment_market_value = dec("2500.00");
    updated.total_value = dec("3500.00");
    let second = repo
        .upsert(updated)
        .await
        .expect("Failed to re-upsert valuation");
    assert_eq!(second.total_value, dec("3500.00"));

    let rows: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM daily_account_valuation WHERE account_id = ? AND valuation_date = ?",
    )
    .bind(&account_id)
    .bind(day.to_string())
    .fetch_one(&pool)
    .await
    .expect("Failed to count valuations");
    assert_eq!(rows, 1, "upsert must not duplicate the (account, date) row");

    // A different date is a distinct row.
    let other_day = NaiveDate::from_ymd_opt(2026, 8, 14).expect("valid date");
    repo.upsert(valuation_input(&account_id, other_day))
        .await
        .expect("Failed to upsert second day");

    let listed = repo
        .list_by_account(&account_id)
        .await
        .expect("Failed to list valuations");
    assert_eq!(listed.len(), 2);

    let fetched = repo
        .get(&account_id, &day.to_string())
        .await
        .expect("Failed to get valuation")
        .expect("Valuation should exist");
    assert_eq!(fetched.total_value, dec("3500.00"));

    repo.delete_for_date(&account_id, &day.to_string())
        .await
        .expect("Failed to delete valuation");
    let gone = repo
        .get(&account_id, &day.to_string())
        .await
        .expect("Failed to get deleted valuation");
    assert!(gone.is_none());
}
