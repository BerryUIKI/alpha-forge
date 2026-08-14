// Tests for OptionService chain acquisition and persistence.

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;

    use crate::database::repositories::greeks_repository::GreeksRepository;
    use crate::database::repositories::option_chain_repository::OptionChainRepository;
    use crate::database::repositories::option_contract_repository::OptionContractRepository;
    use crate::database::repositories::option_strategy_repository::OptionStrategyRepository;
    use crate::services::option_service::OptionService;
    use domain::option::DataSource;

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

    fn service(pool: SqlitePool) -> OptionService {
        OptionService::new(
            Arc::new(OptionChainRepository::new(pool.clone())),
            Arc::new(OptionContractRepository::new(pool.clone())),
            Arc::new(GreeksRepository::new(pool.clone())),
            Arc::new(OptionStrategyRepository::new(pool)),
        )
    }

    #[tokio::test]
    async fn fetch_demo_chain_persists_and_lists_generated_contracts() {
        let service = service(setup_test_db().await);

        let chain = service
            .fetch_chain(" AAPL ", "workspace-1", DataSource::Demo)
            .await
            .expect("Failed to fetch demo chain");
        let chains = service
            .list_chains("workspace-1")
            .await
            .expect("Failed to list persisted chains");
        let contracts = service
            .list_contracts(&chain.id)
            .await
            .expect("Failed to list persisted contracts");

        assert_eq!(chain.symbol, "AAPL");
        assert_eq!(chains.len(), 1);
        assert_eq!(chains[0].id, chain.id);
        assert_eq!(contracts.len(), 22);
        assert!(contracts
            .iter()
            .all(|contract| contract.chain_id == chain.id));

        let second_chain = service
            .fetch_chain("AAPL", "workspace-1", DataSource::Demo)
            .await
            .expect("Failed to fetch the second demo chain");
        assert_ne!(second_chain.id, chain.id);
        assert_eq!(
            service
                .list_contracts(&second_chain.id)
                .await
                .expect("Failed to list second chain contracts")
                .len(),
            22
        );
    }

    #[tokio::test]
    async fn rejects_missing_fetch_scope_before_provider_access() {
        let service = service(setup_test_db().await);

        assert!(service
            .fetch_chain(" ", "workspace-1", DataSource::Demo)
            .await
            .is_err());
        assert!(service
            .fetch_chain("AAPL", " ", DataSource::Demo)
            .await
            .is_err());
    }
}
