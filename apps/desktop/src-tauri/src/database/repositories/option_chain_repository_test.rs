// Tests for option chain and contract persistence.

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use domain::option::{DataSource, OptionChain, OptionContract, OptionType};
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;

    use crate::database::repositories::option_chain_repository::OptionChainRepository;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("Failed to create test database");
        crate::database::migrations::run(&pool)
            .await
            .expect("Failed to run migrations");
        sqlx::query("INSERT INTO workspaces (id, name) VALUES ('workspace-1', 'Options')")
            .execute(&pool)
            .await
            .expect("Failed to create test workspace");
        sqlx::query("INSERT INTO workspaces (id, name) VALUES ('workspace-2', 'Other')")
            .execute(&pool)
            .await
            .expect("Failed to create second workspace");
        pool
    }

    fn fixtures() -> (OptionChain, OptionContract) {
        let now = Utc::now();
        let chain = OptionChain {
            id: "chain-1".into(),
            workspace_id: "workspace-1".into(),
            symbol: "AAPL".into(),
            underlying_price: 150.0,
            as_of: now,
            data_source: DataSource::Demo,
            created_at: now,
        };
        let contract = OptionContract {
            id: "contract-1".into(),
            workspace_id: chain.workspace_id.clone(),
            chain_id: chain.id.clone(),
            symbol: chain.symbol.clone(),
            option_type: OptionType::Call,
            strike: 150.0,
            expiration: now + Duration::days(30),
            contract_multiplier: 100,
            bid: 4.0,
            ask: 5.0,
            last: Some(4.5),
            volume: 10,
            open_interest: 20,
            implied_volatility: 0.25,
            created_at: now,
            updated_at: now,
        };
        (chain, contract)
    }

    #[tokio::test]
    async fn persists_chain_and_contracts_and_lists_by_workspace() {
        let pool = setup_test_db().await;
        let repo = OptionChainRepository::new(pool.clone());
        let (chain, contract) = fixtures();

        repo.create_with_contracts(&chain, &[contract])
            .await
            .expect("Failed to persist option fixtures");

        let chains = repo
            .list_by_workspace("workspace-1")
            .await
            .expect("Failed to list option chains");
        assert_eq!(chains.len(), 1);
        assert_eq!(chains[0].id, "chain-1");
        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM option_contracts WHERE chain_id = 'chain-1'")
                .fetch_one(&pool)
                .await
                .expect("Failed to count option contracts");
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn rolls_back_chain_when_contract_insert_fails() {
        let pool = setup_test_db().await;
        let repo = OptionChainRepository::new(pool.clone());
        let (chain, mut contract) = fixtures();
        // Violate bid <= ask constraint
        contract.bid = 6.0;
        contract.ask = 5.0;

        assert!(repo
            .create_with_contracts(&chain, &[contract])
            .await
            .is_err());
        let chain_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM option_chains WHERE id = 'chain-1'")
                .fetch_one(&pool)
                .await
                .expect("Failed to count option chains");
        assert_eq!(chain_count, 0);
    }

    #[tokio::test]
    async fn rejects_contract_with_mismatched_chain_scope() {
        let pool = setup_test_db().await;
        let repo = OptionChainRepository::new(pool);
        let (chain, mut contract) = fixtures();
        contract.chain_id = "different-chain".into();

        let result = repo.create_with_contracts(&chain, &[contract]).await;
        assert!(matches!(result, Err(crate::error::AppError::Validation(_))));
    }

    #[tokio::test]
    async fn gets_chain_by_id() {
        let pool = setup_test_db().await;
        let repo = OptionChainRepository::new(pool.clone());
        let (chain, contract) = fixtures();

        repo.create_with_contracts(&chain, &[contract])
            .await
            .expect("Failed to persist chain");

        let found = repo
            .get("chain-1")
            .await
            .expect("Failed to get chain")
            .expect("Chain should exist");
        assert_eq!(found.id, "chain-1");
        assert_eq!(found.symbol, "AAPL");
        assert_eq!(found.underlying_price, 150.0);
        assert_eq!(found.data_source, DataSource::Demo);
    }

    #[tokio::test]
    async fn returns_none_for_missing_chain() {
        let pool = setup_test_db().await;
        let repo = OptionChainRepository::new(pool);

        let found = repo.get("nonexistent").await.expect("Failed to get chain");
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn gets_latest_chain_for_symbol() {
        let pool = setup_test_db().await;
        let repo = OptionChainRepository::new(pool.clone());
        let (chain, contract) = fixtures();

        repo.create_with_contracts(&chain, &[contract])
            .await
            .expect("Failed to persist chain");

        let latest = repo
            .get_latest("workspace-1", "AAPL")
            .await
            .expect("Failed to get latest chain")
            .expect("Latest chain should exist");
        assert_eq!(latest.id, "chain-1");
    }

    #[tokio::test]
    async fn deletes_chain_and_cascades_to_contracts() {
        let pool = setup_test_db().await;
        let repo = OptionChainRepository::new(pool.clone());
        let (chain, contract) = fixtures();

        repo.create_with_contracts(&chain, &[contract])
            .await
            .expect("Failed to persist chain");

        repo.delete("chain-1")
            .await
            .expect("Failed to delete chain");

        let found = repo.get("chain-1").await.expect("Failed to get chain");
        assert!(found.is_none());

        let contract_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM option_contracts WHERE chain_id = 'chain-1'")
                .fetch_one(&pool)
                .await
                .expect("Failed to count contracts");
        assert_eq!(contract_count, 0);
    }

    #[tokio::test]
    async fn delete_returns_not_found_for_missing_chain() {
        let pool = setup_test_db().await;
        let repo = OptionChainRepository::new(pool);

        let result = repo.delete("nonexistent").await;
        assert!(matches!(result, Err(crate::error::AppError::NotFound(_))));
    }

    #[tokio::test]
    async fn respects_workspace_isolation() {
        let pool = setup_test_db().await;
        let repo = OptionChainRepository::new(pool.clone());
        let (chain, contract) = fixtures();

        repo.create_with_contracts(&chain, &[contract])
            .await
            .expect("Failed to persist chain");

        let chains = repo
            .list_by_workspace("workspace-2")
            .await
            .expect("Failed to list chains");
        assert!(chains.is_empty());

        let latest = repo
            .get_latest("workspace-2", "AAPL")
            .await
            .expect("Failed to get latest chain");
        assert!(latest.is_none());
    }

    #[tokio::test]
    async fn workspace_cascade_deletes_chains() {
        let pool = setup_test_db().await;
        let repo = OptionChainRepository::new(pool.clone());
        let (chain, contract) = fixtures();

        repo.create_with_contracts(&chain, &[contract])
            .await
            .expect("Failed to persist chain");

        sqlx::query("DELETE FROM workspaces WHERE id = 'workspace-1'")
            .execute(&pool)
            .await
            .expect("Failed to delete workspace");

        let chains = repo
            .list_by_workspace("workspace-1")
            .await
            .expect("Failed to list chains");
        assert!(chains.is_empty());
    }
}
