// Tests for HoldingsService.
//
// Covers empty accounts, lot aggregation, missing accounts, quote-based
// market values, multi-account queries, and archived-account filtering.

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::NaiveDate;
    use rust_decimal::Decimal;
    use sqlx::SqlitePool;

    use crate::database::repositories::account_repository::AccountRepository;
    use crate::database::repositories::asset_repository::{AssetRepository, QuoteRepository};
    use crate::database::repositories::lot_repository::{LotDisposalRepository, LotRepository};
    use crate::database::repositories::test_support::setup_test_db;
    use crate::error::AppError;
    use crate::services::holdings_service::HoldingsService;
    use domain::financial::{
        AccountType, AssetKind, CostBasisMethod, CreateAccountInput, CreateAssetInput,
        CreateLotInput, InstrumentType, QuoteMode, TrackingMode, UpsertQuoteInput,
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

    fn create_service(pool: &SqlitePool) -> HoldingsService {
        HoldingsService::new(
            Arc::new(AccountRepository::new(pool.clone())),
            Arc::new(AssetRepository::new(pool.clone())),
            Arc::new(QuoteRepository::new(pool.clone())),
            Arc::new(LotRepository::new(pool.clone())),
            Arc::new(LotDisposalRepository::new(pool.clone())),
        )
    }

    #[tokio::test]
    async fn test_get_holdings_empty_account() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Empty Account").await;
        let service = create_service(&pool);

        let as_of_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let summary = service
            .get_holdings(&account_id, as_of_date)
            .await
            .expect("Failed to get holdings");

        assert_eq!(summary.account_id, account_id);
        assert_eq!(summary.as_of_date, as_of_date);
        assert!(summary.holdings.is_empty());
        assert_eq!(summary.total_market_value, Decimal::ZERO);
        assert_eq!(summary.total_cost_basis, Decimal::ZERO);
        assert_eq!(summary.total_unrealized_gain, Decimal::ZERO);
        assert_eq!(summary.total_realized_gain, Decimal::ZERO);
        assert_eq!(summary.total_market_value_base, Decimal::ZERO);
        assert_eq!(summary.total_cost_basis_base, Decimal::ZERO);
        assert_eq!(summary.total_unrealized_gain_base, Decimal::ZERO);
        assert_eq!(summary.total_realized_gain_base, Decimal::ZERO);
        assert_eq!(summary.cash_balance, Decimal::ZERO);
        assert_eq!(summary.cash_balance_base, Decimal::ZERO);
    }

    #[tokio::test]
    async fn test_get_holdings_with_lots() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Test Account").await;
        let asset_id = create_asset(&pool).await;
        let lot_repo = LotRepository::new(pool.clone());
        let service = create_service(&pool);

        // Create 2 lots: 10 @ $100, 5 @ $110
        lot_repo
            .create(lot_input(&account_id, &asset_id, "10", "100"))
            .await
            .unwrap();
        lot_repo
            .create(lot_input(&account_id, &asset_id, "5", "110"))
            .await
            .unwrap();

        let as_of_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let summary = service
            .get_holdings(&account_id, as_of_date)
            .await
            .expect("Failed to get holdings");

        assert_eq!(summary.holdings.len(), 1);
        let holding = &summary.holdings[0];
        assert_eq!(holding.asset_id, asset_id);
        assert_eq!(holding.quantity, dec("15"));
        assert_eq!(holding.cost_basis, dec("1550")); // 10*100 + 5*110
        assert_eq!(holding.market_value, dec("1550")); // fallback to cost basis
        assert_eq!(holding.unrealized_gain, Decimal::ZERO);
        assert_eq!(holding.open_lot_count, 2);
        assert_eq!(summary.total_market_value, dec("1550"));
        assert_eq!(summary.total_cost_basis, dec("1550"));
    }

    #[tokio::test]
    async fn test_get_holdings_account_not_found() {
        let pool = setup_test_db().await;
        let service = create_service(&pool);

        let result = service
            .get_holdings(
                "non-existent-id",
                NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date"),
            )
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::NotFound(msg) => assert!(msg.contains("non-existent-id")),
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_get_all_holdings() {
        let pool = setup_test_db().await;
        let account1_id = create_account(&pool, "Account 1").await;
        let account2_id = create_account(&pool, "Account 2").await;
        let asset_id = create_asset(&pool).await;
        let lot_repo = LotRepository::new(pool.clone());
        let service = create_service(&pool);

        // Add lots to account 1 only
        lot_repo
            .create(lot_input(&account1_id, &asset_id, "10", "100"))
            .await
            .unwrap();

        let as_of_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let summaries = service
            .get_all_holdings(as_of_date)
            .await
            .expect("Failed to get all holdings");

        // Both accounts returned; account 1 has holdings, account 2 is empty
        assert_eq!(summaries.len(), 2);

        let acct1 = summaries
            .iter()
            .find(|s| s.account_id == account1_id)
            .expect("Account 1 summary missing");
        assert_eq!(acct1.holdings.len(), 1);
        assert_eq!(acct1.holdings[0].quantity, dec("10"));

        let acct2 = summaries
            .iter()
            .find(|s| s.account_id == account2_id)
            .expect("Account 2 summary missing");
        assert!(acct2.holdings.is_empty());
    }

    #[tokio::test]
    async fn test_get_holdings_with_quote() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Quoted Account").await;
        let asset_id = create_asset(&pool).await;
        let lot_repo = LotRepository::new(pool.clone());
        let quote_repo = QuoteRepository::new(pool.clone());
        let service = create_service(&pool);

        // Create a lot: 10 shares @ $100
        lot_repo
            .create(lot_input(&account_id, &asset_id, "10", "100"))
            .await
            .unwrap();

        // Upsert a quote: $150/share on the as-of date
        let as_of_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        quote_repo
            .upsert(UpsertQuoteInput {
                asset_id: asset_id.clone(),
                day: as_of_date,
                source: "market".to_string(),
                open: None,
                high: None,
                low: None,
                close: dec("150"),
                adjclose: None,
                volume: None,
                currency: "USD".to_string(),
                notes: None,
            })
            .await
            .expect("Failed to upsert quote");

        let summary = service
            .get_holdings(&account_id, as_of_date)
            .await
            .expect("Failed to get holdings");

        assert_eq!(summary.holdings.len(), 1);
        let holding = &summary.holdings[0];
        assert_eq!(holding.quantity, dec("10"));
        assert_eq!(holding.cost_basis, dec("1000"));
        assert_eq!(holding.market_value, dec("1500")); // 10 * $150
        assert_eq!(holding.unrealized_gain, dec("500")); // 1500 - 1000
        assert_eq!(holding.unrealized_gain_pct, Some(dec("50"))); // 50%
        assert_eq!(holding.fx_rate, Decimal::ONE);
    }

    #[tokio::test]
    async fn test_get_holdings_archived_account_skipped() {
        let pool = setup_test_db().await;
        let account1_id = create_account(&pool, "Active Account").await;
        let account2_id = create_account(&pool, "Archived Account").await;
        let asset_id = create_asset(&pool).await;
        let lot_repo = LotRepository::new(pool.clone());
        let service = create_service(&pool);

        // Add lots to both accounts
        lot_repo
            .create(lot_input(&account1_id, &asset_id, "10", "100"))
            .await
            .unwrap();
        lot_repo
            .create(lot_input(&account2_id, &asset_id, "5", "50"))
            .await
            .unwrap();

        // Archive account 2 via raw SQL (no repository method exists)
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query("UPDATE accounts SET is_archived = 1, updated_at = ? WHERE id = ?")
            .bind(&now)
            .bind(&account2_id)
            .execute(&pool)
            .await
            .expect("Failed to archive account");

        let summaries = service
            .get_all_holdings(NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date"))
            .await
            .expect("Failed to get all holdings");

        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].account_id, account1_id);
    }
}
