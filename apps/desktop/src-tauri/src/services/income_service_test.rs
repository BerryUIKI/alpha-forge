// Tests for IncomeService.
//
// Covers empty income, single activity, multiple activities across years,
// and YoY growth calculation.

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::NaiveDate;
    use rust_decimal::Decimal;
    use sqlx::SqlitePool;

    use crate::database::repositories::account_repository::AccountRepository;
    use crate::database::repositories::activity_repository::ActivityRepository;
    use crate::database::repositories::test_support::setup_test_db;
    use crate::services::income_service::IncomeService;
    use domain::financial::{
        AccountType, ActivityStatus, ActivityType, CreateAccountInput, CreateActivityInput,
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

    fn create_service(pool: &SqlitePool) -> IncomeService {
        IncomeService::new(
            Arc::new(ActivityRepository::new(pool.clone())),
            Arc::new(AccountRepository::new(pool.clone())),
        )
    }

    #[tokio::test]
    async fn empty_income_returns_empty_vec() {
        let pool = setup_test_db().await;
        let _account_id = create_account(&pool, "empty-income").await;
        let service = create_service(&pool);

        let result = service
            .get_income_summary(None)
            .await
            .expect("should succeed");
        assert!(
            result.is_empty(),
            "no activities should produce no summaries"
        );
    }

    #[tokio::test]
    async fn single_dividend_produces_one_period_summary() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "div-account").await;
        let service = create_service(&pool);

        // Create a dividend activity
        let activity_repo = ActivityRepository::new(pool.clone());
        activity_repo
            .create(CreateActivityInput {
                account_id: account_id.clone(),
                asset_id: None,
                activity_type: ActivityType::Dividend,
                activity_type_override: None,
                source_type: None,
                subtype: None,
                status: ActivityStatus::Posted,
                activity_date: NaiveDate::from_ymd_opt(2026, 6, 15).expect("valid date"),
                settlement_date: None,
                quantity: None,
                unit_price: None,
                amount: Some(dec("150.00")),
                fee: None,
                tax: None,
                currency: "USD".to_string(),
                fx_rate: None,
                notes: None,
                metadata: None,
                source_system: None,
                source_record_id: None,
                source_group_id: None,
                idempotency_key: None,
                import_run_id: None,
            })
            .await
            .expect("Failed to create activity");

        let result = service
            .get_income_summary(None)
            .await
            .expect("should succeed");
        assert!(!result.is_empty(), "should have at least one summary");

        // Should have at least the ALL period
        let all = result.iter().find(|s| s.period == "ALL");
        assert!(all.is_some(), "should have ALL period summary");
        let all = all.unwrap();
        assert_eq!(all.total_income, dec("150.00"));
        assert_eq!(all.currency, "USD");
    }

    #[tokio::test]
    async fn multiple_activities_across_years_includes_yoy_growth() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "yoy-account").await;
        let service = create_service(&pool);
        let activity_repo = ActivityRepository::new(pool.clone());

        // Create a dividend in 2024
        activity_repo
            .create(CreateActivityInput {
                account_id: account_id.clone(),
                asset_id: None,
                activity_type: ActivityType::Dividend,
                activity_type_override: None,
                source_type: None,
                subtype: None,
                status: ActivityStatus::Posted,
                activity_date: NaiveDate::from_ymd_opt(2024, 6, 15).expect("valid date"),
                settlement_date: None,
                quantity: None,
                unit_price: None,
                amount: Some(dec("100.00")),
                fee: None,
                tax: None,
                currency: "USD".to_string(),
                fx_rate: None,
                notes: None,
                metadata: None,
                source_system: None,
                source_record_id: None,
                source_group_id: None,
                idempotency_key: None,
                import_run_id: None,
            })
            .await
            .expect("Failed to create activity");

        // Create a dividend in 2025 (last year)
        activity_repo
            .create(CreateActivityInput {
                account_id: account_id.clone(),
                asset_id: None,
                activity_type: ActivityType::Dividend,
                activity_type_override: None,
                source_type: None,
                subtype: None,
                status: ActivityStatus::Posted,
                activity_date: NaiveDate::from_ymd_opt(2025, 6, 15).expect("valid date"),
                settlement_date: None,
                quantity: None,
                unit_price: None,
                amount: Some(dec("200.00")),
                fee: None,
                tax: None,
                currency: "USD".to_string(),
                fx_rate: None,
                notes: None,
                metadata: None,
                source_system: None,
                source_record_id: None,
                source_group_id: None,
                idempotency_key: None,
                import_run_id: None,
            })
            .await
            .expect("Failed to create activity");

        // Create a dividend in 2026 (current year / YTD)
        activity_repo
            .create(CreateActivityInput {
                account_id: account_id.clone(),
                asset_id: None,
                activity_type: ActivityType::Dividend,
                activity_type_override: None,
                source_type: None,
                subtype: None,
                status: ActivityStatus::Posted,
                activity_date: NaiveDate::from_ymd_opt(2026, 6, 15).expect("valid date"),
                settlement_date: None,
                quantity: None,
                unit_price: None,
                amount: Some(dec("300.00")),
                fee: None,
                tax: None,
                currency: "USD".to_string(),
                fx_rate: None,
                notes: None,
                metadata: None,
                source_system: None,
                source_record_id: None,
                source_group_id: None,
                idempotency_key: None,
                import_run_id: None,
            })
            .await
            .expect("Failed to create activity");

        let result = service
            .get_income_summary(None)
            .await
            .expect("should succeed");
        assert!(!result.is_empty(), "should have summaries");

        // ALL period should have sum of all dividends
        let all = result.iter().find(|s| s.period == "ALL");
        assert!(all.is_some(), "should have ALL period summary");
        let all = all.unwrap();
        assert_eq!(all.total_income, dec("600.00"));

        // YTD should have the 2026 dividend
        let ytd = result.iter().find(|s| s.period == "YTD");
        assert!(ytd.is_some(), "should have YTD summary");
        let ytd = ytd.unwrap();
        assert_eq!(ytd.total_income, dec("300.00"));

        // LAST_YEAR should have the 2025 dividend
        let last_year = result.iter().find(|s| s.period == "LAST_YEAR");
        assert!(last_year.is_some(), "should have LAST_YEAR summary");
        let last_year = last_year.unwrap();
        assert_eq!(last_year.total_income, dec("200.00"));

        // YTD should have YoY growth (300-200)/200 = 0.50
        assert!(ytd.yoy_growth.is_some(), "YTD should have YoY growth");
        assert_eq!(ytd.yoy_growth.unwrap(), dec("0.50"));

        // Last year should have YoY growth (200-100)/100 = 1.00
        assert!(
            last_year.yoy_growth.is_some(),
            "LAST_YEAR should have YoY growth"
        );
        assert_eq!(last_year.yoy_growth.unwrap(), dec("1.00"));
    }

    #[tokio::test]
    async fn income_by_type_is_populated() {
        let pool = setup_test_db().await;
        let account_id = create_account(&pool, "type-account").await;
        let service = create_service(&pool);
        let activity_repo = ActivityRepository::new(pool.clone());

        // Create a dividend
        activity_repo
            .create(CreateActivityInput {
                account_id: account_id.clone(),
                asset_id: None,
                activity_type: ActivityType::Dividend,
                activity_type_override: None,
                source_type: None,
                subtype: None,
                status: ActivityStatus::Posted,
                activity_date: NaiveDate::from_ymd_opt(2026, 1, 15).expect("valid date"),
                settlement_date: None,
                quantity: None,
                unit_price: None,
                amount: Some(dec("100.00")),
                fee: None,
                tax: None,
                currency: "USD".to_string(),
                fx_rate: None,
                notes: None,
                metadata: None,
                source_system: None,
                source_record_id: None,
                source_group_id: None,
                idempotency_key: None,
                import_run_id: None,
            })
            .await
            .expect("Failed to create activity");

        // Create an interest payment
        activity_repo
            .create(CreateActivityInput {
                account_id: account_id.clone(),
                asset_id: None,
                activity_type: ActivityType::Interest,
                activity_type_override: None,
                source_type: None,
                subtype: None,
                status: ActivityStatus::Posted,
                activity_date: NaiveDate::from_ymd_opt(2026, 2, 15).expect("valid date"),
                settlement_date: None,
                quantity: None,
                unit_price: None,
                amount: Some(dec("50.00")),
                fee: None,
                tax: None,
                currency: "USD".to_string(),
                fx_rate: None,
                notes: None,
                metadata: None,
                source_system: None,
                source_record_id: None,
                source_group_id: None,
                idempotency_key: None,
                import_run_id: None,
            })
            .await
            .expect("Failed to create activity");

        let result = service
            .get_income_summary(None)
            .await
            .expect("should succeed");

        let all = result.iter().find(|s| s.period == "ALL").unwrap();
        assert_eq!(all.by_type.len(), 2, "should have two income types");
        assert_eq!(
            *all.by_type.get("DIVIDEND").unwrap(),
            dec("100.00"),
            "dividend income"
        );
        assert_eq!(
            *all.by_type.get("INTEREST").unwrap(),
            dec("50.00"),
            "interest income"
        );
    }

    #[tokio::test]
    async fn scoped_to_specific_accounts() {
        let pool = setup_test_db().await;
        let account_a = create_account(&pool, "account-a").await;
        let account_b = create_account(&pool, "account-b").await;
        let service = create_service(&pool);
        let activity_repo = ActivityRepository::new(pool.clone());

        // Dividend in account A
        activity_repo
            .create(CreateActivityInput {
                account_id: account_a.clone(),
                asset_id: None,
                activity_type: ActivityType::Dividend,
                activity_type_override: None,
                source_type: None,
                subtype: None,
                status: ActivityStatus::Posted,
                activity_date: NaiveDate::from_ymd_opt(2026, 3, 15).expect("valid date"),
                settlement_date: None,
                quantity: None,
                unit_price: None,
                amount: Some(dec("200.00")),
                fee: None,
                tax: None,
                currency: "USD".to_string(),
                fx_rate: None,
                notes: None,
                metadata: None,
                source_system: None,
                source_record_id: None,
                source_group_id: None,
                idempotency_key: None,
                import_run_id: None,
            })
            .await
            .expect("Failed to create activity");

        // Dividend in account B
        activity_repo
            .create(CreateActivityInput {
                account_id: account_b.clone(),
                asset_id: None,
                activity_type: ActivityType::Dividend,
                activity_type_override: None,
                source_type: None,
                subtype: None,
                status: ActivityStatus::Posted,
                activity_date: NaiveDate::from_ymd_opt(2026, 4, 15).expect("valid date"),
                settlement_date: None,
                quantity: None,
                unit_price: None,
                amount: Some(dec("300.00")),
                fee: None,
                tax: None,
                currency: "USD".to_string(),
                fx_rate: None,
                notes: None,
                metadata: None,
                source_system: None,
                source_record_id: None,
                source_group_id: None,
                idempotency_key: None,
                import_run_id: None,
            })
            .await
            .expect("Failed to create activity");

        // Scoped to account A only
        let account_ids = vec![account_a.clone()];
        let result = service
            .get_income_summary(Some(&account_ids))
            .await
            .expect("should succeed");

        let all = result.iter().find(|s| s.period == "ALL").unwrap();
        assert_eq!(all.total_income, dec("200.00"), "only account A income");
    }
}
