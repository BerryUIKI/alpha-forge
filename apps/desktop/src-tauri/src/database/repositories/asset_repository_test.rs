// Tests for the asset + quote financial repositories.

use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::database::repositories::asset_repository::{AssetRepository, QuoteRepository};
use crate::database::repositories::test_support::setup_test_db;
use domain::financial::{AssetKind, CreateAssetInput, InstrumentType, QuoteMode, UpsertQuoteInput};

async fn create_equity_asset(repo: &AssetRepository) -> String {
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
async fn asset_repository_derives_instrument_key() {
    let pool = setup_test_db().await;
    let repo = AssetRepository::new(pool);

    let asset_id = create_equity_asset(&repo).await;

    let fetched = repo.get(&asset_id).await.expect("Failed to get asset");
    let asset = fetched.expect("Asset should exist");
    assert_eq!(asset.instrument_key.as_deref(), Some("EQUITY:AAPL@XNAS"));
    assert_eq!(asset.kind, AssetKind::Investment);
    assert_eq!(asset.quote_mode, QuoteMode::Market);
    assert!(asset.is_active);
}

#[tokio::test]
async fn asset_repository_finds_by_instrument_key() {
    let pool = setup_test_db().await;
    let repo = AssetRepository::new(pool);

    let asset_id = create_equity_asset(&repo).await;

    let found = repo
        .find_by_instrument_key("EQUITY:AAPL@XNAS")
        .await
        .expect("Failed to find asset by key")
        .expect("Asset should be found by instrument key");
    assert_eq!(found.id, asset_id);

    let missing = repo
        .find_by_instrument_key("EQUITY:MSFT@XNAS")
        .await
        .expect("Failed to query missing key");
    assert!(missing.is_none());
}

#[tokio::test]
async fn asset_repository_rejects_duplicate_instrument_key() {
    let pool = setup_test_db().await;
    let repo = AssetRepository::new(pool);

    create_equity_asset(&repo).await;

    let duplicate = repo
        .create(CreateAssetInput {
            kind: AssetKind::Investment,
            name: Some("Apple Inc (again)".to_string()),
            display_code: Some("AAPL2".to_string()),
            notes: None,
            is_active: true,
            quote_mode: QuoteMode::Market,
            quote_ccy: "USD".to_string(),
            instrument_type: Some(InstrumentType::Equity),
            instrument_symbol: Some("AAPL".to_string()),
            instrument_exchange_mic: Some("XNAS".to_string()),
            provider_config: None,
        })
        .await;
    assert!(
        duplicate.is_err(),
        "duplicate instrument_key must be rejected by the unique index"
    );
}

#[tokio::test]
async fn quote_repository_upserts_per_asset_day_source() {
    let pool = setup_test_db().await;
    let assets = AssetRepository::new(pool.clone());
    let quotes = QuoteRepository::new(pool);

    let asset_id = create_equity_asset(&assets).await;
    let day = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
    let close = Decimal::from_str_exact("241.53").expect("valid decimal");

    let quote = quotes
        .upsert(UpsertQuoteInput {
            asset_id: asset_id.clone(),
            day,
            source: "MARKET".to_string(),
            open: None,
            high: None,
            low: None,
            close,
            adjclose: Some(Decimal::from_str_exact("241.53").expect("valid decimal")),
            volume: Some(Decimal::from_str_exact("52341200").expect("valid decimal")),
            currency: "USD".to_string(),
            notes: None,
        })
        .await
        .expect("Failed to upsert quote");
    assert_eq!(quote.close, close);
    assert_eq!(quote.day, day);

    // Upserting the same (asset, day, source) updates rather than duplicating.
    let updated_close = Decimal::from_str_exact("242.10").expect("valid decimal");
    quotes
        .upsert(UpsertQuoteInput {
            asset_id: asset_id.clone(),
            day,
            source: "MARKET".to_string(),
            open: None,
            high: None,
            low: None,
            close: updated_close,
            adjclose: None,
            volume: None,
            currency: "USD".to_string(),
            notes: Some("manual fix".to_string()),
        })
        .await
        .expect("Failed to update quote");

    let listed = quotes
        .list_for_asset(&asset_id)
        .await
        .expect("Failed to list quotes");
    assert_eq!(listed.len(), 1, "upsert must not duplicate rows");
    assert_eq!(listed[0].close, updated_close);
    assert_eq!(listed[0].notes.as_deref(), Some("manual fix"));
}
