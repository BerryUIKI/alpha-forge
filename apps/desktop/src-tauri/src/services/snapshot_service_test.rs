// Tests for SnapshotService.
//
// Covers snapshot creation (empty and with holdings), retrieval, listing,
// deletion, and account-not-found handling.

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::NaiveDate;
    use rust_decimal::Decimal;
    use sqlx::SqlitePool;

    use crate::database::repositories::account_repository::AccountRepository;
    use crate::database::repositories::asset_repository::{AssetRepository, QuoteRepository};
    use crate::database::repositories::lot_repository::{LotDisposalRepository, LotRepository};
    use crate::database::repositories::snapshot_repository::SnapshotRepository;
    use crate::database::repositories::test_support::setup_test_db;
    use crate::error::AppError;
    use crate::services::holdings_service::HoldingsService;
    use crate::services::snapshot_service::SnapshotService;
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

    fn create_service(pool: &SqlitePool) -> SnapshotService {
        SnapshotService::new(
            Arc::new(SnapshotRepository::new(pool.clone())),
            Arc::new(AccountRepository::new(pool.clone())),
            create_holdings_service(pool),
        )
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
    async fn test_create_snapshot_empty_account() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Empty Account").await;
        let service = create_service(&pool);

        let snapshot_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let snapshot = service
            .create_snapshot(&account_id, snapshot_date, Some("Monthly"))
            .await
            .expect("Failed to create snapshot");

        assert_eq!(snapshot.account_id, account_id);
        assert_eq!(snapshot.snapshot_date, snapshot_date);
        assert!(snapshot.positions.is_empty());
        assert_eq!(snapshot.cash_total_account_currency, Decimal::ZERO);
        assert_eq!(snapshot.cash_total_base_currency, Decimal::ZERO);
        assert_eq!(snapshot.cost_basis, Decimal::ZERO);
    }

    #[tokio::test]
    async fn test_create_snapshot_with_holdings() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Test Account").await;
        let asset_id = create_asset(&pool).await;
        create_lot(&pool, &account_id, &asset_id, "10", "100").await;

        let service = create_service(&pool);

        let snapshot_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let snapshot = service
            .create_snapshot(&account_id, snapshot_date, Some("Monthly"))
            .await
            .expect("Failed to create snapshot");

        assert_eq!(snapshot.positions.len(), 1);

        let position = &snapshot.positions[0];
        assert_eq!(position.asset_id, asset_id);
        assert_eq!(position.quantity, dec("10"));
        // cost basis 1000 / quantity 10 = 100
        assert_eq!(position.average_cost, dec("100"));
        assert_eq!(position.total_cost_basis, dec("1000"));
    }

    #[tokio::test]
    async fn test_get_snapshot() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Test Account").await;
        let asset_id = create_asset(&pool).await;
        create_lot(&pool, &account_id, &asset_id, "10", "100").await;

        let service = create_service(&pool);

        let snapshot_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let created = service
            .create_snapshot(&account_id, snapshot_date, None)
            .await
            .expect("Failed to create snapshot");

        let retrieved = service
            .get_snapshot(&created.id)
            .await
            .expect("Failed to get snapshot")
            .expect("Snapshot should exist");

        assert_eq!(retrieved.id, created.id);
        assert_eq!(retrieved.account_id, account_id);
        assert_eq!(retrieved.positions.len(), 1);
        assert_eq!(retrieved.positions[0].quantity, dec("10"));
    }

    #[tokio::test]
    async fn test_get_snapshot_not_found() {
        let pool = setup_test_db().await;
        let service = create_service(&pool);

        let result = service
            .get_snapshot("non-existent-id")
            .await
            .expect("Failed to get snapshot");

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_list_snapshots() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Test Account").await;
        let service = create_service(&pool);

        let snapshot_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        service
            .create_snapshot(&account_id, snapshot_date, Some("1"))
            .await
            .expect("Failed to create snapshot 1");
        service
            .create_snapshot(&account_id, snapshot_date, Some("2"))
            .await
            .expect("Failed to create snapshot 2");

        let snapshots = service
            .list_snapshots(&account_id)
            .await
            .expect("Failed to list snapshots");

        assert_eq!(snapshots.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_snapshot() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Test Account").await;
        let service = create_service(&pool);

        let snapshot_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let created = service
            .create_snapshot(&account_id, snapshot_date, None)
            .await
            .expect("Failed to create snapshot");

        service
            .delete_snapshot(&created.id)
            .await
            .expect("Failed to delete snapshot");

        let retrieved = service.get_snapshot(&created.id).await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_create_snapshot_account_not_found() {
        let pool = setup_test_db().await;
        let service = create_service(&pool);

        let snapshot_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let result = service
            .create_snapshot("non-existent-id", snapshot_date, None)
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::NotFound(msg) => assert!(msg.contains("non-existent-id")),
            _ => panic!("Expected NotFound error"),
        }
    }
}
