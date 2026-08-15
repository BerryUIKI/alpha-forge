// Tests for AllocationService.
//
// Covers empty accounts, taxonomy-based allocation, all-scope aggregation,
// constraint checking, and target weights.

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::NaiveDate;
    use rust_decimal::Decimal;
    use sqlx::SqlitePool;

    use crate::database::repositories::account_repository::AccountRepository;
    use crate::database::repositories::allocation_target_repository::AllocationTargetRepository;
    use crate::database::repositories::asset_repository::{AssetRepository, QuoteRepository};
    use crate::database::repositories::lot_repository::{LotDisposalRepository, LotRepository};
    use crate::database::repositories::taxonomy_repository::TaxonomyRepository;
    use crate::database::repositories::test_support::setup_test_db;
    use crate::services::allocation_service::AllocationService;
    use crate::services::holdings_service::HoldingsService;
    use domain::financial::{
        AccountType, AllocationTargetWeightInput, AssetKind, AssetTaxonomyAssignmentInput,
        CostBasisMethod, CreateAccountInput, CreateAllocationTargetInput, CreateAssetInput,
        CreateLotInput, CreateTaxonomyCategoryInput, CreateTaxonomyInput, InstrumentType,
        QuoteMode, ScopeType, TrackingMode,
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

    fn create_service(pool: &SqlitePool) -> AllocationService {
        AllocationService::new(
            Arc::new(TaxonomyRepository::new(pool.clone())),
            Arc::new(AllocationTargetRepository::new(pool.clone())),
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

    async fn create_taxonomy(pool: &SqlitePool, name: &str) -> (String, String) {
        let repo = TaxonomyRepository::new(pool.clone());
        let taxonomy = repo
            .create(CreateTaxonomyInput {
                name: name.to_string(),
                color: "#ff0000".to_string(),
                description: None,
                is_system: false,
                is_single_select: true,
                sort_order: 0,
            })
            .await
            .expect("Failed to create taxonomy");

        let category = repo
            .create_category(CreateTaxonomyCategoryInput {
                taxonomy_id: taxonomy.id.clone(),
                parent_id: None,
                name: "Technology".to_string(),
                key: "tech".to_string(),
                color: "#00ff00".to_string(),
                description: None,
                sort_order: 0,
            })
            .await
            .expect("Failed to create category");

        (taxonomy.id, category.id)
    }

    async fn assign_asset_to_category(
        pool: &SqlitePool,
        asset_id: &str,
        taxonomy_id: &str,
        category_id: &str,
    ) {
        let repo = TaxonomyRepository::new(pool.clone());
        repo.assign_asset(AssetTaxonomyAssignmentInput {
            asset_id: asset_id.to_string(),
            taxonomy_id: taxonomy_id.to_string(),
            category_id: category_id.to_string(),
            weight: 10000,
            source: "manual".to_string(),
        })
        .await
        .expect("Failed to assign asset to category");
    }

    #[tokio::test]
    async fn test_get_allocation_empty_account_scope() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Empty Account").await;
        let service = create_service(&pool);

        let as_of_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let allocation = service
            .get_allocation(ScopeType::Account, Some(&account_id), as_of_date)
            .await
            .expect("Failed to get allocation");

        assert_eq!(allocation.scope_type, ScopeType::Account);
        assert_eq!(allocation.scope_id, Some(account_id));
        assert_eq!(allocation.total_market_value, Decimal::ZERO);
        assert_eq!(allocation.total_market_value_base, Decimal::ZERO);
        assert!(allocation.categories.is_empty());
        assert_eq!(allocation.unassigned_market_value, Decimal::ZERO);
        assert_eq!(allocation.unassigned_market_value_base, Decimal::ZERO);
    }

    #[tokio::test]
    async fn test_get_allocation_with_taxonomy() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Test Account").await;
        let asset_id = create_asset(&pool).await;
        create_lot(&pool, &account_id, &asset_id, "10", "100").await;

        let (taxonomy_id, category_id) = create_taxonomy(&pool, "Sector").await;
        assign_asset_to_category(&pool, &asset_id, &taxonomy_id, &category_id).await;

        let service = create_service(&pool);

        let as_of_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let allocation = service
            .get_allocation(ScopeType::Account, Some(&account_id), as_of_date)
            .await
            .expect("Failed to get allocation");

        // 10 shares @ $100 = $1000 market value (fallback to cost basis)
        assert_eq!(allocation.total_market_value, dec("1000"));
        assert_eq!(allocation.categories.len(), 1);

        let cat = &allocation.categories[0];
        assert_eq!(cat.category_name, "Technology");
        assert_eq!(cat.taxonomy_name, "Sector");
        assert_eq!(cat.actual_bps, 10000); // 100%
        assert_eq!(cat.market_value, dec("1000"));
        assert_eq!(allocation.unassigned_market_value, Decimal::ZERO);
        assert_eq!(allocation.unassigned_market_value_base, Decimal::ZERO);
    }

    #[tokio::test]
    async fn test_get_allocation_all_scope() {
        let pool = setup_test_db().await;
        let account1_id = create_account(&pool, "Account 1").await;
        let account2_id = create_account(&pool, "Account 2").await;
        let asset_id = create_asset(&pool).await;

        // Account 1: 10 shares @ $100
        create_lot(&pool, &account1_id, &asset_id, "10", "100").await;
        // Account 2: 5 shares @ $100
        create_lot(&pool, &account2_id, &asset_id, "5", "100").await;

        let (taxonomy_id, category_id) = create_taxonomy(&pool, "Sector").await;
        assign_asset_to_category(&pool, &asset_id, &taxonomy_id, &category_id).await;

        let service = create_service(&pool);

        let as_of_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let allocation = service
            .get_allocation(ScopeType::All, None, as_of_date)
            .await
            .expect("Failed to get allocation");

        // Total market value = 10*100 + 5*100 = 1500
        assert_eq!(allocation.total_market_value, dec("1500"));
        assert_eq!(allocation.categories.len(), 1);

        let cat = &allocation.categories[0];
        assert_eq!(cat.actual_bps, 10000); // 100% in Technology
        assert_eq!(cat.market_value, dec("1500"));
    }

    #[tokio::test]
    async fn test_check_constraints_no_targets() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Test Account").await;
        let service = create_service(&pool);

        let as_of_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let constraints = service
            .check_constraints(ScopeType::Account, Some(&account_id), as_of_date)
            .await
            .expect("Failed to check constraints");

        assert!(constraints.is_empty());
    }

    #[tokio::test]
    async fn test_allocation_with_target_weights() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Test Account").await;
        let asset_id = create_asset(&pool).await;
        create_lot(&pool, &account_id, &asset_id, "10", "100").await;

        let (taxonomy_id, category_id) = create_taxonomy(&pool, "Sector").await;
        assign_asset_to_category(&pool, &asset_id, &taxonomy_id, &category_id).await;

        // Create an allocation target with 8000 bps (80%) weight for the category
        let target_repo = AllocationTargetRepository::new(pool.clone());
        let target = target_repo
            .create(CreateAllocationTargetInput {
                name: "80% Tech Target".to_string(),
                scope_type: ScopeType::Account,
                scope_id: Some(account_id.clone()),
                taxonomy_id: taxonomy_id.clone(),
                trigger_type: "manual".to_string(),
                drift_band_bps: 5000,
                rebalance_goal: "nearest_band".to_string(),
                min_trade_amount: dec("0"),
                whole_shares_only: false,
                allow_sells: true,
                max_turnover_bps: None,
            })
            .await
            .expect("Failed to create allocation target");

        target_repo
            .add_weight(AllocationTargetWeightInput {
                target_id: target.id.clone(),
                taxonomy_id: taxonomy_id.clone(),
                category_id: category_id.clone(),
                target_bps: 8000,
                is_locked: false,
                is_required: false,
            })
            .await
            .expect("Failed to add weight");

        let service = create_service(&pool);

        let as_of_date = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let allocation = service
            .get_allocation(ScopeType::Account, Some(&account_id), as_of_date)
            .await
            .expect("Failed to get allocation");

        assert_eq!(allocation.categories.len(), 1);

        let cat = &allocation.categories[0];
        assert_eq!(cat.actual_bps, 10000); // 100% actual
        assert_eq!(cat.target_bps, Some(8000)); // 80% target
        assert_eq!(cat.difference_bps, 2000); // 10000 - 8000
                                              // The hardcoded drift band in the service is 500 bps (5%).
                                              // 2000 > 500, so within_drift is false.
        assert!(!cat.within_drift);
    }
}
