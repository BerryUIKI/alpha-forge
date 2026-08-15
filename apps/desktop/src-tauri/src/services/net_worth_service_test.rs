// Tests for NetWorthService.
//
// Covers single account, multiple accounts, archived accounts, no accounts,
// and credit card as liability.

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
    use crate::services::holdings_service::HoldingsService;
    use crate::services::net_worth_service::NetWorthService;
    use domain::financial::{
        AccountType, AssetKind, CostBasisMethod, CreateAccountInput, CreateAssetInput,
        CreateLotInput, InstrumentType, QuoteMode, TrackingMode,
    };

    fn dec(value: &str) -> Decimal {
        Decimal::from_str_exact(value).expect("valid decimal")
    }

    fn create_holdings_service(pool: &SqlitePool) -> Arc<HoldingsService> {
        Arc::new(HoldingsService::new(
            Arc::new(AccountRepository::new(pool.clone())),
            Arc::new(AssetRepository::new(pool.clone())),
            Arc::new(QuoteRepository::new(pool.clone())),
            Arc::new(LotRepository::new(pool.clone())),
            Arc::new(LotDisposalRepository::new(pool.clone())),
        ))
    }

    fn create_service(pool: &SqlitePool) -> NetWorthService {
        NetWorthService::new(
            Arc::new(AccountRepository::new(pool.clone())),
            create_holdings_service(pool),
        )
    }

    async fn create_account(pool: &SqlitePool, name: &str, account_type: AccountType) -> String {
        let repo = AccountRepository::new(pool.clone());
        let account = repo
            .create(CreateAccountInput {
                workspace_id: None,
                name: name.to_string(),
                account_type,
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

    async fn create_asset(
        pool: &SqlitePool,
        kind: AssetKind,
        symbol: &str,
        quote_mode: QuoteMode,
    ) -> String {
        let repo = AssetRepository::new(pool.clone());
        let asset = repo
            .create(CreateAssetInput {
                kind,
                name: Some(format!("Asset {}", symbol)),
                display_code: Some(symbol.to_string()),
                notes: None,
                is_active: true,
                quote_mode,
                quote_ccy: "USD".to_string(),
                instrument_type: if quote_mode == QuoteMode::Market {
                    Some(InstrumentType::Equity)
                } else {
                    None
                },
                instrument_symbol: if quote_mode == QuoteMode::Market {
                    Some(symbol.to_string())
                } else {
                    None
                },
                instrument_exchange_mic: if quote_mode == QuoteMode::Market {
                    Some("XNAS".to_string())
                } else {
                    None
                },
                provider_config: None,
            })
            .await
            .expect("Failed to create asset");
        asset.id
    }

    async fn create_lot(
        pool: &SqlitePool,
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

    #[tokio::test]
    async fn test_compute_net_worth_single_account() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Brokerage", AccountType::Securities).await;
        let asset_id = create_asset(&pool, AssetKind::Investment, "AAPL", QuoteMode::Market).await;
        // 10 shares @ $100 = $1000 market value (fallback to cost basis)
        create_lot(&pool, &account_id, &asset_id, "10", "100").await;

        let service = create_service(&pool);

        let as_of_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let nw = service
            .compute_net_worth(as_of_date, "USD")
            .await
            .expect("Failed to compute net worth");

        assert_eq!(nw.as_of_date, as_of_date);
        assert_eq!(nw.base_currency, "USD");
        assert_eq!(nw.total_assets, dec("1000"));
        assert_eq!(nw.total_liabilities, Decimal::ZERO);
        assert_eq!(nw.net_worth, dec("1000"));
        assert_eq!(nw.accounts.len(), 1);
        assert_eq!(nw.accounts[0].account_id, account_id);
    }

    #[tokio::test]
    async fn test_compute_net_worth_multiple_accounts() {
        let pool = setup_test_db().await;
        let account1_id = create_account(&pool, "Brokerage 1", AccountType::Securities).await;
        let account2_id = create_account(&pool, "Brokerage 2", AccountType::Securities).await;
        let asset_id = create_asset(&pool, AssetKind::Investment, "AAPL", QuoteMode::Market).await;

        // Account 1: 10 shares @ $100 = $1000
        create_lot(&pool, &account1_id, &asset_id, "10", "100").await;
        // Account 2: 5 shares @ $200 = $1000
        create_lot(&pool, &account2_id, &asset_id, "5", "200").await;

        let service = create_service(&pool);

        let as_of_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let nw = service
            .compute_net_worth(as_of_date, "USD")
            .await
            .expect("Failed to compute net worth");

        // Total = 1000 + 1000 = 2000
        assert_eq!(nw.total_assets, dec("2000"));
        assert_eq!(nw.total_liabilities, Decimal::ZERO);
        assert_eq!(nw.net_worth, dec("2000"));
        assert_eq!(nw.accounts.len(), 2);
    }

    #[tokio::test]
    async fn test_compute_net_worth_archived_account_skipped() {
        let pool = setup_test_db().await;
        let account1_id = create_account(&pool, "Active", AccountType::Securities).await;
        let account2_id = create_account(&pool, "Archived", AccountType::Securities).await;
        let asset_id = create_asset(&pool, AssetKind::Investment, "AAPL", QuoteMode::Market).await;

        create_lot(&pool, &account1_id, &asset_id, "10", "100").await;
        create_lot(&pool, &account2_id, &asset_id, "10", "100").await;

        // Archive account 2 via raw SQL
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query("UPDATE accounts SET is_archived = 1, updated_at = ? WHERE id = ?")
            .bind(&now)
            .bind(&account2_id)
            .execute(&pool)
            .await
            .expect("Failed to archive account");

        let service = create_service(&pool);

        let as_of_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let nw = service
            .compute_net_worth(as_of_date, "USD")
            .await
            .expect("Failed to compute net worth");

        // Only active account counted
        assert_eq!(nw.total_assets, dec("1000"));
        assert_eq!(nw.total_liabilities, Decimal::ZERO);
        assert_eq!(nw.net_worth, dec("1000"));
        assert_eq!(nw.accounts.len(), 1);
        assert_eq!(nw.accounts[0].account_id, account1_id);
    }

    #[tokio::test]
    async fn test_compute_net_worth_no_accounts() {
        let pool = setup_test_db().await;
        let service = create_service(&pool);

        let as_of_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let nw = service
            .compute_net_worth(as_of_date, "USD")
            .await
            .expect("Failed to compute net worth");

        assert_eq!(nw.total_assets, Decimal::ZERO);
        assert_eq!(nw.total_liabilities, Decimal::ZERO);
        assert_eq!(nw.net_worth, Decimal::ZERO);
        assert!(nw.accounts.is_empty());
    }

    #[tokio::test]
    async fn test_compute_net_worth_credit_card_as_liability() {
        let pool = setup_test_db().await;
        let cc_account_id = create_account(&pool, "Credit Card", AccountType::CreditCard).await;

        // Create a cash-type asset (manual quote mode)
        let cash_asset_id = create_asset(&pool, AssetKind::Other, "CASH", QuoteMode::Manual).await;

        // A credit card balance of -500 (negative cash = liability)
        // The HoldingsService treats assets with kind=Investment/Manual or kind=Other
        // as cash. The quantity is the cash balance.
        create_lot(&pool, &cc_account_id, &cash_asset_id, "-500", "1").await;

        let service = create_service(&pool);

        let as_of_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let nw = service
            .compute_net_worth(as_of_date, "USD")
            .await
            .expect("Failed to compute net worth");

        // The credit card account is a liability type. Its negative cash balance
        // (-500) becomes the total_value. Since it's a liability, this goes into
        // total_liabilities.
        assert_eq!(nw.total_assets, Decimal::ZERO);
        // The total_value_base = cash_balance_base + total_market_value_base
        // cash_balance = -500, cash_balance_base = -500
        // total_market_value = 0, total_market_value_base = 0
        // total_value_base = -500 + 0 = -500
        // But since it's a liability, total_liabilities += -500 = -500
        // net_worth = 0 - (-500) = 500
        assert_eq!(nw.total_liabilities, dec("-500"));
        assert_eq!(nw.net_worth, dec("500"));
        assert_eq!(nw.accounts.len(), 1);
        assert_eq!(nw.accounts[0].account_type, AccountType::CreditCard);
    }
}
