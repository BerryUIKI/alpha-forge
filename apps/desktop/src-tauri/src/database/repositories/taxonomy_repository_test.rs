// Tests for the taxonomy financial repository.

use crate::database::repositories::asset_repository::AssetRepository;
use crate::database::repositories::taxonomy_repository::TaxonomyRepository;
use crate::database::repositories::test_support::setup_test_db;
use domain::financial::{
    AssetKind, AssetTaxonomyAssignmentInput, CreateAssetInput, CreateTaxonomyCategoryInput,
    CreateTaxonomyInput, InstrumentType, QuoteMode,
};

#[tokio::test]
async fn taxonomy_repository_creates_and_lists_system_taxonomies() {
    let pool = setup_test_db().await;
    let repo = TaxonomyRepository::new(pool.clone());

    // Migration 0020 seeds 6 system taxonomies.
    let listed = repo.list().await.expect("Failed to list taxonomies");
    assert_eq!(listed.len(), 6);
    assert!(listed
        .iter()
        .any(|t| t.id == "asset_classes" && t.is_system));

    let asset_classes = repo
        .get("asset_classes")
        .await
        .expect("Failed to get taxonomy")
        .expect("asset_classes should exist");
    assert_eq!(asset_classes.name, "Asset Classes");
    assert!(asset_classes.is_system);
    assert!(!asset_classes.is_single_select);

    let categories = repo
        .list_categories("asset_classes")
        .await
        .expect("Failed to list categories");
    assert_eq!(categories.len(), 79);
}

#[tokio::test]
async fn taxonomy_repository_creates_custom_taxonomy_with_category() {
    let pool = setup_test_db().await;
    let repo = TaxonomyRepository::new(pool.clone());

    let taxonomy = repo
        .create(CreateTaxonomyInput {
            name: "My Themes".to_string(),
            color: "#123456".to_string(),
            description: Some("Personal grouping".to_string()),
            is_system: false,
            is_single_select: false,
            sort_order: 500,
        })
        .await
        .expect("Failed to create taxonomy");

    assert_eq!(taxonomy.name, "My Themes");
    assert!(!taxonomy.is_system);

    let category = repo
        .create_category(CreateTaxonomyCategoryInput {
            taxonomy_id: taxonomy.id.clone(),
            parent_id: None,
            name: "AI Infrastructure".to_string(),
            key: "AI_INFRA".to_string(),
            color: "#654321".to_string(),
            description: None,
            sort_order: 1,
        })
        .await
        .expect("Failed to create category");

    assert_eq!(category.taxonomy_id, taxonomy.id);
    assert_eq!(category.key, "AI_INFRA");

    let fetched = repo
        .list_categories(&taxonomy.id)
        .await
        .expect("Failed to list custom categories");
    assert_eq!(fetched.len(), 1);
}

#[tokio::test]
async fn taxonomy_repository_assigns_asset_to_category_and_upserts() {
    let pool = setup_test_db().await;
    let taxonomy_repo = TaxonomyRepository::new(pool.clone());
    let asset_repo = AssetRepository::new(pool.clone());

    let asset = asset_repo
        .create(CreateAssetInput {
            kind: AssetKind::Investment,
            name: Some("Apple Inc".to_string()),
            display_code: Some("AAPL".to_string()),
            notes: None,
            is_active: true,
            quote_mode: QuoteMode::Market,
            quote_ccy: "USD".to_string(),
            instrument_type: Some(InstrumentType::Equity),
            instrument_symbol: Some("AAPL".to_string()),
            instrument_exchange_mic: Some("XNAS".to_string()),
            provider_config: None,
        })
        .await
        .expect("Failed to create asset");

    let assignment = taxonomy_repo
        .assign_asset(AssetTaxonomyAssignmentInput {
            asset_id: asset.id.clone(),
            taxonomy_id: "asset_classes".to_string(),
            category_id: "EQUITY".to_string(),
            weight: 10000,
            source: "manual".to_string(),
        })
        .await
        .expect("Failed to assign asset");

    assert_eq!(assignment.asset_id, asset.id);
    assert_eq!(assignment.category_id, "EQUITY");
    assert_eq!(assignment.weight, 10000);

    // Re-assigning to the same category updates weight, keeps one row.
    let updated = taxonomy_repo
        .assign_asset(AssetTaxonomyAssignmentInput {
            asset_id: asset.id.clone(),
            taxonomy_id: "asset_classes".to_string(),
            category_id: "EQUITY".to_string(),
            weight: 8000,
            source: "broker".to_string(),
        })
        .await
        .expect("Failed to re-assign asset");
    assert_eq!(updated.weight, 8000);

    let rows: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM asset_taxonomy_assignments WHERE asset_id = ? AND taxonomy_id = 'asset_classes'",
    )
    .bind(&asset.id)
    .fetch_one(&pool)
    .await
    .expect("Failed to count assignments");
    assert_eq!(rows, 1);

    // A second category produces a second assignment row.
    taxonomy_repo
        .assign_asset(AssetTaxonomyAssignmentInput {
            asset_id: asset.id.clone(),
            taxonomy_id: "asset_classes".to_string(),
            category_id: "EQUITY_PUBLIC".to_string(),
            weight: 2000,
            source: "manual".to_string(),
        })
        .await
        .expect("Failed to assign second category");

    let by_asset = taxonomy_repo
        .list_assignments_for_asset(&asset.id)
        .await
        .expect("Failed to list asset assignments");
    assert_eq!(by_asset.len(), 2);

    taxonomy_repo
        .remove_assignment(&by_asset[0].id)
        .await
        .expect("Failed to remove assignment");
    let remaining = taxonomy_repo
        .list_assignments_for_asset(&asset.id)
        .await
        .expect("Failed to list remaining assignments");
    assert_eq!(remaining.len(), 1);
}
