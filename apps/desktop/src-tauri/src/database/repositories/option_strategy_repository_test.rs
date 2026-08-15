#[cfg(test)]
mod tests {
    use chrono::Utc;
    use domain::option::{OptionStrategy, OptionType, PositionType, StrategyLeg, StrategyType};
    use sqlx::sqlite::SqlitePoolOptions;

    use crate::database::repositories::option_strategy_repository::OptionStrategyRepository;

    #[tokio::test]
    async fn rolls_back_strategy_when_a_leg_cannot_be_inserted() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("test database");
        crate::database::migrations::run(&pool)
            .await
            .expect("migrations");
        sqlx::query("INSERT INTO workspaces (id, name) VALUES ('00000000-0000-4000-8000-000000000001', 'Options')")
        .execute(&pool)
        .await
        .expect("workspace fixture");

        let now = Utc::now();
        let strategy = OptionStrategy {
            id: "00000000-0000-4000-8000-000000000002".into(),
            workspace_id: "00000000-0000-4000-8000-000000000001".into(),
            name: "Rollback".into(),
            strategy_type: StrategyType::Custom,
            underlying: "AAPL".into(),
            total_cost: 0.0,
            max_profit: None,
            max_loss: None,
            break_even_points: Vec::new(),
            created_at: now,
            updated_at: now,
        };
        let leg = StrategyLeg {
            id: "00000000-0000-4000-8000-000000000003".into(),
            strategy_id: strategy.id.clone(),
            option_contract_id: "00000000-0000-4000-8000-000000000004".into(),
            quantity: 1,
            position_type: PositionType::Long,
            premium: 1.0,
            strike: 100.0,
            expiration: now,
            option_type: OptionType::Call,
        };

        let repo = OptionStrategyRepository::new(pool.clone());
        assert!(repo.create_with_legs(&strategy, &[leg]).await.is_err());
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM option_strategies")
            .fetch_one(&pool)
            .await
            .expect("strategy count");
        assert_eq!(count, 0);
    }
}
