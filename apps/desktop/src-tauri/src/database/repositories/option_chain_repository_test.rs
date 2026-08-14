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
}
