// Tests for option contract repository persistence.

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use domain::option::{DataSource, OptionChain, OptionContract, OptionType};
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;

    use crate::database::repositories::option_chain_repository::OptionChainRepository;
    use crate::database::repositories::option_contract_repository::OptionContractRepository;

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

    async fn seed_chain(pool: &SqlitePool) -> String {
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
        let chain_repo = OptionChainRepository::new(pool.clone());
        chain_repo
            .create(&chain)
            .await
            .expect("Failed to seed chain");
        "chain-1".to_string()
    }

    fn contract_fixture(chain_id: &str, id: &str, strike: f64) -> OptionContract {
        let now = Utc::now();
        OptionContract {
            id: id.to_string(),
            workspace_id: "workspace-1".into(),
            chain_id: chain_id.to_string(),
            symbol: "AAPL".into(),
            option_type: OptionType::Call,
            strike,
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
        }
    }

    #[tokio::test]
    async fn creates_and_gets_contract() {
        let pool = setup_test_db().await;
        let chain_id = seed_chain(&pool).await;
        let repo = OptionContractRepository::new(pool.clone());

        let contract = contract_fixture(&chain_id, "contract-1", 150.0);
        repo.create(&contract)
            .await
            .expect("Failed to create contract");

        let found = repo
            .get("contract-1")
            .await
            .expect("Failed to get contract")
            .expect("Contract should exist");
        assert_eq!(found.symbol, "AAPL");
        assert_eq!(found.strike, 150.0);
        assert_eq!(found.option_type, OptionType::Call);
    }

    #[tokio::test]
    async fn returns_none_for_missing_contract() {
        let pool = setup_test_db().await;
        let repo = OptionContractRepository::new(pool);

        let result = repo
            .get("nonexistent")
            .await
            .expect("Failed to get contract");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn lists_contracts_by_chain_in_order() {
        let pool = setup_test_db().await;
        let chain_id = seed_chain(&pool).await;
        let repo = OptionContractRepository::new(pool.clone());

        repo.create(&contract_fixture(&chain_id, "contract-1", 150.0))
            .await
            .expect("Failed to create contract 1");
        repo.create(&contract_fixture(&chain_id, "contract-2", 155.0))
            .await
            .expect("Failed to create contract 2");

        let contracts = repo
            .list_by_chain(&chain_id)
            .await
            .expect("Failed to list contracts");
        assert_eq!(contracts.len(), 2);
        // Ordered by strike ASC
        assert_eq!(contracts[0].strike, 150.0);
        assert_eq!(contracts[1].strike, 155.0);
    }

    #[tokio::test]
    async fn empty_list_for_nonexistent_chain() {
        let pool = setup_test_db().await;
        let repo = OptionContractRepository::new(pool);

        let contracts = repo
            .list_by_chain("nonexistent")
            .await
            .expect("Failed to list contracts");
        assert!(contracts.is_empty());
    }

    #[tokio::test]
    async fn updates_contract_fields() {
        let pool = setup_test_db().await;
        let chain_id = seed_chain(&pool).await;
        let repo = OptionContractRepository::new(pool.clone());

        let mut contract = contract_fixture(&chain_id, "contract-1", 150.0);
        repo.create(&contract)
            .await
            .expect("Failed to create contract");

        contract.bid = 5.0;
        contract.ask = 6.0;
        contract.last = Some(5.5);
        repo.update(&contract)
            .await
            .expect("Failed to update contract");

        let updated = repo
            .get("contract-1")
            .await
            .expect("Failed to get updated contract")
            .expect("Contract should exist");
        assert!((updated.bid - 5.0).abs() < 0.001);
        assert!((updated.ask - 6.0).abs() < 0.001);
        assert_eq!(updated.last, Some(5.5));
    }

    #[tokio::test]
    async fn update_returns_not_found_for_missing_contract() {
        let pool = setup_test_db().await;
        let chain_id = seed_chain(&pool).await;
        let repo = OptionContractRepository::new(pool);

        let mut contract = contract_fixture(&chain_id, "nonexistent", 150.0);
        contract.id = "nonexistent".into();
        let result = repo.update(&contract).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn deletes_contract() {
        let pool = setup_test_db().await;
        let chain_id = seed_chain(&pool).await;
        let repo = OptionContractRepository::new(pool.clone());

        repo.create(&contract_fixture(&chain_id, "contract-1", 150.0))
            .await
            .expect("Failed to create contract");

        repo.delete("contract-1")
            .await
            .expect("Failed to delete contract");

        let found = repo
            .get("contract-1")
            .await
            .expect("Failed to get contract");
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn delete_returns_not_found_for_missing_contract() {
        let pool = setup_test_db().await;
        let repo = OptionContractRepository::new(pool);

        let result = repo.delete("nonexistent").await;
        assert!(result.is_err());
    }
}
