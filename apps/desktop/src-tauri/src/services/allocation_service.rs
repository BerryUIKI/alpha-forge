// Allocation service — actual vs target allocation by taxonomy.
//
// Computes actual allocation percentages from current holdings, compares
// them against allocation targets (weights) and surfaces the constraints
// (buy/sell/trade rules) that apply to a scope. Supports all three scope
// types: all, portfolio, and account.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use chrono::NaiveDate;
use domain::financial::{
    AllocationBreakdown, AllocationCategory, ConstraintSubjectType, HoldingsSummary, ScopeType,
    TaxonomyCategory,
};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

use crate::database::repositories::account_repository::AccountRepository;
use crate::database::repositories::allocation_target_repository::AllocationTargetRepository;
use crate::database::repositories::taxonomy_repository::TaxonomyRepository;
use crate::error::AppError;

use super::holdings_service::HoldingsService;

pub struct AllocationService {
    taxonomy_repo: Arc<TaxonomyRepository>,
    target_repo: Arc<AllocationTargetRepository>,
    account_repo: Arc<AccountRepository>,
    holdings_service: Arc<HoldingsService>,
}

impl AllocationService {
    pub fn new(
        taxonomy_repo: Arc<TaxonomyRepository>,
        target_repo: Arc<AllocationTargetRepository>,
        account_repo: Arc<AccountRepository>,
        holdings_service: Arc<HoldingsService>,
    ) -> Self {
        Self {
            taxonomy_repo,
            target_repo,
            account_repo,
            holdings_service,
        }
    }

    /// Compute allocation breakdown for a scope.
    pub async fn get_allocation(
        &self,
        scope_type: ScopeType,
        scope_id: Option<&str>,
        as_of_date: NaiveDate,
    ) -> Result<AllocationBreakdown, AppError> {
        let holdings_summaries = self
            .scope_holdings(scope_type, scope_id, as_of_date)
            .await?;

        // Aggregate holdings across all accounts in scope.
        let mut asset_market_values: HashMap<String, (Decimal, Decimal)> = HashMap::new();
        let mut total_market_value = Decimal::ZERO;
        let mut total_market_value_base = Decimal::ZERO;

        for summary in &holdings_summaries {
            for holding in &summary.holdings {
                let entry = asset_market_values
                    .entry(holding.asset_id.clone())
                    .or_default();
                entry.0 += holding.market_value;
                entry.1 += holding.market_value_base;
                total_market_value += holding.market_value;
                total_market_value_base += holding.market_value_base;
            }
        }

        let taxonomies = self.taxonomy_repo.list().await?;
        let mut categories: Vec<AllocationCategory> = Vec::new();

        for taxonomy in &taxonomies {
            let assignments = self
                .taxonomy_repo
                .list_assignments_by_taxonomy(&taxonomy.id)
                .await?;
            let cat_list = self.taxonomy_repo.list_categories(&taxonomy.id).await?;
            let cat_map: HashMap<String, TaxonomyCategory> =
                cat_list.into_iter().map(|c| (c.id.clone(), c)).collect();

            // Target weights for this taxonomy (first matching target that
            // references this taxonomy; weights are taxonomy-scoped).
            let targets = self.target_repo.list(false).await?;
            let target_bps_map: HashMap<String, i32> = {
                let mut map = HashMap::new();
                if let Some(matching_target) = targets.iter().find(|t| t.taxonomy_id == taxonomy.id)
                {
                    if let Ok(weights) = self.target_repo.list_weights(&matching_target.id).await {
                        for tw in &weights {
                            map.insert(tw.category_id.clone(), tw.target_bps);
                        }
                    }
                }
                map
            };

            // Aggregate market value by category.
            let mut category_values: HashMap<String, Decimal> = HashMap::new();
            let mut category_values_base: HashMap<String, Decimal> = HashMap::new();

            for assignment in &assignments {
                if let Some((mv, mvb)) = asset_market_values.get(&assignment.asset_id) {
                    *category_values
                        .entry(assignment.category_id.clone())
                        .or_default() += *mv;
                    *category_values_base
                        .entry(assignment.category_id.clone())
                        .or_default() += *mvb;
                }
            }

            for (cat_id, cat_mv) in &category_values {
                let cat = cat_map.get(cat_id);
                let cat_name = cat.map(|c| c.name.clone()).unwrap_or_default();
                let actual_bps = if !total_market_value.is_zero() {
                    ((*cat_mv * Decimal::from(10000)) / total_market_value)
                        .round_dp(0)
                        .to_i32()
                        .unwrap_or(0)
                } else {
                    0
                };
                let target_bps = target_bps_map.get(cat_id).copied();
                let difference_bps = match target_bps {
                    Some(t) => actual_bps - t,
                    None => 0,
                };
                let within_drift = target_bps
                    .map(|t| (actual_bps - t).abs() <= 500) // 5% drift band
                    .unwrap_or(true);

                categories.push(AllocationCategory {
                    category_id: cat_id.clone(),
                    category_name: cat_name,
                    taxonomy_id: taxonomy.id.clone(),
                    taxonomy_name: taxonomy.name.clone(),
                    actual_bps,
                    target_bps,
                    difference_bps,
                    market_value: *cat_mv,
                    market_value_base: *category_values_base.get(cat_id).unwrap_or(&Decimal::ZERO),
                    within_drift,
                });
            }
        }

        let assigned_mv: Decimal = categories.iter().map(|c| c.market_value).sum();
        let assigned_mv_base: Decimal = categories.iter().map(|c| c.market_value_base).sum();

        categories.sort_by_key(|b| std::cmp::Reverse(b.actual_bps));

        Ok(AllocationBreakdown {
            scope_type,
            scope_id: scope_id.map(|s| s.to_string()),
            total_market_value,
            total_market_value_base,
            categories,
            unassigned_market_value: total_market_value - assigned_mv,
            unassigned_market_value_base: total_market_value_base - assigned_mv_base,
        })
    }

