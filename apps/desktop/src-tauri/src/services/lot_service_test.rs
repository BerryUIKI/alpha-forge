// Tests for LotService.
//
// Covers FIFO consumption, exact-match sales, validation guards, and
// open-lot queries across assets and accounts.

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
    use crate::error::AppError;
    use crate::services::lot_service::LotService;
    use domain::financial::{
        AccountType, ActivityStatus, ActivityType, AssetKind, CostBasisMethod, CreateAccountInput,
        CreateActivityInput, CreateAssetInput, CreateLotInput, InstrumentType, QuoteMode,
        TrackingMode,
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
            .expect("Failed to create account");
        account.id
    }

    async fn create_asset(pool: &SqlitePool) -> String {
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

    async fn create_sell_activity(
        pool: &SqlitePool,
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

    async fn create_buy_activity(
        pool: &SqlitePool,
        account_id: &str,
        asset_id: &str,
        quantity: &str,
        amount: &str,
    ) -> String {
        let repo = ActivityRepository::new(pool.clone());
        let activity = repo
            .create(CreateActivityInput {
                account_id: account_id.to_string(),
                asset_id: Some(asset_id.to_string()),
                activity_type: ActivityType::Buy,
                activity_type_override: None,
                source_type: Some("TRADE".to_string()),
                subtype: None,
                status: ActivityStatus::Posted,
                activity_date: NaiveDate::from_ymd_opt(2026, 8, 20).expect("valid date"),
                settlement_date: Some(NaiveDate::from_ymd_opt(2026, 8, 20).expect("valid date")),
                quantity: Some(dec(quantity)),
                unit_price: Some(dec(amount) / dec(quantity)),
                amount: Some(dec(amount)),
                fee: Some(dec("0")),
                tax: None,
                currency: "USD".to_string(),
                fx_rate: None,
                notes: None,
                metadata: None,
                source_system: Some("manual".to_string()),
                source_record_id: None,
                source_group_id: None,
                idempotency_key: Some(format!("manual:{}:buy:{}", account_id, asset_id)),
                import_run_id: None,
            })
            .await
            .expect("Failed to create buy activity");
        activity.id
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

    fn create_service(pool: &SqlitePool) -> LotService {
        LotService::new(
            Arc::new(LotRepository::new(pool.clone())),
            Arc::new(LotDisposalRepository::new(pool.clone())),
            Arc::new(ActivityRepository::new(pool.clone())),
        )
    }

    #[tokio::test]
    async fn test_record_sell_fifo_consumption() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "FIFO Account").await;
        let asset_id = create_asset(&pool).await;
        let lot_repo = LotRepository::new(pool.clone());
        let service = create_service(&pool);

        // Lot 1: 10 shares @ $100 = $1000
        let lot1 = lot_repo
            .create(lot_input(&account_id, &asset_id, "10", "100"))
            .await
            .unwrap();
        // Lot 2: 5 shares @ $110 = $550
        let lot2 = lot_repo
            .create(lot_input(&account_id, &asset_id, "5", "110"))
            .await
            .unwrap();

        // Sell 12 shares @ $125 = $1500
        let sell_activity_id =
            create_sell_activity(&pool, &account_id, &asset_id, "12", "1500").await;

        let result = service
            .record_sell(&account_id, &asset_id, &sell_activity_id)
            .await
            .expect("Failed to record sell");

        assert_eq!(result.total_quantity, dec("12"));
        assert_eq!(result.total_proceeds, dec("1500"));
        // Lot 1 (10 @ 100) fully consumed: cost_basis = 1000
        // Lot 2 (5 @ 110) partially consumed: 2 shares, cost_basis = 2/5 * 550 = 220
        // Total cost basis = 1000 + 220 = 1220
        assert_eq!(result.total_cost_basis, dec("1220"));
        // Realized PnL = 1500 - 1220 = 280
        assert_eq!(result.total_realized_pnl, dec("280"));
        assert_eq!(result.lots_consumed, 1); // lot1 fully consumed
        assert_eq!(result.lots_partially_consumed, 1); // lot2 partially consumed

        // Verify lot states
        let lot1_fresh = lot_repo.get(&lot1.id).await.unwrap().unwrap();
        assert!(lot1_fresh.is_closed);
        assert_eq!(lot1_fresh.remaining_quantity, dec("0"));

        let lot2_fresh = lot_repo.get(&lot2.id).await.unwrap().unwrap();
        assert!(!lot2_fresh.is_closed);
        assert_eq!(lot2_fresh.remaining_quantity, dec("3")); // 5 - 2
                                                             // remaining cost basis: 550 - 220 = 330
        assert_eq!(lot2_fresh.remaining_cost_basis, dec("330"));
    }

    #[tokio::test]
    async fn test_record_sell_exact_fifo() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Exact FIFO").await;
        let asset_id = create_asset(&pool).await;
        let lot_repo = LotRepository::new(pool.clone());
        let service = create_service(&pool);

        // Single lot: 10 shares @ $100 = $1000
        let lot = lot_repo
            .create(lot_input(&account_id, &asset_id, "10", "100"))
            .await
            .unwrap();

        // Sell exactly 10 shares @ $125 = $1250
        let sell_activity_id =
            create_sell_activity(&pool, &account_id, &asset_id, "10", "1250").await;

        let result = service
            .record_sell(&account_id, &asset_id, &sell_activity_id)
            .await
            .expect("Failed to record exact sell");

        assert_eq!(result.total_quantity, dec("10"));
        assert_eq!(result.total_proceeds, dec("1250"));
        assert_eq!(result.total_cost_basis, dec("1000"));
        assert_eq!(result.total_realized_pnl, dec("250"));
        assert_eq!(result.lots_consumed, 1);
        assert_eq!(result.lots_partially_consumed, 0);

        // Lot should be closed
        let lot_fresh = lot_repo.get(&lot.id).await.unwrap().unwrap();
        assert!(lot_fresh.is_closed);
        assert_eq!(lot_fresh.remaining_quantity, dec("0"));
    }

    #[tokio::test]
    async fn test_record_sell_non_sell_activity() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Non-Sell").await;
        let asset_id = create_asset(&pool).await;
        let service = create_service(&pool);

        // Create a buy activity, not a sell
        let buy_activity_id =
            create_buy_activity(&pool, &account_id, &asset_id, "10", "1000").await;

        let result = service
            .record_sell(&account_id, &asset_id, &buy_activity_id)
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Validation(msg) => assert!(msg.contains("is not a sell")),
            _ => panic!("Expected Validation error"),
        }
    }

    #[tokio::test]
    async fn test_record_sell_exceeds_available() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Exceeds Available").await;
        let asset_id = create_asset(&pool).await;
        let lot_repo = LotRepository::new(pool.clone());
        let service = create_service(&pool);

        // Lot: 5 shares @ $100
        lot_repo
            .create(lot_input(&account_id, &asset_id, "5", "100"))
            .await
            .unwrap();

        // Try to sell 10 shares
        let sell_activity_id =
            create_sell_activity(&pool, &account_id, &asset_id, "10", "1250").await;

        let result = service
            .record_sell(&account_id, &asset_id, &sell_activity_id)
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Validation(msg) => assert!(msg.contains("exceeds available")),
            _ => panic!("Expected Validation error"),
        }
    }

    #[tokio::test]
    async fn test_record_sell_no_open_lots() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "No Lots").await;
        let asset_id = create_asset(&pool).await;
        let service = create_service(&pool);

        // No lots created; try to sell
        let sell_activity_id =
            create_sell_activity(&pool, &account_id, &asset_id, "1", "100").await;

        let result = service
            .record_sell(&account_id, &asset_id, &sell_activity_id)
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Validation(msg) => assert!(msg.contains("no open lots")),
            _ => panic!("Expected Validation error"),
        }
    }

    #[tokio::test]
    async fn test_get_open_lots() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Open Lots").await;
        let asset_id = create_asset(&pool).await;
        let lot_repo = LotRepository::new(pool.clone());
        let service = create_service(&pool);

        // Create 2 lots
        lot_repo
            .create(lot_input(&account_id, &asset_id, "10", "100"))
            .await
            .unwrap();
        lot_repo
            .create(lot_input(&account_id, &asset_id, "5", "110"))
            .await
            .unwrap();

        let open_lots = service
            .get_open_lots(&account_id, &asset_id)
            .await
            .expect("Failed to get open lots");

        assert_eq!(open_lots.len(), 2);
        // Both should be open
        for lot in &open_lots {
            assert!(!lot.is_closed);
        }
    }

    #[tokio::test]
    async fn test_get_open_lots_for_account() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Multi Asset").await;
        let asset1_id = create_asset(&pool).await;

        // Create a second asset
        let asset2_repo = AssetRepository::new(pool.clone());
        let asset2 = asset2_repo
            .create(CreateAssetInput {
                kind: AssetKind::Investment,
                name: Some("Microsoft Corp".to_string()),
                display_code: Some("MSFT".to_string()),
                notes: None,
                is_active: true,
                quote_mode: QuoteMode::Market,
                quote_ccy: "USD".to_string(),
                instrument_type: Some(InstrumentType::Equity),
                instrument_symbol: Some("MSFT".to_string()),
                instrument_exchange_mic: Some("XNAS".to_string()),
                provider_config: None,
            })
            .await
            .expect("Failed to create asset");
        let asset2_id = asset2.id;

        let lot_repo = LotRepository::new(pool.clone());
        let service = create_service(&pool);

        // Create lot in asset 1
        lot_repo
            .create(lot_input(&account_id, &asset1_id, "10", "100"))
            .await
            .unwrap();
        // Create lot in asset 2
        lot_repo
            .create(lot_input(&account_id, &asset2_id, "20", "50"))
            .await
            .unwrap();

        let open_lots = service
            .get_open_lots_for_account(&account_id)
            .await
            .expect("Failed to get open lots for account");

        assert_eq!(open_lots.len(), 2);

        let asset1_lots: Vec<_> = open_lots
            .iter()
            .filter(|l| l.asset_id == asset1_id)
            .collect();
        let asset2_lots: Vec<_> = open_lots
            .iter()
            .filter(|l| l.asset_id == asset2_id)
            .collect();

        assert_eq!(asset1_lots.len(), 1);
        assert_eq!(asset1_lots[0].remaining_quantity, dec("10"));

        assert_eq!(asset2_lots.len(), 1);
        assert_eq!(asset2_lots[0].remaining_quantity, dec("20"));
    }
}
