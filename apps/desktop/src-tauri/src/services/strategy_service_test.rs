use std::sync::Arc;

use domain::option::{PositionType, StrategyType};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

use crate::database::repositories::option_contract_repository::OptionContractRepository;
use crate::database::repositories::option_strategy_repository::OptionStrategyRepository;
use crate::database::repositories::strategy_leg_repository::StrategyLegRepository;
use crate::services::strategy_service::{
    CreateStrategyInput, CreateStrategyLegInput, StrategyService,
};

const WORKSPACE_ID: &str = "00000000-0000-4000-8000-000000000001";
const OTHER_WORKSPACE_ID: &str = "00000000-0000-4000-8000-000000000002";
const CONTRACT_A: &str = "00000000-0000-4000-8000-000000000003";
const CONTRACT_B: &str = "00000000-0000-4000-8000-000000000004";

async fn setup() -> (SqlitePool, StrategyService) {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(":memory:")
        .await
        .expect("test database");
    crate::database::migrations::run(&pool)
        .await
        .expect("migrations");
    sqlx::query("INSERT INTO workspaces (id, name) VALUES (?, 'Options')")
        .bind(WORKSPACE_ID)
        .execute(&pool)
        .await
        .expect("workspace fixture");
    sqlx::query(
            "INSERT INTO option_chains (id, workspace_id, symbol, underlying_price, as_of, data_source, created_at) VALUES ('00000000-0000-4000-8000-000000000005', ?, 'AAPL', 150, '2026-08-15T00:00:00Z', 'demo', '2026-08-15T00:00:00Z')",
        )
        .bind(WORKSPACE_ID)
        .execute(&pool)
        .await
        .expect("chain fixture");
    for (id, option_type, strike, bid, ask) in [
        (CONTRACT_A, "call", 150.0, 4.0, 5.0),
        (CONTRACT_B, "call", 155.0, 2.0, 3.0),
    ] {
        sqlx::query(
                "INSERT INTO option_contracts (id, workspace_id, chain_id, symbol, option_type, strike, expiration, contract_multiplier, bid, ask, last, volume, open_interest, implied_volatility, created_at, updated_at) VALUES (?, ?, '00000000-0000-4000-8000-000000000005', 'AAPL', ?, ?, '2026-09-15T00:00:00Z', 100, ?, ?, NULL, 0, 0, 0.25, '2026-08-15T00:00:00Z', '2026-08-15T00:00:00Z')",
            )
            .bind(id)
            .bind(WORKSPACE_ID)
            .bind(option_type)
            .bind(strike)
            .bind(bid)
            .bind(ask)
            .execute(&pool)
            .await
            .expect("contract fixture");
    }

    let service = StrategyService::new(
        Arc::new(OptionStrategyRepository::new(pool.clone())),
        Arc::new(StrategyLegRepository::new(pool.clone())),
        Arc::new(OptionContractRepository::new(pool.clone())),
    );
    (pool, service)
}

fn leg(contract_id: &str, quantity: i32, position_type: PositionType) -> CreateStrategyLegInput {
    CreateStrategyLegInput {
        contract_id: contract_id.into(),
        quantity,
        position_type,
    }
}

#[tokio::test]
async fn creates_reloads_and_cascade_deletes_a_strategy() {
    let (pool, service) = setup().await;
    let created = service
        .create_strategy(CreateStrategyInput {
            workspace_id: WORKSPACE_ID.into(),
            name: "  Call spread  ".into(),
            strategy_type: StrategyType::BullCallSpread,
            legs: vec![
                leg(CONTRACT_A, 1, PositionType::Long),
                leg(CONTRACT_B, 2, PositionType::Short),
            ],
        })
        .await
        .expect("create strategy");

    assert_eq!(created.strategy.name, "Call spread");
    assert_eq!(created.strategy.underlying, "AAPL");
    assert_eq!(created.strategy.total_cost, 100.0);
    assert_eq!(created.legs[0].premium, 5.0);
    assert_eq!(created.legs[1].strike, 155.0);
    let reloaded = service
        .get_strategy(&created.strategy.id)
        .await
        .expect("reload strategy");
    assert_eq!(reloaded.legs.len(), 2);
    let listed = service.list_strategies(WORKSPACE_ID).await.unwrap();
    assert_eq!(listed.len(), 1);

    service
        .delete_strategy(&created.strategy.id)
        .await
        .expect("delete strategy");
    let leg_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM strategy_legs")
        .fetch_one(&pool)
        .await
        .expect("leg count");
    assert_eq!(leg_count, 0);
}

#[tokio::test]
async fn rejects_empty_zero_quantity_and_cross_workspace_legs() {
    let (_, service) = setup().await;
    for (workspace_id, legs) in [
        (WORKSPACE_ID, Vec::new()),
        (WORKSPACE_ID, vec![leg(CONTRACT_A, 0, PositionType::Long)]),
        (
            OTHER_WORKSPACE_ID,
            vec![leg(CONTRACT_A, 1, PositionType::Long)],
        ),
    ] {
        assert!(service
            .create_strategy(CreateStrategyInput {
                workspace_id: workspace_id.into(),
                name: "Invalid".into(),
                strategy_type: StrategyType::Custom,
                legs,
            })
            .await
            .is_err());
    }
}
