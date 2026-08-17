// Tests for option position repository persistence.

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use domain::option::{DataSource, OptionChain, OptionContract, OptionPosition, OptionType};
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;

    use crate::database::repositories::option_chain_repository::OptionChainRepository;
    use crate::database::repositories::option_position_repository::OptionPositionRepository;

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
            .expect("Failed to seed contract");
        "contract-1".to_string()
    }

    fn position_fixture(contract_id: &str, id: &str) -> OptionPosition {
        let now = Utc::now();
        OptionPosition {
            id: id.to_string(),
            workspace_id: "workspace-1".into(),
            account_id: None,
            option_contract_id: contract_id.to_string(),
            quantity: 2,
            cost_basis: 1040.0,
            opened_at: now,
            closed_at: None,
            notes: Some("Test position".to_string()),
        }
    }

    #[tokio::test]
    async fn creates_and_gets_position() {
        let pool = setup_test_db().await;
        let contract_id = seed_contract(&pool).await;
        let repo = OptionPositionRepository::new(pool.clone());

        let position = position_fixture(&contract_id, "position-1");
        repo.create(&position)
            .await
            .expect("Failed to create position");

        let found = repo
            .get("position-1")
            .await
            .expect("Failed to get position")
            .expect("Position should exist");
        assert_eq!(found.quantity, 2);
        assert!((found.cost_basis - 1040.0).abs() < 0.001);
        assert!(found.closed_at.is_none());
    }

    #[tokio::test]
    async fn returns_none_for_missing_position() {
        let pool = setup_test_db().await;
        let repo = OptionPositionRepository::new(pool);

        let result = repo
            .get("nonexistent")
            .await
            .expect("Failed to get position");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn lists_positions_by_workspace() {
        let pool = setup_test_db().await;
        let contract_id = seed_contract(&pool).await;
        let repo = OptionPositionRepository::new(pool.clone());

        repo.create(&position_fixture(&contract_id, "position-1"))
            .await
            .expect("Failed to create position 1");
        repo.create(&position_fixture(&contract_id, "position-2"))
            .await
            .expect("Failed to create position 2");

        let positions = repo
            .list_by_workspace("workspace-1")
            .await
            .expect("Failed to list positions");
        assert_eq!(positions.len(), 2);
    }

    #[tokio::test]
    async fn respects_workspace_isolation() {
        let pool = setup_test_db().await;
        let contract_id = seed_contract(&pool).await;
        let repo = OptionPositionRepository::new(pool.clone());

        repo.create(&position_fixture(&contract_id, "position-1"))
            .await
            .expect("Failed to create position");

        let positions = repo
            .list_by_workspace("workspace-2")
            .await
            .expect("Failed to list positions");
        assert!(positions.is_empty());
    }

    #[tokio::test]
    async fn lists_only_open_positions() {
        let pool = setup_test_db().await;
        let contract_id = seed_contract(&pool).await;
        let repo = OptionPositionRepository::new(pool.clone());

        let open_pos = position_fixture(&contract_id, "position-1");
        repo.create(&open_pos)
            .await
            .expect("Failed to create open position");

        let mut closed_pos = position_fixture(&contract_id, "position-2");
        closed_pos.closed_at = Some(Utc::now());
        repo.create(&closed_pos)
            .await
            .expect("Failed to create closed position");

        let open = repo
            .list_open("workspace-1")
            .await
            .expect("Failed to list open positions");
        assert_eq!(open.len(), 1);
        assert_eq!(open[0].id, "position-1");
    }

    #[tokio::test]
    async fn updates_position() {
        let pool = setup_test_db().await;
        let contract_id = seed_contract(&pool).await;
        let repo = OptionPositionRepository::new(pool.clone());

        let mut position = position_fixture(&contract_id, "position-1");
        repo.create(&position)
            .await
            .expect("Failed to create position");

        position.quantity = 5;
        position.cost_basis = 2600.0;
        repo.update(&position)
            .await
            .expect("Failed to update position");

        let updated = repo
            .get("position-1")
            .await
            .expect("Failed to get updated position")
            .expect("Position should exist");
        assert_eq!(updated.quantity, 5);
        assert!((updated.cost_basis - 2600.0).abs() < 0.001);
    }

    #[tokio::test]
    async fn closes_position() {
        let pool = setup_test_db().await;
        let contract_id = seed_contract(&pool).await;
        let repo = OptionPositionRepository::new(pool.clone());

        let position = position_fixture(&contract_id, "position-1");
        repo.create(&position)
            .await
            .expect("Failed to create position");

        let closed_at = Utc::now();
        repo.close("position-1", closed_at)
            .await
            .expect("Failed to close position");

        let updated = repo
            .get("position-1")
            .await
            .expect("Failed to get closed position")
            .expect("Position should exist");
        assert!(updated.closed_at.is_some());
    }

    #[tokio::test]
    async fn close_returns_not_found_for_missing_position() {
        let pool = setup_test_db().await;
        let repo = OptionPositionRepository::new(pool);

        let result = repo.close("nonexistent", Utc::now()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn deletes_position() {
        let pool = setup_test_db().await;
        let contract_id = seed_contract(&pool).await;
        let repo = OptionPositionRepository::new(pool.clone());

        repo.create(&position_fixture(&contract_id, "position-1"))
            .await
            .expect("Failed to create position");

        repo.delete("position-1")
            .await
            .expect("Failed to delete position");

        let found = repo
            .get("position-1")
            .await
            .expect("Failed to get position");
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn delete_returns_not_found_for_missing_position() {
        let pool = setup_test_db().await;
        let repo = OptionPositionRepository::new(pool);

        let result = repo.delete("nonexistent").await;
        assert!(result.is_err());
    }
}
