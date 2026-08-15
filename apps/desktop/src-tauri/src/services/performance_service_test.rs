// Tests for PerformanceService.
//
// Covers compute_summary (insufficient data, single point, two points, account
// not found) and get_time_series. Valuation rows are seeded directly through
// ValuationRepository with zero net contribution so the performance math is
// deterministic (return is then purely the change in total value).

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use rust_decimal::Decimal;
    use std::sync::Arc;

    use crate::database::repositories::account_repository::AccountRepository;
    use crate::database::repositories::test_support::setup_test_db;
    use crate::database::repositories::valuation_repository::ValuationRepository;
    use crate::error::AppError;
    use crate::services::performance_service::PerformanceService;
    use domain::financial::{
        AccountType, BasisStatus, CreateAccountInput, ExternalFlowSource, TrackingMode,
        UpsertValuationInput, ValuationStatus,
    };

    fn dec(value: &str) -> Decimal {
        Decimal::from_str_exact(value).expect("valid decimal")
    }

    fn create_performance_service(pool: &sqlx::SqlitePool) -> PerformanceService {
        let valuation_repo = Arc::new(ValuationRepository::new(pool.clone()));
        let account_repo = Arc::new(AccountRepository::new(pool.clone()));
        PerformanceService::new(valuation_repo, account_repo)
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

    async fn upsert_valuation(
        pool: &sqlx::SqlitePool,
        account_id: &str,
        date: NaiveDate,
        total_value: &str,
    ) {
        let valuation_repo = Arc::new(ValuationRepository::new(pool.clone()));
        valuation_repo
            .upsert(UpsertValuationInput {
                account_id: account_id.to_string(),
                valuation_date: date,
                account_currency: "USD".to_string(),
                base_currency: "USD".to_string(),
                fx_rate_to_base: dec("1"),
                cash_balance: dec("0"),
                investment_market_value: dec(total_value),
                total_value: dec(total_value),
                cost_basis: dec("9500"),
                net_contribution: dec("0"),
                cash_balance_base: dec("0"),
                investment_market_value_base: dec(total_value),
                total_value_base: dec(total_value),
                cost_basis_base: dec("9500"),
                net_contribution_base: dec("0"),
                external_inflow_base: Decimal::ZERO,
                external_outflow_base: Decimal::ZERO,
                performance_eligible_value_base: dec(total_value),
                external_flow_source: ExternalFlowSource::NoFlow,
                value_status: ValuationStatus::Complete,
                basis_status: BasisStatus::Complete,
            })
            .await
            .expect("Failed to upsert valuation");
    }

    // ── Tests ───────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_compute_summary_insufficient_data() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "No Data").await;
        let service = create_performance_service(&pool);

        let aug13 = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let aug14 = NaiveDate::from_ymd_opt(2026, 8, 14).expect("valid date");

        let summary = service
            .compute_summary(&account_id, aug13, aug14)
            .await
            .expect("compute_summary should succeed");

        assert_eq!(summary.data_quality, "insufficient_data");
        assert_eq!(summary.total_return_pct, None);
        assert_eq!(summary.xirr_pct, None);
        assert_eq!(summary.twr_pct, None);
        assert_eq!(summary.start_value, Decimal::ZERO);
        assert_eq!(summary.end_value, Decimal::ZERO);
    }

    #[tokio::test]
    async fn test_compute_summary_single_point() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Single Point").await;
        let aug13 = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        upsert_valuation(&pool, &account_id, aug13, "10000").await;

        let service = create_performance_service(&pool);
        let summary = service
            .compute_summary(&account_id, aug13, aug13)
            .await
            .expect("compute_summary should succeed");

        assert_eq!(summary.data_quality, "insufficient_data");
        assert_eq!(summary.total_return_pct, None);
        assert_eq!(summary.start_value, dec("10000"));
        assert_eq!(summary.end_value, dec("10000"));
    }

    #[tokio::test]
    async fn test_compute_summary_with_two_points() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Two Points").await;
        let aug13 = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let aug14 = NaiveDate::from_ymd_opt(2026, 8, 14).expect("valid date");

        // No net contributions: return reflects only the change in total value
        upsert_valuation(&pool, &account_id, aug13, "10000").await;
        upsert_valuation(&pool, &account_id, aug14, "10500").await;

        let service = create_performance_service(&pool);
        let summary = service
            .compute_summary(&account_id, aug13, aug14)
            .await
            .expect("compute_summary should succeed");

        assert_eq!(summary.data_quality, "partial"); // XIRR cannot converge with no cash flows
        assert_eq!(summary.start_value, dec("10000"));
        assert_eq!(summary.end_value, dec("10500"));
        assert_eq!(summary.net_contribution, Decimal::ZERO);
        assert_eq!(summary.total_return_pct, Some(dec("5"))); // (10500 - 10000) / 10000
        assert_eq!(summary.twr_pct, Some(dec("5")));
        assert_eq!(summary.total_gain, dec("500"));
    }

    #[tokio::test]
    async fn test_get_time_series() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "Time Series").await;
        let aug13 = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let aug14 = NaiveDate::from_ymd_opt(2026, 8, 14).expect("valid date");
        let aug15 = NaiveDate::from_ymd_opt(2026, 8, 15).expect("valid date");

        upsert_valuation(&pool, &account_id, aug13, "10000").await;
        upsert_valuation(&pool, &account_id, aug14, "10500").await;
        upsert_valuation(&pool, &account_id, aug15, "11000").await;

        let service = create_performance_service(&pool);
        let points = service
            .get_time_series(&account_id)
            .await
            .expect("get_time_series should succeed");

        assert_eq!(points.len(), 3);

        // First point has no previous value, so no daily return
        assert_eq!(points[0].date, aug13);
        assert_eq!(points[0].total_value, dec("10000"));
        assert_eq!(points[0].daily_return_pct, None);
        assert_eq!(points[0].cumulative_return_pct, Some(Decimal::ZERO));

        assert_eq!(points[1].date, aug14);
        assert_eq!(points[1].total_value, dec("10500"));
        assert_eq!(points[1].daily_return_pct, Some(dec("5")));
        assert_eq!(points[1].cumulative_return_pct, Some(dec("5")));

        assert_eq!(points[2].date, aug15);
        assert_eq!(points[2].total_value, dec("11000"));
        assert_eq!(points[2].cumulative_return_pct, Some(dec("10")));
    }

    #[tokio::test]
    async fn test_compute_summary_account_not_found() {
        let pool = setup_test_db().await;
        let service = create_performance_service(&pool);

        let aug13 = NaiveDate::from_ymd_opt(2026, 8, 13).expect("valid date");
        let aug14 = NaiveDate::from_ymd_opt(2026, 8, 14).expect("valid date");

        let result = service
            .compute_summary("non-existent-id", aug13, aug14)
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::NotFound(msg) => assert!(msg.contains("not found")),
            _ => panic!("Expected NotFound error"),
        }
    }
}