    /// Surface every target constraint that applies to the scope.
    ///
    /// A constraint names a subject (asset, account, or category) and an
    /// action (buy, sell, or trade) that is either blocked or avoided for
    /// that subject. Only constraints whose subject is held within the scope
    /// are reported, so a rebalancing UI can warn before suggesting a trade.
    pub async fn check_constraints(
        &self,
        scope_type: ScopeType,
        scope_id: Option<&str>,
        as_of_date: NaiveDate,
    ) -> Result<Vec<String>, AppError> {
        let targets = self.target_repo.list(false).await?;
        let scope_holdings = self
            .scope_holdings(scope_type, scope_id, as_of_date)
            .await?;
        let allocation = self
            .get_allocation(scope_type, scope_id, as_of_date)
            .await?;

        // Indices of what is actually held within the scope.
        let held_assets: HashSet<String> = scope_holdings
            .iter()
            .flat_map(|s| s.holdings.iter().map(|h| h.asset_id.clone()))
            .collect();
        let held_categories: HashSet<String> = allocation
            .categories
            .iter()
            .filter(|c| c.actual_bps > 0)
            .map(|c| c.category_id.clone())
            .collect();

        let mut applicable = Vec::new();
        for target in &targets {
            let constraints = self.target_repo.list_constraints(&target.id).await?;
            for constraint in &constraints {
                let present = match constraint.subject_type {
                    ConstraintSubjectType::Asset => held_assets.contains(&constraint.subject_id),
                    ConstraintSubjectType::Category => {
                        held_categories.contains(&constraint.subject_id)
                    }
                    ConstraintSubjectType::Account => scope_id
                        .map(|id| id == constraint.subject_id)
                        .unwrap_or(false),
                };

                if present {
                    applicable.push(format!(
                        "'{}': {} on {} is {:?} ({:?})",
                        target.name,
                        constraint.action,
                        constraint.subject_id,
                        constraint.effect,
                        constraint.reason.as_deref().unwrap_or("no reason given"),
                    ));
                }
            }
        }

        Ok(applicable)
    }

    /// Load holdings for the given scope, mirroring `get_allocation`.
    async fn scope_holdings(
        &self,
        scope_type: ScopeType,
        scope_id: Option<&str>,
        as_of_date: NaiveDate,
    ) -> Result<Vec<HoldingsSummary>, AppError> {
        match scope_type {
            ScopeType::All => self.holdings_service.get_all_holdings(as_of_date).await,
            ScopeType::Portfolio => {
                let workspace_id = scope_id.ok_or_else(|| {
                    AppError::Validation("scope_id required for portfolio scope".to_string())
                })?;
                let accounts = self.account_repo.list_by_workspace(workspace_id).await?;
                let mut summaries = Vec::new();
                for account in &accounts {
                    if account.is_archived {
                        continue;
                    }
                    if let Ok(s) = self
                        .holdings_service
                        .get_holdings(&account.id, as_of_date)
                        .await
                    {
                        summaries.push(s);
                    }
                }
                Ok(summaries)
            }
            ScopeType::Account => {
                let account_id = scope_id.ok_or_else(|| {
                    AppError::Validation("scope_id required for account scope".to_string())
                })?;
                Ok(vec![
                    self.holdings_service
                        .get_holdings(account_id, as_of_date)
                        .await?,
                ])
            }
        }
    }
}
