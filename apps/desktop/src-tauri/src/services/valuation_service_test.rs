// Tests for ValuationService.
//
// Covers calculate_day, get_valuation, get_valuation_series, and calculate_all
// with empty accounts, holdings with quotes, and error cases.

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use rust_decimal::Decimal;
    use std::sync::Arc;

    use crate::database::repositories::account_repository::AccountRepository;
    use crate::database::repositories::asset_repository::{AssetRepository, QuoteRepository};
    use crate::database::repositories::lot_repository::{LotDisposalRepository, LotRepository};
    use crate::database::repositories::test_support::setup_test_db;
    use crate::database::repositories::valuation_repository::ValuationRepository;
    use crate::error::AppError;
    use crate::services::holdings_service::HoldingsService;
    use crate::services::valuation_service::ValuationService;
    use domain::financial::{
        AccountType, AssetKind, BasisStatus, CostBasisMethod, CreateAccountInput, CreateAssetInput,
        CreateLotInput, InstrumentType, QuoteMode, TrackingMode, UpsertQuoteInput, ValuationStatus,
    };

    fn dec(value: &str) -> Decimal {
        Decimal::from_str_exact(value).expect("valid decimal")
    }

    fn create_holdings_service(pool: &sqlx::SqlitePool) -> Arc<HoldingsService> {
        let account_repo = Arc::new(AccountRepository::new(pool.clone()));
        let asset_repo = Arc::new(AssetRepository::new(pool.clone()));
        let quote_repo = Arc::new(QuoteRepository::new(pool.clone()));
        let lot_repo = Arc::new(LotRepository::new(pool.clone()));
        let disposal_repo = Arc::new(LotDisposalRepository::new(pool.clone()));
        Arc::new(HoldingsService::new(
            account_repo,
            asset_repo,
            quote_repo,
            lot_repo,
            disposal_repo,
        ))
    }

    fn create_valuation_service(pool: &sqlx::SqlitePool) -> ValuationService {
        let valuation_repo = Arc::new(ValuationRepository::new(pool.clone()));
        let account_repo = Arc::new(AccountRepository::new(pool.clone()));
        let holdings_service = create_holdings_service(pool);
        ValuationService::new(valuation_repo, account_repo, holdings_service)
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
                tracking_mode: TrackingMode::Transactions,
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

    async fn create_lot(
        pool: &sqlx::SqlitePool,
        account_id: &str,
        asset_id: &str,
        quantity: &str,
        cost_per_unit: &str,
    ) -> String {
        let repo = LotRepository::new(pool.clone());
        let lot = repo
            .create(CreateLotInput {
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
            })
            .await
            .expect("Failed to create lot");
        lot.id
    }

    async fn insert_quote(pool: &sqlx::SqlitePool, asset_id: &str, date: NaiveDate, close: &str) {
        let repo = QuoteRepository::new(pool.clone());
        repo.upsert(UpsertQuoteInput {
            asset_id: asset_id.to_string(),
            day: date,
            source: "market".to_string(),
            open: None,
            high: None,
            low: None,
            close: dec(close),
            adjclose: None,
            volume: None,
            currency: "USD".to_string(),
            notes: None,
        })
        .await
        .expect("Failed to insert quote");
    }

    // ── Tests ───────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_calculate_day_empty_account() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Empty Account").await;
        let service = create_valuation_service(&pool);

        let date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let valuation = service
            .calculate_day(&account_id, date)
            .await
            .expect("calculate_day should succeed for empty account");

        assert_eq!(valuation.account_id, account_id);
        assert_eq!(valuation.total_value, Decimal::ZERO);
        assert_eq!(valuation.cash_balance, Decimal::ZERO);
        assert_eq!(valuation.investment_market_value, Decimal::ZERO);
        assert_eq!(valuation.value_status, ValuationStatus::Unavailable);
        assert_eq!(valuation.basis_status, BasisStatus::NotApplicable);
    }

    #[tokio::test]
    async fn test_calculate_day_with_holdings() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Holding Account").await;
        let asset_id = create_asset(&pool).await;
        let date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");

        // Create a lot of 10 shares @ $100, then insert a quote of $150
        create_lot(&pool, &account_id, &asset_id, "10", "100").await;
        insert_quote(&pool, &asset_id, date, "150").await;

        let service = create_valuation_service(&pool);
        let valuation = service
            .calculate_day(&account_id, date)
            .await
            .expect("calculate_day should succeed");

        assert_eq!(valuation.account_id, account_id);
        assert_eq!(valuation.total_value, dec("1500")); // 10 * 150
        assert_eq!(valuation.cash_balance, Decimal::ZERO);
        assert_eq!(valuation.investment_market_value, dec("1500"));
        assert_eq!(valuation.value_status, ValuationStatus::Complete);
        assert_eq!(valuation.basis_status, BasisStatus::Complete);
    }

    #[tokio::test]
    async fn test_calculate_day_account_not_found() {
        let pool = setup_test_db().await;
        let service = create_valuation_service(&pool);

        let date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let result = service.calculate_day("non-existent-id", date).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::NotFound(msg) => assert!(msg.contains("not found")),
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_calculate_day_persists_and_retrievable() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Persist Test").await;
        let asset_id = create_asset(&pool).await;
        let date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");

        create_lot(&pool, &account_id, &asset_id, "5", "50").await;
        insert_quote(&pool, &asset_id, date, "60").await;

        let service = create_valuation_service(&pool);
        let calculated = service
            .calculate_day(&account_id, date)
            .await
            .expect("calculate_day should succeed");

        let retrieved = service
            .get_valuation(&account_id, "2026-08-13")
            .await
            .expect("get_valuation should succeed")
            .expect("valuation should exist after persist");

        assert_eq!(retrieved.account_id, calculated.account_id);
        assert_eq!(retrieved.valuation_date, calculated.valuation_date);
        assert_eq!(retrieved.total_value, calculated.total_value);
        assert_eq!(
            retrieved.investment_market_value,
            calculated.investment_market_value
        );
        assert_eq!(retrieved.value_status, calculated.value_status);
    }

    #[tokio::test]
    async fn test_calculate_all() {
        let pool = setup_test_db().await;
        let account1_id = create_account(&pool, "Account 1").await;
        let account2_id = create_account(&pool, "Account 2").await;
        let asset_id = create_asset(&pool).await;
        let date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");

        // Add lots to both accounts
        create_lot(&pool, &account1_id, &asset_id, "10", "100").await;
        create_lot(&pool, &account2_id, &asset_id, "20", "50").await;
        insert_quote(&pool, &asset_id, date, "100").await;

        let service = create_valuation_service(&pool);
        let valuations = service
            .calculate_all(date)
            .await
            .expect("calculate_all should succeed");

        assert_eq!(
            valuations.len(),
            2,
            "should return valuations for both accounts"
        );
    }

    #[tokio::test]
    async fn test_get_valuation_series() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Series Test").await;
        let asset_id = create_asset(&pool).await;

        let aug13 = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let aug14 = NaiveDate::from_ymd_opt(2026, 8, 14).expect("valid date");
        let aug15 = NaiveDate::from_ymd_opt(2026, 8, 15).expect("valid date");

        create_lot(&pool, &account_id, &asset_id, "10", "100").await;
        insert_quote(&pool, &asset_id, aug13, "100").await;
        insert_quote(&pool, &asset_id, aug14, "110").await;
        insert_quote(&pool, &asset_id, aug15, "120").await;

        let service = create_valuation_service(&pool);

        service
            .calculate_day(&account_id, aug13)
            .await
            .expect("day 1");
        service
            .calculate_day(&account_id, aug14)
            .await
            .expect("day 2");
        service
            .calculate_day(&account_id, aug15)
            .await
            .expect("day 3");

        let series = service
            .get_valuation_series(&account_id)
            .await
            .expect("get_valuation_series should succeed");

        assert_eq!(series.len(), 3, "should return 3 valuations");
        assert_eq!(series[0].valuation_date, aug13);
        assert_eq!(series[1].valuation_date, aug14);
        assert_eq!(series[2].valuation_date, aug15);
    }
}
