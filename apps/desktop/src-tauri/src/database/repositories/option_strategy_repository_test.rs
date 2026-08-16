// Tests for option strategy repository persistence.

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

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("test database");
        crate::database::migrations::run(&pool)
            .await
            .expect("migrations");
        sqlx::query("INSERT INTO workspaces (id, name) VALUES ('workspace-1', 'Options')")
            .execute(&pool)
            .await
            .expect("workspace fixture");
        sqlx::query("INSERT INTO workspaces (id, name) VALUES ('workspace-2', 'Other')")
            .execute(&pool)
            .await
            .expect("second workspace fixture");
        pool
    }

    async fn seed_contract(pool: &SqlitePool) -> String {
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
            .expect("seed contract");
        "contract-1".to_string()
    }

    fn strategy_fixture(id: &str, workspace_id: &str) -> OptionStrategy {
        let now = Utc::now();
        OptionStrategy {
            id: id.into(),
            workspace_id: workspace_id.into(),
            name: "Bull Call Spread".into(),
            strategy_type: StrategyType::BullCallSpread,
            underlying: "AAPL".into(),
            total_cost: -250.0,
            max_profit: Some(250.0),
            max_loss: Some(-250.0),
            break_even_points: vec![152.5],
            created_at: now,
            updated_at: now,
        }
    }

    fn leg_fixture(strategy_id: &str, contract_id: &str) -> StrategyLeg {
        let now = Utc::now();
        StrategyLeg {
            id: "leg-1".into(),
            strategy_id: strategy_id.into(),
            option_contract_id: contract_id.into(),
            quantity: 1,
            position_type: PositionType::Long,
            premium: 5.0,
            strike: 150.0,
            expiration: now,
            option_type: OptionType::Call,
        }
    }

    #[tokio::test]
    async fn rolls_back_strategy_when_a_leg_cannot_be_inserted() {
        let pool = setup_test_db().await;
        let repo = OptionStrategyRepository::new(pool.clone());

        let strategy = strategy_fixture("strategy-1", "workspace-1");
        // Leg references a nonexistent contract, so the transaction must fail
        let mut leg = leg_fixture("strategy-1", "missing-contract");
        leg.id = "leg-missing".into();
        assert!(repo.create_with_legs(&strategy, &[leg]).await.is_err());
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM option_strategies")
            .fetch_one(&pool)
            .await
            .expect("strategy count");
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn rejects_leg_that_belongs_to_another_strategy() {
        let pool = setup_test_db().await;
        let repo = OptionStrategyRepository::new(pool);

        let strategy = strategy_fixture("strategy-1", "workspace-1");
        let mut leg = leg_fixture("strategy-1", "contract-1");
        leg.strategy_id = "other-strategy".into();

        let result = repo.create_with_legs(&strategy, &[leg]).await;
        assert!(matches!(result, Err(crate::error::AppError::Validation(_))));
    }

    #[tokio::test]
    async fn creates_strategy_and_lists_by_workspace() {
        let pool = setup_test_db().await;
        let repo = OptionStrategyRepository::new(pool.clone());

        repo.create(&strategy_fixture("strategy-1", "workspace-1"))
            .await
            .expect("create strategy");

        let strategies = repo
            .list_by_workspace("workspace-1")
            .await
            .expect("list strategies");
        assert_eq!(strategies.len(), 1);
        assert_eq!(strategies[0].id, "strategy-1");
        assert_eq!(strategies[0].strategy_type, StrategyType::BullCallSpread);
        assert_eq!(strategies[0].break_even_points, vec![152.5]);
    }

    #[tokio::test]
    async fn creates_strategy_with_legs() {
        let pool = setup_test_db().await;
        let contract_id = seed_contract(&pool).await;
        let repo = OptionStrategyRepository::new(pool.clone());

        let strategy = strategy_fixture("strategy-1", "workspace-1");
        let leg = leg_fixture("strategy-1", &contract_id);
        repo.create_with_legs(&strategy, &[leg])
            .await
            .expect("create strategy with legs");

        let found = repo
            .get("strategy-1")
            .await
            .expect("get strategy")
            .expect("strategy exists");
        assert_eq!(found.id, "strategy-1");
        let leg_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM strategy_legs")
            .fetch_one(&pool)
            .await
            .expect("leg count");
        assert_eq!(leg_count, 1);
    }

    #[tokio::test]
    async fn gets_strategy_by_id() {
        let pool = setup_test_db().await;
        let repo = OptionStrategyRepository::new(pool.clone());

        repo.create(&strategy_fixture("strategy-1", "workspace-1"))
            .await
            .expect("create strategy");

        let found = repo
            .get("strategy-1")
            .await
            .expect("get strategy")
            .expect("strategy exists");
        assert_eq!(found.name, "Bull Call Spread");
    }

    #[tokio::test]
    async fn returns_none_for_missing_strategy() {
        let pool = setup_test_db().await;
        let repo = OptionStrategyRepository::new(pool);

        let result = repo.get("nonexistent").await.expect("get strategy");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn respects_workspace_isolation() {
        let pool = setup_test_db().await;
        let repo = OptionStrategyRepository::new(pool.clone());

        repo.create(&strategy_fixture("strategy-1", "workspace-1"))
            .await
            .expect("create strategy");

        let strategies = repo
            .list_by_workspace("workspace-2")
            .await
            .expect("list strategies");
        assert!(strategies.is_empty());
    }

    #[tokio::test]
    async fn updates_strategy() {
        let pool = setup_test_db().await;
        let repo = OptionStrategyRepository::new(pool.clone());

        let mut strategy = strategy_fixture("strategy-1", "workspace-1");
        repo.create(&strategy).await.expect("create strategy");

        strategy.name = "Renamed".into();
        strategy.max_profit = Some(300.0);
        repo.update(&strategy).await.expect("update strategy");

        let updated = repo
            .get("strategy-1")
            .await
            .expect("get strategy")
            .expect("strategy exists");
        assert_eq!(updated.name, "Renamed");
        assert_eq!(updated.max_profit, Some(300.0));
    }

    #[tokio::test]
    async fn update_returns_not_found_for_missing_strategy() {
        let pool = setup_test_db().await;
        let repo = OptionStrategyRepository::new(pool);

        let mut strategy = strategy_fixture("nonexistent", "workspace-1");
        strategy.id = "nonexistent".into();
        let result = repo.update(&strategy).await;
        assert!(matches!(result, Err(crate::error::AppError::NotFound(_))));
    }

    #[tokio::test]
    async fn deletes_strategy_and_cascades_to_legs() {
        let pool = setup_test_db().await;
        let repo = OptionStrategyRepository::new(pool.clone());

        let strategy = strategy_fixture("strategy-1", "workspace-1");
        repo.create(&strategy).await.expect("create strategy");
        repo.delete("strategy-1").await.expect("delete strategy");

        let result = repo.get("strategy-1").await.expect("get strategy");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn delete_returns_not_found_for_missing_strategy() {
        let pool = setup_test_db().await;
        let repo = OptionStrategyRepository::new(pool);

        let result = repo.delete("nonexistent").await;
        assert!(matches!(result, Err(crate::error::AppError::NotFound(_))));
    }
}
