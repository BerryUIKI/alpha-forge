// Tests for the holdings-snapshot financial repository.

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde_json::json;

use crate::database::repositories::account_repository::AccountRepository;
use crate::database::repositories::asset_repository::AssetRepository;
use crate::database::repositories::snapshot_repository::SnapshotRepository;
use crate::database::repositories::test_support::setup_test_db;
use domain::financial::{
    AccountType, AssetKind, CreateAccountInput, CreateAssetInput, CreateSnapshotInput,
    HoldingSnapshotSource, InstrumentType, QuoteMode, SnapshotPositionInput, TrackingMode,
};

fn dec(value: &str) -> Decimal {
    Decimal::from_str_exact(value).expect("valid decimal")
}

async fn create_account(pool: &sqlx::SqlitePool, name: &str) -> String {
    let repo = AccountRepository::new(pool.clone());
    let account = repo
        .create(CreateAccountInput {
            workspace_id: None,
            name: name.to_string(),
            account_type: AccountType::Securities,
            group_name: None,
            currency: "USD".to_string(),
            is_default: false,
            platform_id: None,
            account_number: None,
            tracking_mode: TrackingMode::Holdings,
        })
        .await
        .expect("Failed to create account");
    account.id
}

async fn create_asset(pool: &sqlx::SqlitePool, symbol: &str) -> String {
    let repo = AssetRepository::new(pool.clone());
    let asset = repo
        .create(CreateAssetInput {
            kind: AssetKind::Investment,
            name: Some(symbol.to_string()),
            display_code: Some(symbol.to_string()),
            notes: None,
            is_active: true,
            quote_mode: QuoteMode::Manual,
            quote_ccy: "USD".to_string(),
            instrument_type: Some(InstrumentType::Equity),
            instrument_symbol: Some(symbol.to_string()),
            instrument_exchange_mic: Some("XNYS".to_string()),
            provider_config: None,
        })
        .await
        .expect("Failed to create asset");
    asset.id
}

fn position(asset_id: &str, quantity: &str, average_cost: &str) -> SnapshotPositionInput {
    SnapshotPositionInput {
        asset_id: asset_id.to_string(),
        quantity: dec(quantity),
        average_cost: dec(average_cost),
        total_cost_basis: dec(average_cost) * dec(quantity),
        currency: "USD".to_string(),
        contract_multiplier: dec("1"),
        inception_date: NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
        is_alternative: false,
        cost_basis_base: Some(dec(average_cost) * dec(quantity)),
        cost_basis_account: Some(dec(average_cost) * dec(quantity)),
    }
}

#[tokio::test]
async fn snapshot_repository_creates_snapshot_with_positions_atomically() {
    let pool = setup_test_db().await;
    let account_id = create_account(&pool, "acct-snap").await;
    let aapl_id = create_asset(&pool, "AAPL").await;
    let msft_id = create_asset(&pool, "MSFT").await;
    let repo = SnapshotRepository::new(pool.clone());

    let snapshot = repo
        .create(CreateSnapshotInput {
            account_id: account_id.clone(),
            snapshot_date: NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date"),
            currency: "USD".to_string(),
            positions: vec![
                position(&aapl_id, "10", "241.53"),
                position(&msft_id, "5", "425.00"),
            ],
            cash_balances: json!({"USD": "1000.00"}),
            cost_basis: dec("4530.30"),
            net_contribution: dec("5000"),
            net_contribution_base: dec("5000"),
            cash_total_account_currency: dec("1000"),
            cash_total_base_currency: dec("1000"),
            source: HoldingSnapshotSource::ManualEntry,
        })
        .await
        .expect("Failed to create snapshot");

    assert_eq!(snapshot.positions.len(), 2);
    assert_eq!(snapshot.source, HoldingSnapshotSource::ManualEntry);
    assert_eq!(snapshot.positions[0].asset_id, aapl_id);
    assert_eq!(snapshot.positions[0].quantity, dec("10"));
    assert_eq!(snapshot.positions[1].asset_id, msft_id);

    let fetched = repo
        .get(&snapshot.id)
        .await
        .expect("Failed to get snapshot")
        .expect("Snapshot should exist");
    assert_eq!(fetched.positions.len(), 2);
    assert_eq!(fetched.cash_balances, json!({"USD": "1000.00"}));

    let listed = repo
        .list_by_account(&account_id)
        .await
        .expect("Failed to list snapshots");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, snapshot.id);

    // The JSON mirror column is populated for tools that read only JSON.
    let mirror: serde_json::Value =
        sqlx::query_scalar("SELECT positions FROM holdings_snapshots WHERE id = ?")
            .bind(&snapshot.id)
            .fetch_one(&pool)
            .await
            .expect("Failed to read snapshot positions JSON");
    assert_eq!(
        mirror.as_array().expect("positions must be an array").len(),
        2
    );

    repo.delete(&snapshot.id)
        .await
        .expect("Failed to delete snapshot");
    let after_delete = repo
        .get(&snapshot.id)
        .await
        .expect("Failed to get deleted snapshot");
    assert!(after_delete.is_none());
}
