// Tests for ActivityImportService.
//
// Covers generic CSV parsing, IBKR statement parsing, asset auto-creation,
// FIFO lot creation on Buys, FIFO lot disposal on Sells, deterministic
// deduplication on retry, and terminal ImportRun audit states.

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use domain::financial::{AccountType, CreateAccountInput, TrackingMode};
    use rust_decimal::Decimal;

    use crate::database::repositories::account_repository::AccountRepository;
    use crate::database::repositories::activity_repository::{
        ActivityRepository, ImportRunRepository,
    };
    use crate::database::repositories::asset_repository::AssetRepository;
    use crate::database::repositories::lot_repository::{LotDisposalRepository, LotRepository};
    use crate::database::repositories::test_support::setup_test_db;
    use crate::services::activity_import_service::{ActivityImportService, ImportRunSummary};
    use crate::services::activity_service::ActivityService;

    #[tokio::test]
    async fn test_import_generic_csv_and_idempotent_reimport() {
        let pool = setup_test_db().await;
        let account_repo = Arc::new(AccountRepository::new(pool.clone()));
        let asset_repo = Arc::new(AssetRepository::new(pool.clone()));
        let activity_repo = Arc::new(ActivityRepository::new(pool.clone()));
        let import_run_repo = Arc::new(ImportRunRepository::new(pool.clone()));
        let lot_repo = Arc::new(LotRepository::new(pool.clone()));
        let disposal_repo = Arc::new(LotDisposalRepository::new(pool.clone()));

        let activity_service = Arc::new(ActivityService::new(
            pool.clone(),
            activity_repo.clone(),
            account_repo.clone(),
            asset_repo.clone(),
            lot_repo.clone(),
            disposal_repo.clone(),
        ));

        let service = ActivityImportService::new(
            account_repo.clone(),
            asset_repo.clone(),
            activity_repo.clone(),
            import_run_repo.clone(),
            lot_repo.clone(),
            activity_service.clone(),
        );

        let account = account_repo
            .create(CreateAccountInput {
                workspace_id: None,
                name: "Test Trading Account".to_string(),
                account_type: AccountType::Securities,
                group_name: None,
                currency: "USD".to_string(),
                is_default: false,
                platform_id: None,
                account_number: None,
                tracking_mode: TrackingMode::Transactions,
            })
            .await
            .expect("create account");

        let csv_data = "date,type,symbol,quantity,price,amount,fee,currency,notes\n\
                        2026-08-01,BUY,NVDA,10,120.00,1200.00,1.50,USD,Initial purchase\n\
                        2026-08-05,SELL,NVDA,5,130.00,650.00,1.00,USD,Partial take-profit";

        // First import run: creates 1 buy and 1 sell, adjusts lot from 10 down to 5
        let run = service
            .import_csv(&account.id, "GENERIC", csv_data)
            .await
            .expect("import generic csv");

        assert_eq!(run.status, "COMPLETED");
        assert!(run.finished_at.is_some());
        assert!(run.applied_at.is_some());

        let summary_text = run.summary.expect("summary should be present");
        let summary: ImportRunSummary =
            serde_json::from_str(&summary_text).expect("valid summary json");
        assert_eq!(summary.total_rows, 2);
        assert_eq!(summary.created_count, 2);
        assert_eq!(summary.skipped_count, 0);
        assert_eq!(summary.failed_count, 0);

        // Verify activities created
        let activities = activity_repo
            .list_by_account(&account.id)
            .await
            .expect("list activities");
        assert_eq!(activities.len(), 2);

        // Verify lot created for the buy activity and reduced by the sell activity
        let lots = lot_repo
            .list_open_by_account(&account.id)
            .await
            .expect("list lots");
        assert_eq!(lots.len(), 1);
        assert_eq!(lots[0].original_quantity, Decimal::from(10));
        assert_eq!(lots[0].remaining_quantity, Decimal::from(5));

        // Verify disposals created
        let disposals = disposal_repo
            .list_by_account(&account.id)
            .await
            .expect("list disposals");
        assert_eq!(disposals.len(), 1);
        assert_eq!(disposals[0].quantity, Decimal::from(5));

        // Re-import identical CSV: must be idempotent, skip rows, not duplicate lots or disposals
        let reimport_run = service
            .import_csv(&account.id, "GENERIC", csv_data)
            .await
            .expect("reimport generic csv");

        assert_eq!(reimport_run.status, "COMPLETED");
        let reimport_summary_text = reimport_run.summary.expect("summary should be present");
        let reimport_summary: ImportRunSummary =
            serde_json::from_str(&reimport_summary_text).expect("valid summary json");
        assert_eq!(reimport_summary.total_rows, 2);
        assert_eq!(reimport_summary.created_count, 0);
        assert_eq!(reimport_summary.skipped_count, 2);
        assert_eq!(reimport_summary.failed_count, 0);

        // Still exactly 2 activities, 1 lot with 5 remaining, 1 disposal
        let activities_after = activity_repo
            .list_by_account(&account.id)
            .await
            .expect("list activities");
        assert_eq!(activities_after.len(), 2);

        let lots_after = lot_repo
            .list_open_by_account(&account.id)
            .await
            .expect("list lots");
        assert_eq!(lots_after.len(), 1);
        assert_eq!(lots_after[0].remaining_quantity, Decimal::from(5));

        let disposals_after = disposal_repo
            .list_by_account(&account.id)
            .await
            .expect("list disposals");
        assert_eq!(disposals_after.len(), 1);
    }

    #[tokio::test]
    async fn test_import_ibkr_csv() {
        let pool = setup_test_db().await;
        let account_repo = Arc::new(AccountRepository::new(pool.clone()));
        let asset_repo = Arc::new(AssetRepository::new(pool.clone()));
        let activity_repo = Arc::new(ActivityRepository::new(pool.clone()));
        let import_run_repo = Arc::new(ImportRunRepository::new(pool.clone()));
        let lot_repo = Arc::new(LotRepository::new(pool.clone()));
        let disposal_repo = Arc::new(LotDisposalRepository::new(pool.clone()));

        let activity_service = Arc::new(ActivityService::new(
            pool.clone(),
            activity_repo.clone(),
            account_repo.clone(),
            asset_repo.clone(),
            lot_repo.clone(),
            disposal_repo.clone(),
        ));

        let service = ActivityImportService::new(
            account_repo.clone(),
            asset_repo.clone(),
            activity_repo.clone(),
            import_run_repo.clone(),
            lot_repo.clone(),
            activity_service.clone(),
        );

        let account = account_repo
            .create(CreateAccountInput {
                workspace_id: None,
                name: "IBKR Pro Account".to_string(),
                account_type: AccountType::Securities,
                group_name: None,
                currency: "USD".to_string(),
                is_default: false,
                platform_id: None,
                account_number: Some("U1234567".to_string()),
                tracking_mode: TrackingMode::Transactions,
            })
            .await
            .expect("create account");

        let ibkr_csv = "Statement,Header,Field Name,Field Value\n\
                        Trades,Header,DataDiscriminator,Asset Category,Currency,Symbol,Date/Time,Quantity,T. Price,C. Price,Proceeds,Comm/Fee\n\
                        Trades,Data,Order,Stocks,USD,GOOGL,2026-08-10, 14:30:00,15,180.00,180.00,-2700.00,-1.00\n\
                        Trades,Data,Order,Stocks,USD,GOOGL,2026-08-12, 11:00:00,-5,185.00,185.00,925.00,-1.00";

        let run = service
            .import_csv(&account.id, "IBKR", ibkr_csv)
            .await
            .expect("import ibkr csv");

        assert_eq!(run.account_id, account.id);
        assert_eq!(run.status, "COMPLETED");

        let activities = activity_repo
            .list_by_account(&account.id)
            .await
            .expect("list activities");
        assert_eq!(activities.len(), 2);

        let lots = lot_repo
            .list_open_by_account(&account.id)
            .await
            .expect("list lots");
        assert_eq!(lots.len(), 1);
        assert_eq!(lots[0].remaining_quantity, Decimal::from(10));
    }

    #[tokio::test]
    async fn test_import_insufficient_lots_records_failure_status() {
        let pool = setup_test_db().await;
        let account_repo = Arc::new(AccountRepository::new(pool.clone()));
        let asset_repo = Arc::new(AssetRepository::new(pool.clone()));
        let activity_repo = Arc::new(ActivityRepository::new(pool.clone()));
        let import_run_repo = Arc::new(ImportRunRepository::new(pool.clone()));
        let lot_repo = Arc::new(LotRepository::new(pool.clone()));
        let disposal_repo = Arc::new(LotDisposalRepository::new(pool.clone()));

        let activity_service = Arc::new(ActivityService::new(
            pool.clone(),
            activity_repo.clone(),
            account_repo.clone(),
            asset_repo.clone(),
            lot_repo.clone(),
            disposal_repo.clone(),
        ));

        let service = ActivityImportService::new(
            account_repo.clone(),
            asset_repo.clone(),
            activity_repo.clone(),
            import_run_repo.clone(),
            lot_repo.clone(),
            activity_service.clone(),
        );

        let account = account_repo
            .create(CreateAccountInput {
                workspace_id: None,
                name: "Short Test Account".to_string(),
                account_type: AccountType::Securities,
                group_name: None,
                currency: "USD".to_string(),
                is_default: false,
                platform_id: None,
                account_number: None,
                tracking_mode: TrackingMode::Transactions,
            })
            .await
            .expect("create account");

        // Attempting to sell shares without prior buy inventory
        let bad_csv = "date,type,symbol,quantity,price,amount,fee,currency,notes\n\
                       2026-08-05,SELL,TSLA,5,200.00,1000.00,1.00,USD,Naked sell without lots";

        let run = service
            .import_csv(&account.id, "GENERIC", bad_csv)
            .await
            .expect("import executes and concludes with failed status");

        assert_eq!(run.status, "FAILED");
        assert!(run.error.is_some());

        let summary: ImportRunSummary =
            serde_json::from_str(&run.summary.unwrap()).expect("summary json");
        assert_eq!(summary.failed_count, 1);
        assert_eq!(summary.created_count, 0);
        assert_eq!(summary.errors.len(), 1);
    }

    #[tokio::test]
    async fn test_import_exceeding_batch_budget_returns_validation_error() {
        let pool = setup_test_db().await;
        let account_repo = Arc::new(AccountRepository::new(pool.clone()));
        let asset_repo = Arc::new(AssetRepository::new(pool.clone()));
        let activity_repo = Arc::new(ActivityRepository::new(pool.clone()));
        let import_run_repo = Arc::new(ImportRunRepository::new(pool.clone()));
        let lot_repo = Arc::new(LotRepository::new(pool.clone()));
        let disposal_repo = Arc::new(LotDisposalRepository::new(pool.clone()));

        let activity_service = Arc::new(ActivityService::new(
            pool.clone(),
            activity_repo.clone(),
            account_repo.clone(),
            asset_repo.clone(),
            lot_repo.clone(),
            disposal_repo.clone(),
        ));

        let service = ActivityImportService::new(
            account_repo.clone(),
            asset_repo.clone(),
            activity_repo.clone(),
            import_run_repo.clone(),
            lot_repo.clone(),
            activity_service.clone(),
        );

        let account = account_repo
            .create(CreateAccountInput {
                workspace_id: None,
                name: "Budget Test Account".to_string(),
                account_type: AccountType::Securities,
                group_name: None,
                currency: "USD".to_string(),
                is_default: false,
                platform_id: None,
                account_number: None,
                tracking_mode: TrackingMode::Transactions,
            })
            .await
            .expect("create account");

        let mut huge_csv =
            String::from("date,type,symbol,quantity,price,amount,fee,currency,notes\n");
        for _ in 0..10_001 {
            huge_csv.push_str("2026-08-01,BUY,NVDA,1,120.00,120.00,0.00,USD,Excessive batch\n");
        }

        let err = service
            .import_csv(&account.id, "GENERIC", &huge_csv)
            .await
            .expect_err("should reject batch over 10,000 rows");

        assert!(matches!(err, crate::error::AppError::Validation(_)));
        assert!(err.to_string().contains("exceeds maximum batch budget"));
    }
}
