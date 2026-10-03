// Tests for ActivityService — atomicity, FIFO lots, and idempotency.

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::NaiveDate;
    use rust_decimal::Decimal;
    use sqlx::SqlitePool;

    use crate::database::repositories::account_repository::AccountRepository;
    use crate::database::repositories::activity_repository::ActivityRepository;
    use crate::database::repositories::asset_repository::AssetRepository;
    use crate::database::repositories::lot_repository::{LotDisposalRepository, LotRepository};
    use crate::database::repositories::test_support::setup_test_db;
    use crate::services::activity_service::ActivityService;
    use domain::financial::{
        AccountType, ActivityStatus, ActivityType, AssetKind, CreateAccountInput,
        CreateActivityInput, CreateAssetInput, InstrumentType, QuoteMode, TrackingMode,
    };

    fn dec(value: &str) -> Decimal {
        Decimal::from_str_exact(value).expect("valid decimal")
    }

    async fn create_account(pool: &SqlitePool, name: &str) -> String {
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
            .expect("failed to create account");
        account.id
    }

    async fn create_asset(pool: &SqlitePool, symbol: &str) -> String {
        let repo = AssetRepository::new(pool.clone());
        let asset = repo
            .create(CreateAssetInput {
                kind: AssetKind::Investment,
                name: Some(format!("{symbol} Inc")),
                display_code: Some(symbol.to_string()),
                notes: None,
                is_active: true,
                quote_mode: QuoteMode::Market,
                quote_ccy: "USD".to_string(),
                instrument_type: Some(InstrumentType::Equity),
                instrument_symbol: Some(symbol.to_string()),
                instrument_exchange_mic: Some("XNAS".to_string()),
                provider_config: None,
            })
            .await
            .expect("failed to create asset");
        asset.id
    }

    fn create_service(pool: &SqlitePool) -> ActivityService {
        ActivityService::new(
            pool.clone(),
            Arc::new(ActivityRepository::new(pool.clone())),
            Arc::new(AccountRepository::new(pool.clone())),
            Arc::new(AssetRepository::new(pool.clone())),
            Arc::new(LotRepository::new(pool.clone())),
            Arc::new(LotDisposalRepository::new(pool.clone())),
        )
    }

    #[tokio::test]
    async fn buy_activity_creates_activity_and_open_lot_atomically() {
        let pool = setup_test_db().await;
        let service = create_service(&pool);
        let lot_repo = LotRepository::new(pool.clone());

        let account_id = create_account(&pool, "acct-buy-1").await;
        let asset_id = create_asset(&pool, "AAPL").await;
        let day = NaiveDate::from_ymd_opt(2026, 8, 20).expect("valid date");

        let input = CreateActivityInput {
            account_id: account_id.clone(),
            asset_id: Some(asset_id.clone()),
            activity_type: ActivityType::Buy,
            activity_type_override: None,
            source_type: Some("TRADE".to_string()),
            subtype: None,
            status: ActivityStatus::Posted,
            activity_date: day,
            settlement_date: Some(day),
            quantity: Some(dec("10")),
            unit_price: Some(dec("150.00")),
            amount: None, // Service computes amount = qty * price + fee = 1505.00
            fee: Some(dec("5.00")),
            tax: None,
            currency: "USD".to_string(),
            fx_rate: None,
            notes: Some("Initial manual purchase".to_string()),
            metadata: None,
            source_system: Some("manual".to_string()),
            source_record_id: None,
            source_group_id: None,
            idempotency_key: Some("idem-buy-1".to_string()),
            import_run_id: None,
        };

        let activity = service
            .create_activity(input)
            .await
            .expect("activity creation succeeds");

        assert_eq!(activity.quantity, Some(dec("10")));
        assert_eq!(activity.unit_price, Some(dec("150.00")));
        assert_eq!(activity.amount, Some(dec("1505.00")));
        assert_eq!(activity.fee, Some(dec("5.00")));

        // Verify open lot is automatically created with exact cost basis
        let open_lots = lot_repo
            .list_open_by_account_asset(&account_id, &asset_id)
            .await
            .expect("fetch open lots succeeds");
        assert_eq!(open_lots.len(), 1);

        let lot = &open_lots[0];
        assert_eq!(lot.open_activity_id, Some(activity.id));
        assert_eq!(lot.original_quantity, dec("10"));
        assert_eq!(lot.remaining_quantity, dec("10"));
        assert_eq!(lot.cost_per_unit, dec("150.00"));
        assert_eq!(lot.original_cost_basis, dec("1505.00"));
        assert_eq!(lot.remaining_cost_basis, dec("1505.00"));
        assert_eq!(lot.fee_allocated, dec("5.00"));
        assert!(!lot.is_closed);
    }

    #[tokio::test]
    async fn sell_activity_disposes_open_lots_atomically() {
        let pool = setup_test_db().await;
        let service = create_service(&pool);
        let lot_repo = LotRepository::new(pool.clone());
        let disposal_repo = LotDisposalRepository::new(pool.clone());

        let account_id = create_account(&pool, "acct-sell-1").await;
        let asset_id = create_asset(&pool, "MSFT").await;
        let day1 = NaiveDate::from_ymd_opt(2026, 8, 10).expect("valid date");
        let day2 = NaiveDate::from_ymd_opt(2026, 8, 20).expect("valid date");

        // 1. Buy 10 shares at $100 (total cost 1000)
        service
            .create_activity(CreateActivityInput {
                account_id: account_id.clone(),
                asset_id: Some(asset_id.clone()),
                activity_type: ActivityType::Buy,
                activity_type_override: None,
                source_type: None,
                subtype: None,
                status: ActivityStatus::Posted,
                activity_date: day1,
                settlement_date: None,
                quantity: Some(dec("10")),
                unit_price: Some(dec("100.00")),
                amount: Some(dec("1000.00")),
                fee: Some(dec("0")),
                tax: None,
                currency: "USD".to_string(),
                fx_rate: None,
                notes: None,
                metadata: None,
                source_system: Some("manual".to_string()),
                source_record_id: None,
                source_group_id: None,
                idempotency_key: Some("idem-buy-msft".to_string()),
                import_run_id: None,
            })
            .await
            .expect("buy succeeds");

        // 2. Sell 4 shares at $150 (proceeds $600)
        let _sell_act = service
            .create_activity(CreateActivityInput {
                account_id: account_id.clone(),
                asset_id: Some(asset_id.clone()),
                activity_type: ActivityType::Sell,
                activity_type_override: None,
                source_type: None,
                subtype: None,
                status: ActivityStatus::Posted,
                activity_date: day2,
                settlement_date: None,
                quantity: Some(dec("4")),
                unit_price: Some(dec("150.00")),
                amount: Some(dec("600.00")),
                fee: Some(dec("0")),
                tax: None,
                currency: "USD".to_string(),
                fx_rate: None,
                notes: None,
                metadata: None,
                source_system: Some("manual".to_string()),
                source_record_id: None,
                source_group_id: None,
                idempotency_key: Some("idem-sell-msft".to_string()),
                import_run_id: None,
            })
            .await
            .expect("sell succeeds");

        // 3. Verify open lots have 6 shares remaining with cost basis 600
        let open_lots = lot_repo
            .list_open_by_account_asset(&account_id, &asset_id)
            .await
            .expect("list open lots succeeds");
        assert_eq!(open_lots.len(), 1);
        assert_eq!(open_lots[0].remaining_quantity, dec("6"));
        assert_eq!(open_lots[0].remaining_cost_basis, dec("600.00"));

        // 4. Verify disposal record created
        let disposals = disposal_repo
            .list_by_account(&account_id)
            .await
            .expect("list disposals succeeds");
        assert_eq!(disposals.len(), 1);
        assert_eq!(disposals[0].quantity, dec("4"));
        assert_eq!(disposals[0].proceeds, dec("600.00"));
        assert_eq!(disposals[0].cost_basis, dec("400.00"));
        assert_eq!(disposals[0].realized_pnl, dec("200.00"));
    }

    #[tokio::test]
    async fn sell_activity_fails_and_rolls_back_if_insufficient_lots() {
        let pool = setup_test_db().await;
        let service = create_service(&pool);
        let activity_repo = ActivityRepository::new(pool.clone());
        let lot_repo = LotRepository::new(pool.clone());

        let account_id = create_account(&pool, "acct-insufficient").await;
        let asset_id = create_asset(&pool, "NVDA").await;
        let day = NaiveDate::from_ymd_opt(2026, 8, 15).expect("valid date");

        // Buy only 5 shares
        service
            .create_activity(CreateActivityInput {
                account_id: account_id.clone(),
                asset_id: Some(asset_id.clone()),
                activity_type: ActivityType::Buy,
                activity_type_override: None,
                source_type: None,
                subtype: None,
                status: ActivityStatus::Posted,
                activity_date: day,
                settlement_date: None,
                quantity: Some(dec("5")),
                unit_price: Some(dec("100.00")),
                amount: Some(dec("500.00")),
                fee: None,
                tax: None,
                currency: "USD".to_string(),
                fx_rate: None,
                notes: None,
                metadata: None,
                source_system: Some("manual".to_string()),
                source_record_id: None,
                source_group_id: None,
                idempotency_key: Some("idem-buy-nvda".to_string()),
                import_run_id: None,
            })
            .await
            .expect("buy succeeds");

        // Attempt to sell 10 shares (exceeds available 5)
        let sell_result = service
            .create_activity(CreateActivityInput {
                account_id: account_id.clone(),
                asset_id: Some(asset_id.clone()),
                activity_type: ActivityType::Sell,
                activity_type_override: None,
                source_type: None,
                subtype: None,
                status: ActivityStatus::Posted,
                activity_date: day,
                settlement_date: None,
                quantity: Some(dec("10")),
                unit_price: Some(dec("120.00")),
                amount: Some(dec("1200.00")),
                fee: None,
                tax: None,
                currency: "USD".to_string(),
                fx_rate: None,
                notes: None,
                metadata: None,
                source_system: Some("manual".to_string()),
                source_record_id: None,
                source_group_id: None,
                idempotency_key: Some("idem-sell-nvda-fail".to_string()),
                import_run_id: None,
            })
            .await;

        assert!(
            sell_result.is_err(),
            "selling more than available must fail"
        );

        // Verify transaction was rolled back: NO sell activity exists!
        let all_activities = activity_repo
            .list_by_account(&account_id)
            .await
            .expect("list activities succeeds");
        assert_eq!(all_activities.len(), 1, "only the buy activity remains");
        assert_eq!(all_activities[0].activity_type, ActivityType::Buy);

        // Verify open lot is untouched: still 5 shares open
        let open_lots = lot_repo
            .list_open_by_account_asset(&account_id, &asset_id)
            .await
            .expect("list lots succeeds");
        assert_eq!(open_lots.len(), 1);
        assert_eq!(open_lots[0].remaining_quantity, dec("5"));
    }

    #[tokio::test]
    async fn idempotent_submission_returns_existing_without_duplicates() {
        let pool = setup_test_db().await;
        let service = create_service(&pool);
        let activity_repo = ActivityRepository::new(pool.clone());
        let lot_repo = LotRepository::new(pool.clone());

        let account_id = create_account(&pool, "acct-idem-test").await;
        let asset_id = create_asset(&pool, "GOOGL").await;
        let day = NaiveDate::from_ymd_opt(2026, 8, 12).expect("valid date");

        let input = CreateActivityInput {
            account_id: account_id.clone(),
            asset_id: Some(asset_id.clone()),
            activity_type: ActivityType::Buy,
            activity_type_override: None,
            source_type: None,
            subtype: None,
            status: ActivityStatus::Posted,
            activity_date: day,
            settlement_date: None,
            quantity: Some(dec("20")),
            unit_price: Some(dec("150.00")),
            amount: Some(dec("3000.00")),
            fee: None,
            tax: None,
            currency: "USD".to_string(),
            fx_rate: None,
            notes: None,
            metadata: None,
            source_system: Some("manual".to_string()),
            source_record_id: None,
            source_group_id: None,
            idempotency_key: Some("stable-dialog-submission-uuid-1".to_string()),
            import_run_id: None,
        };

        // First call
        let act1 = service
            .create_activity(input.clone())
            .await
            .expect("first submission succeeds");

        // Second call with same idempotency key (e.g. user retried on network delay)
        let act2 = service
            .create_activity(input)
            .await
            .expect("retry submission succeeds");

        assert_eq!(act1.id, act2.id, "must return identical activity ID");

        // Verify exactly one activity and one lot exist
        let activities = activity_repo
            .list_by_account(&account_id)
            .await
            .expect("list activities succeeds");
        assert_eq!(activities.len(), 1);

        let lots = lot_repo
            .list_open_by_account_asset(&account_id, &asset_id)
            .await
            .expect("list lots succeeds");
        assert_eq!(lots.len(), 1);
        assert_eq!(lots[0].original_quantity, dec("20"));
    }

    #[tokio::test]
    async fn high_precision_fractional_quantities_preserved_without_float_distortion() {
        let pool = setup_test_db().await;
        let service = create_service(&pool);
        let lot_repo = LotRepository::new(pool.clone());

        let account_id = create_account(&pool, "acct-crypto").await;
        let asset_id = create_asset(&pool, "BTC").await;
        let day = NaiveDate::from_ymd_opt(2026, 8, 14).expect("valid date");

        // Fractional crypto with high precision: 0.12345678 BTC @ 60000.00
        let input = CreateActivityInput {
            account_id: account_id.clone(),
            asset_id: Some(asset_id.clone()),
            activity_type: ActivityType::Buy,
            activity_type_override: None,
            source_type: None,
            subtype: None,
            status: ActivityStatus::Posted,
            activity_date: day,
            settlement_date: None,
            quantity: Some(dec("0.12345678")),
            unit_price: Some(dec("60000.00")),
            amount: None,
            fee: Some(dec("1.50")),
            tax: None,
            currency: "USD".to_string(),
            fx_rate: None,
            notes: None,
            metadata: None,
            source_system: Some("manual".to_string()),
            source_record_id: None,
            source_group_id: None,
            idempotency_key: Some("idem-crypto-btc".to_string()),
            import_run_id: None,
        };

        let act = service
            .create_activity(input)
            .await
            .expect("fractional purchase succeeds");

        assert_eq!(act.quantity, Some(dec("0.12345678")));

        let open_lots = lot_repo
            .list_open_by_account_asset(&account_id, &asset_id)
            .await
            .expect("fetch open lots succeeds");
        assert_eq!(open_lots.len(), 1);
        assert_eq!(open_lots[0].remaining_quantity, dec("0.12345678"));
        // 0.12345678 * 60000.00 = 7407.4068 -> round_dp(2) = 7407.41 + 1.50 = 7408.91
        assert_eq!(open_lots[0].original_cost_basis, dec("7408.91"));
    }
}
