// Tests for strategy leg repository persistence.

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use domain::option::{
        DataSource, OptionChain, OptionContract, OptionStrategy, OptionType, PositionType,
        StrategyLeg, StrategyType,
    };
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;

    use crate::database::repositories::option_chain_repository::OptionChainRepository;
    use crate::database::repositories::option_strategy_repository::OptionStrategyRepository;
    use crate::database::repositories::strategy_leg_repository::StrategyLegRepository;

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

    async fn seed_contract_and_strategy(pool: &SqlitePool) -> (String, String) {
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
            workspace_id: "workspace-1".into(),
            chain_id: "chain-1".into(),
            symbol: "AAPL".into(),
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
        let chain_repo = OptionChainRepository::new(pool.clone());
        chain_repo
            .create_with_contracts(&chain, &[contract])
            .await
            .expect("Failed to seed contract");

        let strategy = OptionStrategy {
            id: "strategy-1".into(),
            workspace_id: "workspace-1".into(),
            name: "Test Strategy".into(),
            strategy_type: StrategyType::Custom,
            underlying: "AAPL".into(),
            total_cost: 0.0,
            max_profit: None,
            max_loss: None,
            break_even_points: Vec::new(),
            created_at: now,
            updated_at: now,
        };
        let strategy_repo = OptionStrategyRepository::new(pool.clone());
        strategy_repo
            .create(&strategy)
            .await
            .expect("Failed to seed strategy");
        ("contract-1".to_string(), "strategy-1".to_string())
    }

    fn leg_fixture(strategy_id: &str, contract_id: &str, id: &str) -> StrategyLeg {
        let now = Utc::now();
        StrategyLeg {
            id: id.to_string(),
            strategy_id: strategy_id.to_string(),
            option_contract_id: contract_id.to_string(),
            quantity: 1,
            position_type: PositionType::Long,
            premium: 5.0,
            strike: 150.0,
            expiration: now,
            option_type: OptionType::Call,
        }
    }

    #[tokio::test]
    async fn creates_and_gets_leg() {
        let pool = setup_test_db().await;
        let (contract_id, strategy_id) = seed_contract_and_strategy(&pool).await;
        let repo = StrategyLegRepository::new(pool.clone());

        let leg = leg_fixture(&strategy_id, &contract_id, "leg-1");
        repo.create(&leg).await.expect("Failed to create leg");

        let found = repo
            .get("leg-1")
            .await
            .expect("Failed to get leg")
            .expect("Leg should exist");
        assert_eq!(found.quantity, 1);
        assert_eq!(found.position_type, PositionType::Long);
    }

    #[tokio::test]
    async fn returns_none_for_missing_leg() {
        let pool = setup_test_db().await;
        let repo = StrategyLegRepository::new(pool);

        let result = repo.get("nonexistent").await.expect("Failed to get leg");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn lists_legs_by_strategy() {
        let pool = setup_test_db().await;
        let (contract_id, strategy_id) = seed_contract_and_strategy(&pool).await;
        let repo = StrategyLegRepository::new(pool.clone());

        repo.create(&leg_fixture(&strategy_id, &contract_id, "leg-1"))
            .await
            .expect("Failed to create leg 1");
        repo.create(&leg_fixture(&strategy_id, &contract_id, "leg-2"))
            .await
            .expect("Failed to create leg 2");

        let legs = repo
            .list_by_strategy(&strategy_id)
            .await
            .expect("Failed to list legs");
        assert_eq!(legs.len(), 2);
    }

    #[tokio::test]
    async fn lists_legs_by_workspace_through_strategy_join() {
        let pool = setup_test_db().await;
        let (contract_id, strategy_id) = seed_contract_and_strategy(&pool).await;
        let repo = StrategyLegRepository::new(pool.clone());

        repo.create(&leg_fixture(&strategy_id, &contract_id, "leg-1"))
            .await
            .expect("Failed to create leg");

        let legs = repo
            .list_by_workspace("workspace-1")
            .await
            .expect("Failed to list legs by workspace");
        assert_eq!(legs.len(), 1);
    }

    #[tokio::test]
    async fn respects_workspace_isolation_for_leg_listing() {
        let pool = setup_test_db().await;
        let (contract_id, strategy_id) = seed_contract_and_strategy(&pool).await;
        let repo = StrategyLegRepository::new(pool.clone());

        repo.create(&leg_fixture(&strategy_id, &contract_id, "leg-1"))
            .await
            .expect("Failed to create leg");

        let legs = repo
            .list_by_workspace("workspace-2")
            .await
            .expect("Failed to list legs by workspace");
        assert!(legs.is_empty());
    }

    #[tokio::test]
    async fn updates_leg() {
        let pool = setup_test_db().await;
        let (contract_id, strategy_id) = seed_contract_and_strategy(&pool).await;
        let repo = StrategyLegRepository::new(pool.clone());

        let mut leg = leg_fixture(&strategy_id, &contract_id, "leg-1");
        repo.create(&leg).await.expect("Failed to create leg");

        leg.quantity = 3;
        leg.premium = 6.0;
        repo.update(&leg).await.expect("Failed to update leg");

        let updated = repo
            .get("leg-1")
            .await
            .expect("Failed to get updated leg")
            .expect("Leg should exist");
        assert_eq!(updated.quantity, 3);
        assert!((updated.premium - 6.0).abs() < 0.001);
    }

    #[tokio::test]
    async fn update_returns_not_found_for_missing_leg() {
        let pool = setup_test_db().await;
        let (contract_id, strategy_id) = seed_contract_and_strategy(&pool).await;
        let repo = StrategyLegRepository::new(pool);

        let mut leg = leg_fixture(&strategy_id, &contract_id, "nonexistent");
        leg.id = "nonexistent".into();
        let result = repo.update(&leg).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn deletes_leg() {
        let pool = setup_test_db().await;
        let (contract_id, strategy_id) = seed_contract_and_strategy(&pool).await;
        let repo = StrategyLegRepository::new(pool.clone());

        repo.create(&leg_fixture(&strategy_id, &contract_id, "leg-1"))
            .await
            .expect("Failed to create leg");

        repo.delete("leg-1").await.expect("Failed to delete leg");

        let found = repo.get("leg-1").await.expect("Failed to get leg");
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn delete_returns_not_found_for_missing_leg() {
        let pool = setup_test_db().await;
        let repo = StrategyLegRepository::new(pool);

        let result = repo.delete("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn deletes_all_legs_by_strategy() {
        let pool = setup_test_db().await;
        let (contract_id, strategy_id) = seed_contract_and_strategy(&pool).await;
        let repo = StrategyLegRepository::new(pool.clone());

        repo.create(&leg_fixture(&strategy_id, &contract_id, "leg-1"))
            .await
            .expect("Failed to create leg 1");
        repo.create(&leg_fixture(&strategy_id, &contract_id, "leg-2"))
            .await
            .expect("Failed to create leg 2");

        repo.delete_by_strategy(&strategy_id)
            .await
            .expect("Failed to delete legs by strategy");

        let legs = repo
            .list_by_strategy(&strategy_id)
            .await
            .expect("Failed to list legs");
        assert!(legs.is_empty());
    }
}
