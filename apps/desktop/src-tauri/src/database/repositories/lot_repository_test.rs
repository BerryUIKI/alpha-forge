// Tests for the lot + lot-disposal financial repositories.

use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::database::repositories::account_repository::AccountRepository;
use crate::database::repositories::activity_repository::ActivityRepository;
use crate::database::repositories::asset_repository::AssetRepository;
use crate::database::repositories::lot_repository::{LotDisposalRepository, LotRepository};
use crate::database::repositories::test_support::setup_test_db;
use domain::financial::{
    AccountType, ActivityStatus, ActivityType, AssetKind, CostBasisMethod, CreateAccountInput,
    CreateActivityInput, CreateAssetInput, CreateLotDisposalInput, CreateLotInput, InstrumentType,
    QuoteMode, TrackingMode,
};

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
            tracking_mode: TrackingMode::Transactions,
        })
        .await
        .expect("Failed to create account");
    account.id
}

/// Creates a sell activity that a disposal row references via its
/// `disposal_activity_id` FK, mirroring how the real services persist disposals.
async fn create_sell_activity(
    pool: &sqlx::SqlitePool,
    account_id: &str,
    asset_id: &str,
    quantity: &str,
    proceeds: &str,
) -> String {
    let repo = ActivityRepository::new(pool.clone());
    let activity = repo
        .create(CreateActivityInput {
            account_id: account_id.to_string(),
            asset_id: Some(asset_id.to_string()),
            activity_type: ActivityType::Sell,
            activity_type_override: None,
            source_type: Some("TRADE".to_string()),
            subtype: None,
            status: ActivityStatus::Posted,
            activity_date: NaiveDate::from_ymd_opt(2026, 8, 20).expect("valid date"),
            settlement_date: Some(NaiveDate::from_ymd_opt(2026, 8, 20).expect("valid date")),
            quantity: Some(dec(quantity)),
            unit_price: Some(dec(proceeds) / dec(quantity)),
            amount: Some(dec(proceeds)),
            fee: Some(dec("0")),
            tax: None,
            currency: "USD".to_string(),
            fx_rate: None,
            notes: None,
            metadata: None,
            source_system: Some("manual".to_string()),
            source_record_id: None,
            source_group_id: None,
            idempotency_key: Some(format!("manual:{}:sell:{}", account_id, asset_id)),
            import_run_id: None,
        })
        .await
        .expect("Failed to create sell activity");
    activity.id
}

async fn create_asset(pool: &sqlx::SqlitePool) -> String {
    let repo = AssetRepository::new(pool.clone());
    let asset = repo
        .create(CreateAssetInput {
            kind: AssetKind::Investment,
            name: Some("Apple Inc".to_string()),
            display_code: Some("AAPL".to_string()),
            notes: None,
            is_active: true,
            quote_mode: QuoteMode::Market,
            quote_ccy: "USD".to_string(),
            instrument_type: Some(InstrumentType::Equity),
            instrument_symbol: Some("AAPL".to_string()),
            instrument_exchange_mic: Some("XNAS".to_string()),
            provider_config: None,
        })
        .await
        .expect("Failed to create asset");
    asset.id
}

fn dec(value: &str) -> Decimal {
    Decimal::from_str_exact(value).expect("valid decimal")
}

fn lot_input(
    account_id: &str,
    asset_id: &str,
    quantity: &str,
    cost_per_unit: &str,
) -> CreateLotInput {
    CreateLotInput {
        account_id: account_id.to_string(),
        asset_id: asset_id.to_string(),
        open_date: NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date"),
        open_activity_id: None,
        original_quantity: dec(quantity),
        cost_per_unit: dec(cost_per_unit),
        original_cost_basis: dec(cost_per_unit) * dec(quantity),
        fee_allocated: dec("0"),
        currency: "USD".to_string(),
        base_currency: "USD".to_string(),
        fx_rate_to_base: dec("1"),
        fx_rate_to_account: None,
        account_currency: None,
        cost_basis_method: CostBasisMethod::Fifo,
    }
}

#[tokio::test]
async fn lot_repository_creates_open_lot_with_mirrored_cost_basis() {
    let pool = setup_test_db().await;
    let account_id = create_account(&pool, "acct-lot").await;
    let asset_id = create_asset(&pool).await;
    let repo = LotRepository::new(pool.clone());

    let lot = repo
        .create(lot_input(&account_id, &asset_id, "10", "241.53"))
        .await
        .expect("Failed to create lot");

    assert_eq!(lot.original_quantity, dec("10"));
    assert_eq!(lot.cost_per_unit, dec("241.53"));
    // The repository mirrors original cost basis into remaining on open.
    assert_eq!(lot.original_cost_basis, dec("2415.30"));
    assert_eq!(lot.remaining_cost_basis, dec("2415.30"));
    assert_eq!(lot.remaining_quantity, dec("10"));
    assert_eq!(lot.cost_basis_method, CostBasisMethod::Fifo);
    assert!(!lot.is_closed);

    let open = repo
        .list_open_by_account_asset(&account_id, &asset_id)
        .await
        .expect("Failed to list open lots");
    assert_eq!(open.len(), 1);
    assert_eq!(open[0].id, lot.id);
}

#[tokio::test]
async fn lot_disposal_repository_records_realized_pnl() {
    let pool = setup_test_db().await;
    let account_id = create_account(&pool, "acct-dis").await;
    let asset_id = create_asset(&pool).await;
    let lot_repo = LotRepository::new(pool.clone());
    let disposal_repo = LotDisposalRepository::new(pool.clone());

    let lot = lot_repo
        .create(lot_input(&account_id, &asset_id, "10", "100"))
        .await
        .expect("Failed to create lot");

    // The disposal row's `disposal_activity_id` FK must point at a real sell
    // activity, exactly as the services persist disposals.
    let sell_activity_id = create_sell_activity(&pool, &account_id, &asset_id, "4", "480").await;

    let disposal = disposal_repo
        .create(CreateLotDisposalInput {
            lot_id: lot.id.clone(),
            account_id: account_id.clone(),
            asset_id: asset_id.clone(),
            disposal_activity_id: sell_activity_id,
            disposal_date: NaiveDate::from_ymd_opt(2026, 8, 20).expect("valid date"),
            quantity: dec("4"),
            proceeds: dec("480"),
            cost_basis: dec("400"),
            realized_pnl: dec("80"),
            proceeds_base: dec("480"),
            cost_basis_base: dec("400"),
            realized_pnl_base: dec("80"),
            currency: "USD".to_string(),
            base_currency: "USD".to_string(),
            fx_rate_to_base: dec("1"),
            cost_basis_method: CostBasisMethod::Fifo,
        })
        .await
        .expect("Failed to create disposal");

    assert_eq!(disposal.lot_id, lot.id);
    assert_eq!(disposal.realized_pnl, dec("80"));

    let listed = disposal_repo
        .list_by_account(&account_id)
        .await
        .expect("Failed to list disposals");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].quantity, dec("4"));
}
