// Tests for Greeks repository persistence.

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Duration, Utc};
    use domain::option::{
        DataSource, Greeks, OptionChain, OptionContract, OptionType, PricingModel,
    };
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;

    use crate::database::repositories::greeks_repository::GreeksRepository;
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

    fn greeks_fixture(id: &str, contract_id: &str, calculated_at: DateTime<Utc>) -> Greeks {
        Greeks {
            id: id.to_string(),
            option_contract_id: contract_id.to_string(),
            delta: 0.52,
            gamma: 0.08,
            theta: -0.05,
            vega: 0.15,
            rho: 0.02,
            iv: 0.25,
            calculated_at,
            calculation_model: PricingModel::BlackScholes,
        }
    }

    #[tokio::test]
    async fn creates_and_gets_latest_greeks() {
        let pool = setup_test_db().await;
        let contract_id = seed_contract(&pool).await;
        let repo = GreeksRepository::new(pool.clone());

        let now = Utc::now();
        let greeks = greeks_fixture("greeks-1", &contract_id, now);
        repo.create(&greeks).await.expect("Failed to create Greeks");

        let latest = repo
            .get_latest(&contract_id)
            .await
            .expect("Failed to get latest Greeks")
            .expect("Greeks should exist");
        assert!((latest.delta - 0.52).abs() < 0.001);
        assert!((latest.gamma - 0.08).abs() < 0.001);
        assert_eq!(latest.calculation_model, PricingModel::BlackScholes);
    }

    #[tokio::test]
    async fn returns_none_for_missing_contract() {
        let pool = setup_test_db().await;
        let repo = GreeksRepository::new(pool);

        let result = repo
            .get_latest("nonexistent")
            .await
            .expect("Failed to get Greeks");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn lists_greeks_by_contract_in_reverse_chronological_order() {
        let pool = setup_test_db().await;
        let contract_id = seed_contract(&pool).await;
        let repo = GreeksRepository::new(pool.clone());

        let early = Utc::now() - Duration::hours(2);
        let late = Utc::now();

        repo.create(&greeks_fixture("greeks-early", &contract_id, early))
            .await
            .expect("Failed to create early Greeks");
        repo.create(&greeks_fixture("greeks-late", &contract_id, late))
            .await
            .expect("Failed to create late Greeks");

        let list = repo
            .list_by_contract(&contract_id)
            .await
            .expect("Failed to list Greeks");
        assert_eq!(list.len(), 2);
        // Most recent first
        assert_eq!(list[0].calculated_at, late);
        assert_eq!(list[1].calculated_at, early);
    }

    #[tokio::test]
    async fn deletes_greeks_by_contract() {
        let pool = setup_test_db().await;
        let contract_id = seed_contract(&pool).await;
        let repo = GreeksRepository::new(pool.clone());

        repo.create(&greeks_fixture("greeks-delete", &contract_id, Utc::now()))
            .await
            .expect("Failed to create Greeks");

        repo.delete_by_contract(&contract_id)
            .await
            .expect("Failed to delete Greeks");

        let latest = repo
            .get_latest(&contract_id)
            .await
            .expect("Failed to get Greeks");
        assert!(latest.is_none());
    }

    #[tokio::test]
    async fn unique_constraint_on_contract_and_timestamp() {
        let pool = setup_test_db().await;
        let contract_id = seed_contract(&pool).await;
        let repo = GreeksRepository::new(pool);

        let now = Utc::now();
        repo.create(&greeks_fixture("greeks-uniq-1", &contract_id, now))
            .await
            .expect("Failed to create first Greeks");

        let result = repo
            .create(&greeks_fixture("greeks-uniq-2", &contract_id, now))
            .await;
        assert!(
            result.is_err(),
            "Duplicate (contract, calculated_at) should fail"
        );
    }
}
