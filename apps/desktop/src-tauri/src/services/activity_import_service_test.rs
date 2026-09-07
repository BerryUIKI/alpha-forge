// Tests for ActivityImportService.
//
// Covers generic CSV parsing, IBKR statement parsing, asset auto-creation,
// activity creation, and FIFO lot opening on buys.

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
    use crate::database::repositories::lot_repository::LotRepository;
    use crate::database::repositories::test_support::setup_test_db;
    use crate::services::activity_import_service::ActivityImportService;

    #[tokio::test]
    async fn test_import_generic_csv() {
        let pool = setup_test_db().await;
        let account_repo = Arc::new(AccountRepository::new(pool.clone()));
        let asset_repo = Arc::new(AssetRepository::new(pool.clone()));
        let activity_repo = Arc::new(ActivityRepository::new(pool.clone()));
        let import_run_repo = Arc::new(ImportRunRepository::new(pool.clone()));
        let lot_repo = Arc::new(LotRepository::new(pool.clone()));

        let service = ActivityImportService::new(
            account_repo.clone(),
            asset_repo.clone(),
            activity_repo.clone(),
            import_run_repo.clone(),
            lot_repo.clone(),
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

        let run = service
            .import_csv(&account.id, "GENERIC", csv_data)
            .await
            .expect("import generic csv");

        assert_eq!(run.status, "PROCESSING");

        // Verify activities created
        let activities = activity_repo
            .list_by_account(&account.id)
            .await
            .expect("list activities");
        assert_eq!(activities.len(), 2);

        // Verify lot created for the buy activity
        let lots = lot_repo
            .list_open_by_account(&account.id)
            .await
            .expect("list lots");
        assert_eq!(lots.len(), 1);
        assert_eq!(lots[0].original_quantity, Decimal::from(10));
    }

    #[tokio::test]
    async fn test_import_ibkr_csv() {
        let pool = setup_test_db().await;
        let account_repo = Arc::new(AccountRepository::new(pool.clone()));
        let asset_repo = Arc::new(AssetRepository::new(pool.clone()));
        let activity_repo = Arc::new(ActivityRepository::new(pool.clone()));
        let import_run_repo = Arc::new(ImportRunRepository::new(pool.clone()));
        let lot_repo = Arc::new(LotRepository::new(pool.clone()));

        let service = ActivityImportService::new(
            account_repo.clone(),
            asset_repo.clone(),
            activity_repo.clone(),
            import_run_repo.clone(),
            lot_repo.clone(),
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

        let activities = activity_repo
            .list_by_account(&account.id)
            .await
            .expect("list activities");
        assert_eq!(activities.len(), 2);
    }
}
