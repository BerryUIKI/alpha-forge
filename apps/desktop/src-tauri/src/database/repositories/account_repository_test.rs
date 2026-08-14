// Tests for the platform + account financial repositories.

use crate::database::repositories::account_repository::{AccountRepository, PlatformRepository};
use crate::database::repositories::test_support::setup_test_db;
use domain::financial::{AccountType, CreateAccountInput, CreatePlatformInput, TrackingMode};

#[tokio::test]
async fn platform_repository_creates_and_lists() {
    let pool = setup_test_db().await;
    let repo = PlatformRepository::new(pool);

    let platform = repo
        .create(CreatePlatformInput {
            name: Some("Interactive Brokers".to_string()),
            url: "https://www.interactivebrokers.com".to_string(),
            kind: "BROKERAGE".to_string(),
        })
        .await
        .expect("Failed to create platform");

    assert_eq!(platform.name.as_deref(), Some("Interactive Brokers"));
    assert_eq!(platform.kind, "BROKERAGE");

    let listed = repo.list().await.expect("Failed to list platforms");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, platform.id);

    let fetched = repo
        .get(&platform.id)
        .await
        .expect("Failed to get platform");
    assert!(fetched.is_some());
    assert_eq!(fetched.unwrap().url, "https://www.interactivebrokers.com");
}

#[tokio::test]
async fn account_repository_creates_and_lists_by_workspace() {
    let pool = setup_test_db().await;
    // Seed the workspace the account is scoped to (FK to workspaces.id).
    sqlx::query("INSERT INTO workspaces (id, name) VALUES ('ws-1', 'Test Workspace')")
        .execute(&pool)
        .await
        .expect("Failed to seed workspace");
    let repo = AccountRepository::new(pool);

    let account = repo
        .create(CreateAccountInput {
            workspace_id: Some("ws-1".to_string()),
            name: "Main Brokerage".to_string(),
            account_type: AccountType::Securities,
            group_name: Some("Personal".to_string()),
            currency: "USD".to_string(),
            is_default: true,
            platform_id: None,
            account_number: Some("U123456".to_string()),
            tracking_mode: TrackingMode::Transactions,
        })
        .await
        .expect("Failed to create account");

    assert_eq!(account.name, "Main Brokerage");
    assert_eq!(account.account_type, AccountType::Securities);
    assert_eq!(account.tracking_mode, TrackingMode::Transactions);
    assert!(account.is_default);
    assert!(account.is_active);
    assert!(!account.is_archived);

    let by_workspace = repo
        .list_by_workspace("ws-1")
        .await
        .expect("Failed to list workspace accounts");
    assert_eq!(by_workspace.len(), 1);

    let other_workspace = repo
        .list_by_workspace("ws-2")
        .await
        .expect("Failed to list other workspace accounts");
    assert!(other_workspace.is_empty());

    let fetched = repo.get(&account.id).await.expect("Failed to get account");
    assert!(fetched.is_some());
    assert_eq!(fetched.unwrap().account_number.as_deref(), Some("U123456"));
}

#[tokio::test]
async fn account_repository_rejects_missing_workspace_fk() {
    let pool = setup_test_db().await;
    let repo = AccountRepository::new(pool);

    let result = repo
        .create(CreateAccountInput {
            workspace_id: Some("does-not-exist".to_string()),
            name: "Orphan".to_string(),
            account_type: AccountType::Cash,
            group_name: None,
            currency: "EUR".to_string(),
            is_default: false,
            platform_id: None,
            account_number: None,
            tracking_mode: TrackingMode::Transactions,
        })
        .await;

    assert!(result.is_err(), "workspace FK must be enforced");
}
