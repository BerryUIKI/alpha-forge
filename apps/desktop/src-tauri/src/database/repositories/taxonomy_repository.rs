// Financial repositories — taxonomies + categories + asset assignments.
//
// SQLx persistence for `taxonomies`, `taxonomy_categories`, and
// `asset_taxonomy_assignments` (migration 0020). Taxonomy categories use the
// composite natural key (taxonomy_id, id); assignments link an asset to a
// category within one taxonomy with a weight in basis points (0..10000).

use chrono::Utc;
use sqlx::SqlitePool;

use crate::database::repositories::financial_support::parse_timestamp;
use crate::error::AppError;
use domain::financial::{
    AssetTaxonomyAssignment, AssetTaxonomyAssignmentInput, CreateTaxonomyCategoryInput,
    CreateTaxonomyInput, Taxonomy, TaxonomyCategory,
};

pub struct TaxonomyRepository {
    pool: SqlitePool,
}

impl TaxonomyRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateTaxonomyInput) -> Result<Taxonomy, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO taxonomies
                (id, name, color, description, is_system, is_single_select, sort_order,
                 created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&input.name)
        .bind(&input.color)
        .bind(&input.description)
        .bind(input.is_system)
        .bind(input.is_single_select)
        .bind(input.sort_order)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to create taxonomy: {e}")))?;

        Ok(Taxonomy {
            id,
            name: input.name,
            color: input.color,
            description: input.description,
            is_system: input.is_system,
            is_single_select: input.is_single_select,
            sort_order: input.sort_order,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get(&self, id: &str) -> Result<Option<Taxonomy>, AppError> {
        let row = sqlx::query_as::<_, TaxonomyRow>(
            "SELECT id, name, color, description, is_system, is_single_select, sort_order,
                    created_at, updated_at
             FROM taxonomies WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to get taxonomy: {e}")))?;

        row.map(TryInto::try_into).transpose()
    }

    pub async fn list(&self) -> Result<Vec<Taxonomy>, AppError> {
        let rows = sqlx::query_as::<_, TaxonomyRow>(
            "SELECT id, name, color, description, is_system, is_single_select, sort_order,
                    created_at, updated_at
             FROM taxonomies ORDER BY sort_order, name",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to list taxonomies: {e}")))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    pub async fn create_category(
        &self,
        input: CreateTaxonomyCategoryInput,
    ) -> Result<TaxonomyCategory, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO taxonomy_categories
                (id, taxonomy_id, parent_id, name, key, color, description, sort_order,
                 created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&input.taxonomy_id)
        .bind(&input.parent_id)
        .bind(&input.name)
        .bind(&input.key)
        .bind(&input.color)
        .bind(&input.description)
        .bind(input.sort_order)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to create taxonomy category: {e}")))?;

        Ok(TaxonomyCategory {
            id,
            taxonomy_id: input.taxonomy_id,
            parent_id: input.parent_id,
            name: input.name,
            key: input.key,
            color: input.color,
            description: input.description,
            sort_order: input.sort_order,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn list_categories(
        &self,
        taxonomy_id: &str,
    ) -> Result<Vec<TaxonomyCategory>, AppError> {
        let rows = sqlx::query_as::<_, CategoryRow>(
            "SELECT id, taxonomy_id, parent_id, name, key, color, description, sort_order,
                    created_at, updated_at
             FROM taxonomy_categories WHERE taxonomy_id = ? ORDER BY sort_order, name",
        )
        .bind(taxonomy_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to list taxonomy categories: {e}")))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    pub async fn assign_asset(
        &self,
        input: AssetTaxonomyAssignmentInput,
    ) -> Result<AssetTaxonomyAssignment, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        // Upsert on the (asset, taxonomy, category) natural key so re-assigning
        // an asset to the same category updates the weight in place instead of
        // creating a competing row (the schema also enforces this uniquely).
        sqlx::query(
            "INSERT INTO asset_taxonomy_assignments
                (id, asset_id, taxonomy_id, category_id, weight, source, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT (asset_id, taxonomy_id, category_id) DO UPDATE SET
                 weight = excluded.weight,
                 source = excluded.source,
                 updated_at = excluded.updated_at",
        )
        .bind(&id)
        .bind(&input.asset_id)
        .bind(&input.taxonomy_id)
        .bind(&input.category_id)
        .bind(input.weight)
        .bind(&input.source)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to assign asset taxonomy: {e}")))?;

        let row = sqlx::query_as::<_, AssignmentRow>(
            "SELECT id, asset_id, taxonomy_id, category_id, weight, source, created_at, updated_at
             FROM asset_taxonomy_assignments
             WHERE asset_id = ? AND taxonomy_id = ? AND category_id = ?",
        )
        .bind(&input.asset_id)
        .bind(&input.taxonomy_id)
        .bind(&input.category_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to read taxonomy assignment: {e}")))?;

        row.try_into()
    }

    pub async fn list_assignments_for_asset(
        &self,
        asset_id: &str,
    ) -> Result<Vec<AssetTaxonomyAssignment>, AppError> {
        let rows = sqlx::query_as::<_, AssignmentRow>(
            "SELECT id, asset_id, taxonomy_id, category_id, weight, source, created_at, updated_at
             FROM asset_taxonomy_assignments WHERE asset_id = ? ORDER BY taxonomy_id, category_id",
        )
        .bind(asset_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            AppError::Internal(format!("failed to list asset taxonomy assignments: {e}"))
        })?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    pub async fn list_assignments_by_taxonomy(
        &self,
        taxonomy_id: &str,
    ) -> Result<Vec<AssetTaxonomyAssignment>, AppError> {
        let rows = sqlx::query_as::<_, AssignmentRow>(
            "SELECT id, asset_id, taxonomy_id, category_id, weight, source, created_at, updated_at
             FROM asset_taxonomy_assignments WHERE taxonomy_id = ? ORDER BY asset_id, category_id",
        )
        .bind(taxonomy_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("failed to list taxonomy assignments: {e}")))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    pub async fn remove_assignment(&self, id: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM asset_taxonomy_assignments WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                AppError::Internal(format!("failed to remove taxonomy assignment: {e}"))
            })?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct TaxonomyRow {
    id: String,
    name: String,
    color: String,
    description: Option<String>,
    is_system: bool,
    is_single_select: bool,
    sort_order: i32,
    created_at: String,
    updated_at: String,
}

impl TryFrom<TaxonomyRow> for Taxonomy {
    type Error = AppError;

    fn try_from(row: TaxonomyRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            name: row.name,
            color: row.color,
            description: row.description,
            is_system: row.is_system,
            is_single_select: row.is_single_select,
            sort_order: row.sort_order,
            created_at: parse_timestamp(&row.created_at, "taxonomy creation")?,
            updated_at: parse_timestamp(&row.updated_at, "taxonomy update")?,
        })
    }
}

#[derive(sqlx::FromRow)]
struct CategoryRow {
    id: String,
    taxonomy_id: String,
    parent_id: Option<String>,
    name: String,
    key: String,
    color: String,
    description: Option<String>,
    sort_order: i32,
    created_at: String,
    updated_at: String,
}

impl TryFrom<CategoryRow> for TaxonomyCategory {
    type Error = AppError;

    fn try_from(row: CategoryRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            taxonomy_id: row.taxonomy_id,
            parent_id: row.parent_id,
            name: row.name,
            key: row.key,
            color: row.color,
            description: row.description,
            sort_order: row.sort_order,
            created_at: parse_timestamp(&row.created_at, "taxonomy category creation")?,
            updated_at: parse_timestamp(&row.updated_at, "taxonomy category update")?,
        })
    }
}

#[derive(sqlx::FromRow)]
struct AssignmentRow {
    id: String,
    asset_id: String,
    taxonomy_id: String,
    category_id: String,
    weight: i32,
    source: String,
    created_at: String,
    updated_at: String,
}

impl TryFrom<AssignmentRow> for AssetTaxonomyAssignment {
    type Error = AppError;

    fn try_from(row: AssignmentRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            asset_id: row.asset_id,
            taxonomy_id: row.taxonomy_id,
            category_id: row.category_id,
            weight: row.weight,
            source: row.source,
            created_at: parse_timestamp(&row.created_at, "taxonomy assignment creation")?,
            updated_at: parse_timestamp(&row.updated_at, "taxonomy assignment update")?,
        })
    }
}
