// Tests for the import-run + activity financial repositories.

use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::database::repositories::account_repository::AccountRepository;
use crate::database::repositories::activity_repository::{ActivityRepository, ImportRunRepository};
use crate::database::repositories::asset_repository::AssetRepository;
use crate::database::repositories::test_support::setup_test_db;
use domain::financial::{
    AccountType, ActivityStatus, ActivityType, AssetKind, CreateAccountInput, CreateActivityInput,
    CreateAssetInput, CreateImportRunInput, InstrumentType, QuoteMode,
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
            tracking_mode: domain::financial::TrackingMode::Transactions,
        })
        .await
        .expect("Failed to create account");
    account.id
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

#[tokio::test]
async fn import_run_repository_creates_and_lists() {
    let pool = setup_test_db().await;
    let account_id = create_account(&pool, "acct-import").await;
    let repo = ImportRunRepository::new(pool.clone());

    let run = repo
        .create(CreateImportRunInput {
            account_id: account_id.clone(),
            source_system: "csv".to_string(),
            run_type: "activities".to_string(),
            mode: "new".to_string(),
            status: "in_progress".to_string(),
            review_mode: "off".to_string(),
        })
        .await
        .expect("Failed to create import run");

    assert_eq!(run.source_system, "csv");
    assert_eq!(run.status, "in_progress");
    assert!(run.finished_at.is_none());

    let listed = repo
        .list_by_account(&account_id)
        .await
        .expect("Failed to list import runs");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, run.id);
}

#[tokio::test]
async fn activity_repository_round_trips_trade() {
    let pool = setup_test_db().await;
    let account_id = create_account(&pool, "acct-act").await;
    let asset_id = create_asset(&pool).await;
    let repo = ActivityRepository::new(pool.clone());

    let day = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
    let activity = repo
        .create(CreateActivityInput {
            account_id: account_id.clone(),
            asset_id: Some(asset_id.clone()),
            activity_type: ActivityType::Buy,
            activity_type_override: None,
            source_type: Some("TRADE".to_string()),
            subtype: None,
            status: ActivityStatus::Posted,
            activity_date: day,
            settlement_date: Some(day),
            quantity: Some(Decimal::from_str_exact("10").expect("valid decimal")),
            unit_price: Some(Decimal::from_str_exact("241.53").expect("valid decimal")),
            amount: Some(Decimal::from_str_exact("-2415.30").expect("valid decimal")),
            fee: Some(Decimal::from_str_exact("1.00").expect("valid decimal")),
            tax: None,
            currency: "USD".to_string(),
            fx_rate: None,
            notes: Some("Initial buy".to_string()),
            metadata: None,
            source_system: Some("manual".to_string()),
            source_record_id: None,
            source_group_id: None,
            idempotency_key: Some("manual:acct-act:1".to_string()),
            import_run_id: None,
        })
        .await
        .expect("Failed to create activity");

    assert_eq!(activity.activity_type, ActivityType::Buy);
    assert_eq!(activity.status, ActivityStatus::Posted);
    assert_eq!(activity.activity_date, day);
    assert_eq!(
        activity.quantity,
        Some(Decimal::from_str_exact("10").expect("valid decimal"))
    );
    assert!(!activity.is_user_modified);
    assert!(!activity.needs_review);

    let listed = repo
        .list_by_account(&account_id)
        .await
        .expect("Failed to list activities");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, activity.id);

    let by_asset = repo
        .list_by_asset(&asset_id)
        .await
        .expect("Failed to list asset activities");
    assert_eq!(by_asset.len(), 1);
}

#[tokio::test]
async fn activity_repository_rejects_duplicate_idempotency_key() {
    let pool = setup_test_db().await;
    let account_id = create_account(&pool, "acct-idem").await;
    let asset_id = create_asset(&pool).await;
    let repo = ActivityRepository::new(pool.clone());

    let day = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
    let input = CreateActivityInput {
        account_id: account_id.clone(),
        asset_id: Some(asset_id),
        activity_type: ActivityType::Buy,
        activity_type_override: None,
        source_type: None,
        subtype: None,
        status: ActivityStatus::Posted,
        activity_date: day,
        settlement_date: None,
        quantity: Some(Decimal::from_str_exact("1").expect("valid decimal")),
        unit_price: Some(Decimal::from_str_exact("100").expect("valid decimal")),
        amount: Some(Decimal::from_str_exact("-100").expect("valid decimal")),
        fee: None,
        tax: None,
        currency: "USD".to_string(),
        fx_rate: None,
        notes: None,
        metadata: None,
        source_system: Some("csv".to_string()),
        source_record_id: Some("row-1".to_string()),
        source_group_id: None,
        idempotency_key: Some("csv:row-1".to_string()),
        import_run_id: None,
    };

    repo.create(input.clone())
        .await
        .expect("First insert should succeed");

    let duplicate = repo.create(input).await;
    assert!(
        duplicate.is_err(),
        "duplicate idempotency key must be rejected"
    );
}
