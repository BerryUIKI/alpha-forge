// Tests for the allocation-target financial repository.

use rust_decimal::Decimal;

use crate::database::repositories::allocation_target_repository::AllocationTargetRepository;
use crate::database::repositories::test_support::setup_test_db;
use domain::financial::{
    AllocationTargetConstraintInput, AllocationTargetWeightInput, ConstraintAction,
    ConstraintEffect, ConstraintSubjectType, CreateAllocationTargetInput, ScopeType,
};

fn dec(value: &str) -> Decimal {
    Decimal::from_str_exact(value).expect("valid decimal")
}

fn target_input(name: &str) -> CreateAllocationTargetInput {
    CreateAllocationTargetInput {
        name: name.to_string(),
        scope_type: ScopeType::All,
        scope_id: None,
        taxonomy_id: "asset_classes".to_string(),
        trigger_type: "threshold".to_string(),
        drift_band_bps: 500,
        rebalance_goal: "nearest_band".to_string(),
        min_trade_amount: dec("0"),
        whole_shares_only: true,
        allow_sells: false,
        max_turnover_bps: Some(2000),
    }
}

#[tokio::test]
async fn allocation_target_repository_creates_target_with_weights_and_constraints() {
    let pool = setup_test_db().await;
    let repo = AllocationTargetRepository::new(pool.clone());

    let target = repo
        .create(target_input("Core Portfolio"))
        .await
        .expect("Failed to create target");

    assert_eq!(target.name, "Core Portfolio");
    assert_eq!(target.scope_type, ScopeType::All);
    assert_eq!(target.taxonomy_id, "asset_classes");
    assert!(target.whole_shares_only);
    assert_eq!(target.max_turnover_bps, Some(2000));
    assert!(target.archived_at.is_none());

    let listed = repo.list(false).await.expect("Failed to list targets");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, target.id);

    let weight = repo
        .add_weight(AllocationTargetWeightInput {
            target_id: target.id.clone(),
            taxonomy_id: "asset_classes".to_string(),
            category_id: "EQUITY".to_string(),
            target_bps: 6000,
            is_locked: false,
            is_required: true,
        })
        .await
        .expect("Failed to add weight");
    assert_eq!(weight.target_bps, 6000);

    let weights = repo
        .list_weights(&target.id)
        .await
        .expect("Failed to list weights");
    assert_eq!(weights.len(), 1);

    let constraint = repo
        .add_constraint(AllocationTargetConstraintInput {
            target_id: target.id.clone(),
            subject_type: ConstraintSubjectType::Category,
            subject_id: "EQUITY".to_string(),
            action: ConstraintAction::Sell,
            effect: ConstraintEffect::Avoid,
            reason: Some("Holding period target".to_string()),
            metadata_json: None,
        })
        .await
        .expect("Failed to add constraint");
    assert_eq!(constraint.action, ConstraintAction::Sell);
    assert_eq!(constraint.effect, ConstraintEffect::Avoid);

    let constraints = repo
        .list_constraints(&target.id)
        .await
        .expect("Failed to list constraints");
    assert_eq!(constraints.len(), 1);

    repo.archive(&target.id)
        .await
        .expect("Failed to archive target");
    let active = repo
        .list(false)
        .await
        .expect("Failed to list active targets");
    assert!(active.is_empty());
    let with_archived = repo
        .list(true)
        .await
        .expect("Failed to list archived targets");
    assert_eq!(with_archived.len(), 1);
}

#[tokio::test]
async fn allocation_target_repository_rejects_weight_with_wrong_taxonomy() {
    let pool = setup_test_db().await;
    let repo = AllocationTargetRepository::new(pool.clone());

    let target = repo
        .create(target_input("Taxonomy Guard"))
        .await
        .expect("Failed to create target");

    // The trigger in migration 0020 aborts weights whose taxonomy does not
    // match the owning target; the repository maps that to a typed error.
    let result = repo
        .add_weight(AllocationTargetWeightInput {
            target_id: target.id.clone(),
            taxonomy_id: "instrument_type".to_string(),
            category_id: "EQUITY_SECURITY".to_string(),
            target_bps: 5000,
            is_locked: false,
            is_required: true,
        })
        .await;

    assert!(
        result.is_err(),
        "mismatched weight taxonomy must be rejected"
    );
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("does not match target"));
}
