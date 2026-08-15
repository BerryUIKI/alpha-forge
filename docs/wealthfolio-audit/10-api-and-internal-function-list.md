# Wealthfolio -- API and Internal Function List

> System inventory of every HTTP API endpoint, Tauri IPC command, and MCP agent tool.
> Generated 2026-08-12. All paths relative to repo root F:\dev\wealthfolio.

## Architecture Overview

```
Frontend (React) -> Adapter (tauri/web) -> Command wrapper
        |
Tauri IPC (desktop) | Axum HTTP (web mode)
        |
crates/core (business logic) -> crates/storage-sqlite (Diesel/ORM)
```

Three surfaces are documented in this file: HTTP API (Axum), Tauri IPC, MCP agent tools.

## Cross-cutting conventions

- **Auth (HTTP)**: When `auth` is configured, ALL `/api/v1/*` routes require a valid JWT (`auth::require_jwt` middleware).
- **Rate limiting (HTTP)**: login + OIDC endpoints limited to 5 req/60s per peer IP (`tower_governor`).
- **Security headers (HTTP)**: CSP, `X-Content-Type-Options: nosniff`, `Referrer-Policy: no-referrer` applied globally.
- **Feature gates**: `connect-sync` -> `connect` + `brokers_sync`; `device-sync` -> `device_sync`, `sync_crypto`, `device_enroll_service`.
- **Serialization**: JSON bodies use `camelCase`.
- **Errors (HTTP)**: `ApiResult<T>` -> 2xx JSON or `ApiError`.
- **Errors (Tauri)**: `CommandError::ServiceError(String)`; `CommandResult<T, E = CommandError>`.
- **MCP**: PAT-based auth (`Authorization: Bearer <pat>`), audit logging, scope-based permissions. Desktop-only.

# PART 1 - HTTP API (Axum, Web Mode)

Base path: `/api/v1`. Unless noted, all routes require JWT auth.

## 1.1 Auth / Public endpoints

| Method | Path | Handler | Mandatory args | Response | Errors |
|---|---|---|---|---|---|
| GET | `/healthz` | `healthz` | -- | `200 ok` | -- |
| GET | `/readyz` | `readyz` | -- | `200 ok` | -- |
| GET | `/auth/status` | `auth::auth_status` | -- | `200` auth status | -- |
| POST | `/auth/login` | `auth::login` | body `{username,password}` | `200 {token,...}` | 401, 429 |
| POST | `/auth/logout` | `auth::logout` | -- | `200` | -- |
| GET | `/auth/me` | `auth::auth_me` | -- | `200` user | 401 |
| GET | `/auth/oidc/login` | `oidc::oidc_login` | query `provider` | `200` redirect URL | 429 |
| GET | `/auth/oidc/logout` | `oidc::oidc_logout` | -- | `200` | -- |
| GET | `/auth/oidc/callback` | `oidc::oidc_callback` | query `code`,`state` | `200` session token | 400/429 |
| GET | `/api/v1/openapi.json` | (inline) | -- | `200` OpenAPI spec | -- |

**Permission**: none (public). **Rate limit**: login/oidc 5/60s.

## 1.2 Account CRUD

File: `apps/server/src/api/accounts.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/accounts` | `list_accounts` |
| POST | `/accounts` | `create_account` |
| PUT | `/accounts/{id}` | `update_account` |
| DELETE | `/accounts/{id}` | `delete_account` |

## 1.3 Portfolio (named groups) CRUD

File: `apps/server/src/api/portfolios.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/portfolios` | `list_portfolios` |
| POST | `/portfolios` | `create_portfolio` |
| GET | `/portfolios/{id}` | `get_portfolio` |
| PUT | `/portfolios/{id}` | `update_portfolio` |
| DELETE | `/portfolios/{id}` | `delete_portfolio` |

## 1.4 Settings + app info + update checks

File: `apps/server/src/api/settings.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/settings` | `get_settings` |
| PUT | `/settings` | `update_settings` |
| GET | `/settings/auto-update-enabled` | `is_auto_update_check_enabled` |
| GET | `/app/info` | `get_app_info` |
| GET | `/app/check-update` | `check_update` |

## 1.5 Data export

File: `apps/server/src/api/data_exports.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/utilities/export/{data_type}/{format}` | `export_data_route` |

## 1.6 Database backup management

File: `apps/server/src/api/database_backups.rs`

| Method | Path | Handler |
|---|---|---|
| POST | `/utilities/database/backup` | `backup_database_route` |
| GET | `/utilities/database/backups` | `list_backup_files_route` |
| GET | `/utilities/database/backups/{filename}/download` | `download_backup_file_route` |
| DELETE | `/utilities/database/backups/{filename}` | `delete_backup_file_route` |

## 1.7 Portfolio update + SSE events

File: `apps/server/src/api/portfolio.rs`

| Method | Path | Handler |
|---|---|---|
| POST | `/portfolio/update` | `update_portfolio` |
| POST | `/portfolio/recalculate` | `recalculate_portfolio` |
| GET | `/events/stream` | `stream_events` |

## 1.8 Holdings, valuations, allocations, snapshots

File: `apps/server/src/api/holdings/mod.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/holdings` | `handlers::get_holdings_for_account` |
| POST | `/holdings/query` | `handlers::get_holdings` |
| GET | `/holdings/list` | `handlers::get_holdings_list_for_account` |
| POST | `/holdings/list/query` | `handlers::get_holdings_list` |
| GET | `/holdings/item` | `handlers::get_holding` |
| GET | `/holdings/by-asset` | `handlers::get_asset_holdings` |
| GET | `/holdings/lots` | `handlers::get_asset_lots` |
| GET | `/valuations/history` | `handlers::get_historical_valuations` |
| POST | `/valuations/history/query` | `handlers::get_historical_valuations_for_scope` |
| GET | `/valuations/latest` | `handlers::get_latest_valuations` |
| POST | `/valuations/current/query` | `handlers::get_current_valuation` |
| GET | `/allocations` | `handlers::get_allocations_for_account` |
| POST | `/allocations/query` | `handlers::get_portfolio_allocations` |
| GET | `/allocations/holdings` | `handlers::get_holdings_by_allocation_for_account` |
| POST | `/allocations/holdings/query` | `handlers::get_holdings_by_allocation` |
| GET | `/snapshots` | `handlers::get_snapshots` |
| POST | `/snapshots` | `handlers::save_manual_holdings_handler` |
| DELETE | `/snapshots` | `handlers::delete_snapshot_handler` |
| GET | `/snapshots/holdings` | `handlers::get_snapshot_by_date` |
| POST | `/snapshots/import` | `handlers::import_holdings_csv_handler` |
| POST | `/snapshots/import/check` | `handlers::check_holdings_import_handler` |

## 1.9 Performance calculations

File: `apps/server/src/api/performance.rs`

| Method | Path | Handler |
|---|---|---|
| POST | `/performance/accounts/simple` | `calculate_accounts_simple_performance` |
| POST | `/performance/history` | `calculate_performance_history` |
| POST | `/performance/summary` | `calculate_performance_summary` |
| POST | `/performance/summaries` | `get_performance_summaries` |
| GET | `/income/summary` | `get_income_summary_for_account` |
| POST | `/income/summary/query` | `get_income_summary` |

## 1.10 Activity management + import pipeline

File: `apps/server/src/api/activities.rs`

| Method | Path | Handler |
|---|---|---|
| POST | `/activities/search` | `search_activities` |
| POST | `/activities` | `create_activity` |
| PUT | `/activities` | `update_activity` |
| POST | `/activities/bulk` | `save_activities` |
| DELETE | `/activities/{id}` | `delete_activity` |
| GET | `/activities/{id}/transfer-pair` | `get_transfer_pair_for_activity` |
| POST | `/activities/transfer-pair` | `save_internal_transfer_pair` |
| POST | `/activities/transfer-match-candidates` | `find_transfer_match_candidates` |
| POST | `/activities/link` | `link_transfer_activities` |
| POST | `/activities/unlink` | `unlink_transfer_activities` |
| POST | `/activities/import/check` | `check_activities_import` |
| POST | `/activities/import/assets/preview` | `preview_import_assets` |
| POST | `/activities/import` | `import_activities` |
| POST | `/activities/import/parse` | `parse_csv_endpoint` |
| GET | `/activities/import/mapping` | `get_account_import_mapping` |
| POST | `/activities/import/mapping` | `save_account_import_mapping` |
| GET | `/activities/import/templates` | `list_import_templates` |
| POST | `/activities/import/templates` | `save_import_template` |
| DELETE | `/activities/import/templates` | `delete_import_template` |
| GET | `/activities/import/templates/item` | `get_import_template` |
| POST | `/activities/import/templates/link` | `link_account_template` |
| POST | `/activities/import/check-duplicates` | `check_existing_duplicates` |

## 1.11 Goals CRUD + retirement simulations

File: `apps/server/src/api/goals.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/goals` | `get_goals` |
| POST | `/goals` | `create_goal` |
| PUT | `/goals` | `update_goal` |
| GET | `/goals/{id}` | `get_goal` |
| DELETE | `/goals/{id}` | `delete_goal` |
| GET | `/goals/{id}/funding` | `get_goal_funding` |
| PUT | `/goals/{id}/funding` | `save_goal_funding` |
| GET | `/goals/{id}/plan` | `get_goal_plan` |
| DELETE | `/goals/{id}/plan` | `delete_goal_plan` |
| POST | `/goals/{id}/refresh-summary` | `refresh_goal_summary` |
| POST | `/goals/refresh-summaries` | `refresh_all_goal_summaries` |
| GET | `/goals/{id}/retirement/overview` | `get_retirement_overview` |
| GET | `/goals/{id}/save-up/overview` | `get_save_up_overview` |
| POST | `/goals/save-up/preview` | `preview_save_up_overview` |
| POST | `/goals/plan` | `save_goal_plan` |
| POST | `/goals/retirement/projection` | `retirement_projection` |
| POST | `/goals/retirement/monte-carlo` | `retirement_monte_carlo` |
| POST | `/goals/retirement/stress-tests` | `retirement_stress_tests` |
| POST | `/goals/retirement/scenario-analysis` | `retirement_scenario_analysis` |
| POST | `/goals/retirement/decision-sensitivity-map` | `retirement_decision_sensitivity_map` |
| POST | `/goals/retirement/sequence-of-returns` | `retirement_sequence_of_returns` |

## 1.12 FX rates

File: `apps/server/src/api/exchange_rates.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/exchange-rates/latest` | `get_latest_exchange_rates` |
| PUT | `/exchange-rates` | `update_exchange_rate` |
| POST | `/exchange-rates` | `add_exchange_rate` |
| DELETE | `/exchange-rates/{id}` | `delete_exchange_rate` |

## 1.13 Market data providers, quotes, dividends, sync

File: `apps/server/src/api/market_data.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/exchanges` | `get_exchanges` |
| GET | `/providers` | `get_market_data_providers` |
| GET | `/providers/settings` | `get_market_data_provider_settings` |
| PUT | `/providers/settings` | `update_market_data_provider_settings` |
| GET | `/market-data/search` | `search_symbol` |
| GET | `/market-data/resolve-currency` | `resolve_symbol_quote` |
| GET | `/market-data/quotes/history` | `get_quote_history` |
| GET | `/market-data/dividends` | `fetch_dividends` |
| POST | `/market-data/quotes/latest` | `get_latest_quotes` |
| PUT | `/market-data/quotes/{symbol}` | `update_quote` |
| DELETE | `/market-data/quotes/id/{id}` | `delete_quote` |
| POST | `/market-data/quotes/check` | `check_quotes_import` |
| POST | `/market-data/quotes/import` | `import_quotes_csv` |
| POST | `/market-data/sync/history` | `sync_history_quotes` |
| POST | `/market-data/sync` | `sync_market_data` |

## 1.14 Asset management

File: `apps/server/src/api/assets.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/assets` | `list_assets` |
| POST | `/assets` | `create_asset` |
| DELETE | `/assets/{id}` | `delete_asset` |
| GET | `/assets/profile` | `get_asset_profile` |
| PUT | `/assets/profile/{id}` | `update_asset_profile` |
| PUT | `/assets/pricing-mode/{id}` | `update_quote_mode` |

## 1.15 Secret store (OS keyring)

File: `apps/server/src/api/secrets.rs`

| Method | Path | Handler |
|---|---|---|
| POST | `/secrets` | `set_secret` |
| GET | `/secrets` | `get_secret` |
| DELETE | `/secrets` | `delete_secret` |
| POST | `/addons/{addon_id}/secrets` | `set_addon_secret` |
| GET | `/addons/{addon_id}/secrets` | `get_addon_secret` |
| DELETE | `/addons/{addon_id}/secrets` | `delete_addon_secret` |

## 1.16 Addon network proxy

File: `apps/server/src/api/addon_network.rs`

| Method | Path | Handler |
|---|---|---|
| POST | `/addons/{addon_id}/network/request` | `addon_network_request` |

## 1.17 Contribution limits

File: `apps/server/src/api/limits.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/limits` | `get_contribution_limits` |
| POST | `/limits` | `create_contribution_limit` |
| PUT | `/limits/{id}` | `update_contribution_limit` |
| DELETE | `/limits/{id}` | `delete_contribution_limit` |
| GET | `/limits/{id}/deposits` | `calculate_deposits_for_contribution_limit` |

## 1.18 Addon management

File: `apps/server/src/api/addons.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/addons/installed` | `list_installed_addons_web` |
| POST | `/addons/install-zip` | `install_addon_zip_web` |
| POST | `/addons/toggle` | `toggle_addon_web` |
| DELETE | `/addons/{id}` | `uninstall_addon_web` |
| GET | `/addons/runtime/{id}` | `load_addon_for_runtime_web` |
| GET | `/addons/runtime/{id}/assets/{asset_id}` | `load_addon_asset_web` |
| GET | `/addons/enabled-on-startup` | `get_enabled_addons_on_startup_web` |
| POST | `/addons/extract` | `extract_addon_zip_web` |
| GET | `/addons/store/listings` | `fetch_addon_store_listings_web` |
| GET | `/addons/store/ratings` | `get_addon_ratings_web` |
| POST | `/addons/store/ratings` | `submit_addon_rating_web` |
| POST | `/addons/store/check-update` | `check_addon_update_web` |
| POST | `/addons/store/check-all` | `check_all_addon_updates_web` |
| POST | `/addons/store/update` | `update_addon_from_store_by_id_web` |
| POST | `/addons/store/staging/download` | `download_addon_to_staging_web` |
| POST | `/addons/store/install-from-staging` | `install_addon_from_staging_web` |
| POST | `/addons/network-approvals` | `update_addon_network_approvals_web` |
| DELETE | `/addons/store/staging` | `clear_addon_staging_web` |
| GET | `/addons/storage/{addon_id}/{key}` | `get_addon_storage_item_web` |
| PUT | `/addons/storage/{addon_id}/{key}` | `set_addon_storage_item_web` |
| DELETE | `/addons/storage/{addon_id}/{key}` | `delete_addon_storage_item_web` |

## 1.19 Taxonomy/classification management

File: `apps/server/src/api/taxonomies.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/taxonomies` | `get_taxonomies` |
| POST | `/taxonomies` | `create_taxonomy` |
| PUT | `/taxonomies` | `update_taxonomy` |
| GET | `/taxonomies/{id}` | `get_taxonomy` |
| DELETE | `/taxonomies/{id}` | `delete_taxonomy` |
| POST | `/taxonomies/categories` | `create_category` |
| PUT | `/taxonomies/categories` | `update_category` |
| DELETE | `/taxonomies/{taxonomyId}/categories/{categoryId}` | `delete_category` |
| POST | `/taxonomies/categories/move` | `move_category` |
| POST | `/taxonomies/import` | `import_taxonomy_json` |
| GET | `/taxonomies/{id}/export` | `export_taxonomy_json` |
| GET | `/taxonomies/assignments/asset/{assetId}` | `get_asset_taxonomy_assignments` |
| PUT | `/taxonomies/assignments/asset/{assetId}/taxonomy/{taxonomyId}` | `replace_asset_taxonomy_assignments` |
| POST | `/taxonomies/assignments` | `assign_asset_to_category` |
| DELETE | `/taxonomies/assignments/{id}` | `remove_asset_taxonomy_assignment` |
| GET | `/taxonomies/migration/status` | `get_migration_status` |
| POST | `/taxonomies/migration/run` | `migrate_legacy_classifications` |

## 1.20 Net worth

File: `apps/server/src/api/net_worth.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/net-worth` | `get_net_worth` |
| GET | `/net-worth/history` | `get_net_worth_history` |

## 1.21 Alternative assets

File: `apps/server/src/api/alternative_assets.rs`

| Method | Path | Handler |
|---|---|---|
| POST | `/alternative-assets` | `create_alternative_asset` |
| PUT | `/alternative-assets/{id}/valuation` | `update_alternative_asset_valuation` |
| DELETE | `/alternative-assets/{id}` | `delete_alternative_asset` |
| POST | `/alternative-assets/{id}/link-liability` | `link_liability` |
| DELETE | `/alternative-assets/{id}/link-liability` | `unlink_liability` |
| PUT | `/alternative-assets/{id}/metadata` | `update_alternative_asset_metadata` |
| GET | `/alternative-holdings` | `get_alternative_holdings` |

## 1.22 AI provider management

File: `apps/server/src/api/ai_providers.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/ai/providers` | `get_ai_providers` |
| PUT | `/ai/providers/settings` | `update_provider_settings` |
| POST | `/ai/providers/default` | `set_default_provider` |
| GET | `/ai/providers/{provider_id}/models` | `list_models` |

## 1.23 AI chat streaming

File: `apps/server/src/api/ai_chat.rs`

| Method | Path | Handler |
|---|---|---|
| POST | `/ai/chat/stream` | `stream_chat` |
| GET | `/ai/threads` | `list_threads` |
| GET | `/ai/threads/{id}` | `get_thread` |
| PUT | `/ai/threads/{id}` | `update_thread` |
| DELETE | `/ai/threads/{id}` | `delete_thread` |
| GET | `/ai/threads/{id}/messages` | `get_thread_messages` |
| PATCH | `/ai/tool-result` | `update_tool_result` |
| GET | `/ai/threads/{id}/tags` | `get_tags` |
| POST | `/ai/threads/{id}/tags` | `add_tag` |
| DELETE | `/ai/threads/{id}/tags/{tag}` | `remove_tag` |

## 1.24 System health checks

File: `apps/server/src/api/health.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/health/status` | `get_health_status` |
| POST | `/health/check` | `run_health_checks` |
| POST | `/health/dismiss` | `dismiss_health_issue` |
| POST | `/health/restore` | `restore_health_issue` |
| GET | `/health/dismissed` | `get_dismissed_health_issues` |
| POST | `/health/fix` | `execute_health_fix` |
| GET | `/health/config` | `get_health_config` |
| PUT | `/health/config` | `update_health_config` |

## 1.25 Custom market data providers

File: `apps/server/src/api/custom_providers.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/custom-providers` | `get_custom_providers` |
| POST | `/custom-providers` | `create_custom_provider` |
| PUT | `/custom-providers/{id}` | `update_custom_provider` |
| DELETE | `/custom-providers/{id}` | `delete_custom_provider` |
| POST | `/custom-providers/test-source` | `test_custom_provider_source` |

## 1.26 Spending management (~40 endpoints)

File: `apps/server/src/api/spending.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/spending/settings` | `get_spending_settings` |
| PUT | `/spending/settings` | `update_spending_settings` |
| GET | `/spending/cash-activities` | `list_cash_activities` |
| POST | `/spending/cash-activities/search` | `search_cash_activities` |
| PUT | `/spending/cash-activities/{activity_id}/event` | `set_activity_event` |
| GET | `/spending/activities/{activity_id}/assignments` | `get_activity_assignments` |
| PUT | `/spending/activities/{activity_id}/assignments` | `assign_activity_category` |
| DELETE | `/spending/activities/{activity_id}/assignments/{taxonomy_id}` | `unassign_activity_category` |
| GET | `/spending/activities/{activity_id}/splits` | `get_activity_splits` |
| PUT | `/spending/activities/{activity_id}/splits` | `replace_activity_splits` |
| DELETE | `/spending/activities/{activity_id}/splits` | `clear_activity_splits` |
| POST | `/spending/assignments/bulk` | `bulk_assign_categories` |
| GET | `/spending/rules` | `list_categorization_rules` |
| POST | `/spending/rules` | `create_categorization_rule` |
| PUT | `/spending/rules/{id}` | `update_categorization_rule` |
| DELETE | `/spending/rules/{id}` | `delete_categorization_rule` |
| POST | `/spending/rules/rerun` | `rerun_categorization_rules` |
| GET | `/spending/rule-presets` | `list_rule_presets` |
| POST | `/spending/rule-presets/{preset_id}/import` | `import_rule_preset` |
| DELETE | `/spending/rule-presets/{preset_id}` | `remove_rule_preset` |
| GET | `/spending/event-types` | `list_event_types` |
| POST | `/spending/event-types` | `create_event_type` |
| PUT | `/spending/event-types/{id}` | `update_event_type` |
| DELETE | `/spending/event-types/{id}` | `delete_event_type` |
| GET | `/spending/events` | `list_events` |
| POST | `/spending/events` | `create_event` |
| PUT | `/spending/events/{id}` | `update_event` |
| DELETE | `/spending/events/{id}` | `delete_event` |
| GET | `/spending/budget` | `get_budget` |
| POST | `/spending/budget/targets` | `upsert_budget_target` |
| DELETE | `/spending/budget/targets/{id}` | `delete_budget_target` |
| POST | `/spending/budget/rollovers` | `upsert_budget_rollover_setting` |
| DELETE | `/spending/budget/rollovers/{id}` | `delete_budget_rollover_setting` |
| POST | `/spending/budget/groups` | `create_budget_group` |
| POST | `/spending/budget/groups/reset` | `reset_budget_groups` |
| PUT | `/spending/budget/groups/{id}` | `update_budget_group` |
| DELETE | `/spending/budget/groups/{id}` | `delete_budget_group` |
| POST | `/spending/budget/group-assignments` | `assign_category_to_group` |
| POST | `/spending/budget/copy` | `copy_budget_targets` |
| POST | `/spending/report` | `get_spending_report` |
| POST | `/spending/insight` | `get_spending_insight` |
| POST | `/spending/event-spending-summaries` | `get_event_spending_summaries` |

## 1.27 Allocation targets, drift, rebalance

File: `apps/server/src/api/allocation_targets.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/allocation-targets` | `list_targets` |
| POST | `/allocation-targets` | `create_target` |
| POST | `/allocation-targets/save-with-weights` | `save_target_with_weights` |
| GET | `/allocation-targets/{id}` | `get_target` |
| PUT | `/allocation-targets/{id}` | `update_target` |
| DELETE | `/allocation-targets/{id}` | `delete_target` |
| POST | `/allocation-targets/{id}/archive` | `archive_target` |
| GET | `/allocation-targets/{id}/weights` | `list_weights` |
| POST | `/allocation-targets/{id}/weights` | `save_weights` |
| GET | `/allocation-targets/{id}/constraints` | `list_target_constraints_handler` |
| POST | `/allocation-targets/{id}/constraints` | `save_target_constraints_handler` |
| POST | `/allocation-targets/{id}/drift` | `get_drift_for_target` |
| POST | `/allocation-targets/rebalance/calculate` | `calculate_plan` |

## 1.28 MCP agent access

File: `apps/server/src/api/agent_access.rs`

| Method | Path | Handler |
|---|---|---|
| GET | `/agent-access/status` | `status` |
| GET | `/agent-access/tokens` | `list_tokens` |
| POST | `/agent-access/tokens` | `create_token` |
| DELETE | `/agent-access/tokens/{id}` | `delete_token` |
| GET | `/agent-access/audit` | `list_audit` |
| POST | `/agent-access/audit/purge` | `purge_audit` |

## 1.29 Connect (Cloud Sync) - Feature: connect-sync or device-sync

File: `apps/server/src/api/connect.rs` (1647 lines)

| Method | Path | Handler |
|---|---|---|
| POST | `/connect/session` | `store_sync_session` |
| POST | `/connect/post-login-bootstrap` | `post_login_bootstrap` |
| DELETE | `/connect/session` | `clear_sync_session` |
| GET | `/connect/session/status` | `get_sync_session_status` |
| GET | `/connect/session/restore` | `restore_sync_session` |
| GET | `/connect/connections` | `list_broker_connections` |
| GET | `/connect/accounts` | `list_broker_accounts` |
| POST | `/connect/sync` | `sync_broker_data` |
| POST | `/connect/sync/connections` | `sync_broker_connections` |
| POST | `/connect/sync/accounts` | `sync_broker_accounts` |
| POST | `/connect/sync/activities` | `sync_broker_activities` |
| GET | `/connect/synced-accounts` | `get_synced_accounts` |
| GET | `/connect/platforms` | `get_platforms` |
| GET | `/connect/sync-states` | `get_broker_sync_states` |
| GET | `/connect/import-runs` | `get_import_runs` |
| GET | `/connect/broker-sync-profile` | `get_broker_sync_profile` |
| POST | `/connect/broker-sync-profile` | `save_broker_sync_profile_rules` |
| GET | `/connect/plans` | `get_subscription_plans` |
| GET | `/connect/plans/public` | `get_subscription_plans_public` |
| GET | `/connect/user` | `get_user_info` |
| GET | `/connect/device/sync-state` | `get_device_sync_state` |
| POST | `/connect/device/enable` | `enable_device_sync` |
| DELETE | `/connect/device/sync-data` | `clear_device_sync_data` |
| POST | `/connect/device/reinitialize` | `reinitialize_device_sync` |
| GET | `/connect/device/engine-status` | `get_device_sync_engine_status` |
| GET | `/connect/device/pairing-source-status` | `get_device_sync_pairing_source_status` |
| GET | `/connect/device/bootstrap-overwrite-check` | `get_device_sync_bootstrap_overwrite_check` |
| POST | `/connect/device/reconcile-ready-state` | `reconcile_device_sync_ready_state` |
| POST | `/connect/device/bootstrap-snapshot` | `bootstrap_device_snapshot` |
| POST | `/connect/device/trigger-cycle` | `trigger_device_sync_cycle` |
| POST | `/connect/device/start-background` | `start_device_sync_background_engine` |
| POST | `/connect/device/stop-background` | `stop_device_sync_background_engine` |
| POST | `/connect/device/generate-snapshot` | `generate_device_snapshot_now` |
| POST | `/connect/device/cancel-snapshot` | `cancel_device_snapshot_upload` |

## 1.30 Device Sync - Feature: device-sync

File: `apps/server/src/api/device_sync.rs` (838 lines)

| Method | Path | Handler |
|---|---|---|
| POST | `/sync/device/register` | `register_device` |
| GET | `/sync/device/current` | `get_current_device` |
| GET | `/sync/devices` | `list_devices` |
| GET | `/sync/device/{device_id}` | `get_device_endpoint` |
| PATCH | `/sync/device/{device_id}` | `update_device_endpoint` |
| DELETE | `/sync/device/{device_id}` | `delete_device_endpoint` |
| POST | `/sync/device/{device_id}/revoke` | `revoke_device_endpoint` |
| POST | `/sync/keys/initialize` | `initialize_team_keys` |
| POST | `/sync/keys/initialize/commit` | `commit_initialize_team_keys` |
| POST | `/sync/keys/rotate` | `rotate_team_keys` |
| POST | `/sync/keys/rotate/commit` | `commit_rotate_team_keys` |
| POST | `/sync/team/reset` | `reset_team_sync` |
| POST | `/sync/pairing` | `create_pairing` |
| GET | `/sync/pairing/{pairing_id}` | `get_pairing` |
| POST | `/sync/pairing/{pairing_id}/approve` | `approve_pairing` |
| POST | `/sync/pairing/{pairing_id}/complete` | `complete_pairing` |
| POST | `/sync/pairing/{pairing_id}/cancel` | `cancel_pairing` |
| POST | `/sync/pairing/claim` | `claim_pairing` |
| GET | `/sync/pairing/{pairing_id}/messages` | `get_pairing_messages` |
| POST | `/sync/pairing/{pairing_id}/confirm` | `confirm_pairing_endpoint` |
| POST | `/sync/pairing/complete-with-transfer` | `complete_pairing_with_transfer` |
| POST | `/sync/pairing/confirm-with-bootstrap` | `confirm_pairing_with_bootstrap` |
| POST | `/sync/pairing/flow/begin` | `begin_pairing_confirm` |
| POST | `/sync/pairing/flow/state` | `get_pairing_flow_state` |
| POST | `/sync/pairing/flow/approve-overwrite` | `approve_pairing_overwrite_endpoint` |
| POST | `/sync/pairing/flow/cancel` | `cancel_pairing_flow` |

## 1.31 Sync Crypto - Feature: device-sync

File: `apps/server/src/api/sync_crypto.rs` (200 lines)

| Method | Path | Handler |
|---|---|---|
| POST | `/sync/crypto/generate-root-key` | `generate_root_key` |
| POST | `/sync/crypto/derive-dek` | `derive_dek` |
| POST | `/sync/crypto/generate-keypair` | `generate_keypair` |
| POST | `/sync/crypto/compute-shared-secret` | `compute_shared_secret` |
| POST | `/sync/crypto/derive-session-key` | `derive_session_key` |
| POST | `/sync/crypto/encrypt` | `encrypt` |
| POST | `/sync/crypto/decrypt` | `decrypt` |
| POST | `/sync/crypto/generate-pairing-code` | `generate_pairing_code` |
| POST | `/sync/crypto/hash-pairing-code` | `hash_pairing_code` |
| POST | `/sync/crypto/hmac-sha256` | `hmac_sha256` |
| POST | `/sync/crypto/compute-sas` | `compute_sas` |
| POST | `/sync/crypto/generate-device-id` | `generate_device_id` |

## 1.32 MCP (Agent Access) - requires config.mcp_enabled

PAT-based auth. Desktop-only. Files in `apps/server/src/mcp/`

| Method | Path | Summary |
|---|---|---|
| GET | `/mcp` | SSE transport for MCP protocol |
| POST | `/mcp` | MCP JSON-RPC messages |

## 1.33 Error Codes (HTTP)

| Code | Meaning |
|---|---|
| 200 | OK - successful response |
| 204 | No Content - delete success |
| 400 | Bad Request - invalid args/body |
| 401 | Unauthorized - missing/invalid JWT |
| 404 | Not Found - unknown resource |
| 429 | Too Many Requests - rate limit exceeded |
| 500 | Internal Server Error |

# PART 2 - Tauri IPC Commands (Desktop)

All commands are `#[tauri::command]` functions in `apps/tauri/src/commands/`.
Accessed via `invoke('cmd_name', {args})` from the frontend.
Error type: `CommandError::ServiceError(String)`; `CommandResult<T>`.

## Account CRUD (4 commands)

File: `apps/tauri/src/commands/account.rs`

- `get_accounts`
- `create_account`
- `update_account`
- `delete_account`

## Activity management + import (22 commands)

File: `apps/tauri/src/commands/activity.rs`

- `search_activities`
- `create_activity`
- `update_activity`
- `delete_activity`
- `get_transfer_pair_for_activity`
- `find_transfer_match_candidates`
- `save_internal_transfer_pair`
- `link_transfer_activities`
- `unlink_transfer_activities`
- `save_activities`
- `get_account_import_mapping`
- `save_account_import_mapping`
- `link_account_template`
- `list_import_templates`
- `get_import_template`
- `save_import_template`
- `delete_import_template`
- `check_activities_import`
- `preview_import_assets`
- `import_activities`
- `check_existing_duplicates`
- `parse_csv`

## Addon management (18 commands)

File: `apps/tauri/src/commands/addon.rs`

- `install_addon_zip`
- `list_installed_addons`
- `toggle_addon`
- `uninstall_addon`
- `load_addon_for_runtime`
- `load_addon_asset`
- `get_enabled_addons_on_startup`
- `extract_addon_zip`
- `check_addon_update`
- `check_all_addon_updates`
- `update_addon_from_store_by_id`
- `fetch_addon_store_listings`
- `download_addon_to_staging`
- `install_addon_from_staging`
- `update_addon_network_approvals`
- `clear_addon_staging`
- `submit_addon_rating`
- `get_addon_storage_item`
- `set_addon_storage_item`
- `delete_addon_storage_item`

## Addon network proxy (1 commands)

File: `apps/tauri/src/commands/addon_network.rs`

- `addon_network_request`

## AI chat streaming (10 commands)

File: `apps/tauri/src/commands/ai_chat.rs`

- `stream_ai_chat`
- `list_ai_threads`
- `get_ai_thread`
- `get_ai_thread_messages`
- `update_ai_thread`
- `delete_ai_thread`
- `add_ai_thread_tag`
- `remove_ai_thread_tag`
- `get_ai_thread_tags`
- `update_tool_result`

## AI provider management (4 commands)

File: `apps/tauri/src/commands/ai_providers.rs`

- `get_ai_providers`
- `update_ai_provider_settings`
- `set_default_ai_provider`
- `list_ai_models`

## Allocation targets, drift, rebalance (13 commands)

File: `apps/tauri/src/commands/allocation_targets.rs`

- `list_allocation_targets`
- `get_allocation_target`
- `create_allocation_target`
- `update_allocation_target`
- `archive_allocation_target`
- `delete_allocation_target`
- `list_allocation_target_weights`
- `save_allocation_target_weights`
- `save_allocation_target_with_weights`
- `list_target_constraints`
- `save_target_constraints`
- `get_allocation_target_drift`
- `calculate_rebalance_plan`

## Alternative assets + net worth (10 commands)

File: `apps/tauri/src/commands/alternative_assets.rs`

- `create_alternative_asset`
- `update_alternative_asset_valuation`
- `update_alternative_asset_metadata`
- `delete_alternative_asset`
- `link_liability`
- `unlink_liability`
- `get_alternative_holdings`
- `get_net_worth`
- `get_net_worth_history`

## Asset management (6 commands)

File: `apps/tauri/src/commands/asset.rs`

- `get_asset_profile`
- `get_assets`
- `update_asset_profile`
- `update_quote_mode`
- `create_asset`
- `delete_asset`

## Custom market data providers (5 commands)

File: `apps/tauri/src/commands/custom_provider.rs`

- `get_custom_providers`
- `create_custom_provider`
- `update_custom_provider`
- `delete_custom_provider`
- `test_custom_provider_source`

## Retirement/FIRE calculations (7 commands)

File: `apps/tauri/src/commands/fire.rs`

- `calculate_retirement_projection`
- `run_retirement_monte_carlo`
- `run_retirement_stress_tests`
- `run_retirement_scenario_analysis`
- `run_retirement_sorr`
- `run_retirement_decision_sensitivity_map`

## Goal management + retirement overview (14 commands)

File: `apps/tauri/src/commands/goal.rs`

- `get_goals`
- `get_goal`
- `create_goal`
- `update_goal`
- `delete_goal`
- `get_goal_funding`
- `save_goal_funding`
- `get_goal_plan`
- `save_goal_plan`
- `delete_goal_plan`
- `refresh_goal_summary`
- `refresh_all_goal_summaries`
- `get_retirement_overview`
- `get_save_up_overview`
- `preview_save_up_overview`

## Health checks (10 commands)

File: `apps/tauri/src/commands/health.rs`

- `get_health_status`
- `run_health_checks`
- `dismiss_health_issue`
- `restore_health_issue`
- `get_dismissed_health_issues`
- `execute_health_fix`
- `get_health_config`
- `update_health_config`

## Contribution limits (5 commands)

File: `apps/tauri/src/commands/limits.rs`

- `get_contribution_limits`
- `create_contribution_limit`
- `update_contribution_limit`
- `delete_contribution_limit`
- `calculate_deposits_for_contribution_limit`

## Market data (13 commands)

File: `apps/tauri/src/commands/market_data.rs`

- `search_symbol`
- `sync_market_data`
- `synch_quotes`
- `update_quote`
- `delete_quote`
- `get_quote_history`
- `get_latest_quotes`
- `get_market_data_providers`
- `check_quotes_import`
- `import_quotes_csv`
- `resolve_symbol_quote`
- `get_exchanges`
- `fetch_dividends`

## MCP agent access (desktop-only) (11 commands)

File: `apps/tauri/src/commands/mcp.rs`

- `mcp_get_status`
- `mcp_set_enabled`
- `mcp_set_auto_start`
- `mcp_start`
- `mcp_stop`
- `mcp_set_audit_enabled`
- `mcp_list_audit_log`
- `mcp_list_tokens`
- `mcp_create_token`
- `mcp_delete_token`
- `mcp_purge_audit_log`

## Platform detection (3 commands)

File: `apps/tauri/src/commands/platform.rs`

- `get_platform`
- `is_mobile`
- `is_desktop`

## Portfolio, holdings, valuations, performance (25 commands)

File: `apps/tauri/src/commands/portfolio.rs`

- `recalculate_portfolio`
- `update_portfolio`
- `get_holdings`
- `get_holdings_list`
- `get_holding`
- `get_asset_holdings`
- `get_asset_lots`
- `get_portfolio_allocations`
- `get_holdings_by_allocation`
- `get_historical_valuations`
- `get_latest_valuations`
- `get_current_valuation`
- `get_income_summary`
- `calculate_accounts_simple_performance`
- `calculate_performance_history`
- `calculate_performance_summary`
- `get_performance_summaries`
- `save_manual_holdings`
- `check_holdings_import`
- `import_holdings_csv`
- `get_snapshots`
- `get_snapshot_by_date`
- `delete_snapshot`

## Portfolio (named groups) CRUD (5 commands)

File: `apps/tauri/src/commands/portfolios.rs`

- `get_portfolios`
- `get_portfolio`
- `create_portfolio`
- `update_portfolio_entry`
- `delete_portfolio_entry`

## Market data provider settings (2 commands)

File: `apps/tauri/src/commands/providers_settings.rs`

- `get_market_data_providers_settings`
- `update_market_data_provider_settings`

## Secret store (OS keyring) (6 commands)

File: `apps/tauri/src/commands/secrets.rs`

- `set_secret`
- `get_secret`
- `delete_secret`
- `set_addon_secret`
- `get_addon_secret`
- `delete_addon_secret`

## Settings + exchange rates (7 commands)

File: `apps/tauri/src/commands/settings.rs`

- `get_settings`
- `is_auto_update_check_enabled`
- `update_settings`
- `update_exchange_rate`
- `get_latest_exchange_rates`
- `add_exchange_rate`
- `delete_exchange_rate`

## Spending management (32 commands)

File: `apps/tauri/src/commands/spending.rs`

- `get_spending_settings`
- `update_spending_settings`
- `list_cash_activities`
- `search_cash_activities`
- `set_activity_event`
- `get_activity_assignments`
- `assign_activity_category`
- `unassign_activity_category`
- `get_activity_splits`
- `replace_activity_splits`
- `clear_activity_splits`
- `bulk_assign_categories`
- `list_categorization_rules`
- `create_categorization_rule`
- `update_categorization_rule`
- `delete_categorization_rule`
- `rerun_categorization_rules`
- `list_rule_presets`
- `import_rule_preset`
- `remove_rule_preset`
- `list_event_types`
- `create_event_type`
- `update_event_type`
- `delete_event_type`
- `list_events`
- `create_event`
- `update_event`
- `delete_event`
- `get_budget`
- `upsert_budget_target`
- `delete_budget_target`
- `upsert_budget_rollover_setting`
- `delete_budget_rollover_setting`
- `create_budget_group`
- `update_budget_group`
- `delete_budget_group`
- `assign_category_to_group`
- `reset_budget_groups`
- `copy_budget_targets`
- `get_spending_report`
- `get_spending_insight`
- `get_event_spending_summaries`

## Taxonomy/classification (16 commands)

File: `apps/tauri/src/commands/taxonomy.rs`

- `get_taxonomies`
- `get_taxonomy`
- `create_taxonomy`
- `update_taxonomy`
- `delete_taxonomy`
- `create_category`
- `update_category`
- `delete_category`
- `move_category`
- `import_taxonomy_json`
- `export_taxonomy_json`
- `get_asset_taxonomy_assignments`
- `assign_asset_to_category`
- `replace_asset_taxonomy_assignments`
- `remove_asset_taxonomy_assignment`
- `get_migration_status`
- `migrate_legacy_classifications`

## File export, DB backup, app info, updates (12 commands)

File: `apps/tauri/src/commands/utilities.rs`

- `save_text_file_with_dialog`
- `save_file_with_dialog`
- `write_pending_export_text_file`
- `write_pending_export_file`
- `export_data_file`
- `open_external_url`
- `get_app_info`
- `check_for_updates`
- `install_app_update`
- `backup_database`
- `backup_database_to_pending_export`
- `backup_database_to_path`
- `restore_database`

## Broker Sync (14 commands) - Feature: connect-sync

File: `apps/tauri/src/commands/brokers_sync.rs` (402 lines)

- `sync_broker_data`
- `broker_ingest_run`
- `get_synced_accounts`
- `get_platforms`
- `list_broker_connections`
- `list_broker_accounts`
- `get_subscription_plans`
- `get_subscription_plans_public`
- `get_user_info`
- `get_broker_sync_states`
- `get_broker_ingest_states`
- `get_import_runs`
- `get_data_import_runs`
- `get_broker_sync_profile`
- `save_broker_sync_profile_rules`

## Device Enroll (4 commands) - Feature: device-sync

File: `apps/tauri/src/commands/device_enroll_service.rs` (132 lines)

- `get_device_sync_state`
- `enable_device_sync`
- `clear_device_sync_data`
- `reinitialize_device_sync`

## Sync Crypto (12 commands) - Feature: device-sync

File: `apps/tauri/src/commands/sync_crypto.rs` (70 lines)

- `sync_generate_root_key`
- `sync_derive_dek`
- `sync_generate_keypair`
- `sync_compute_shared_secret`
- `sync_derive_session_key`
- `sync_encrypt`
- `sync_decrypt`
- `sync_generate_pairing_code`
- `sync_hash_pairing_code`
- `sync_hmac_sha256`
- `sync_compute_sas`
- `sync_generate_device_id`

## Wealthfolio Connect (4 commands) - Feature: connect-sync or device-sync

File: `apps/tauri/src/commands/wealthfolio_connect.rs` (501 lines)

- `store_sync_session`
- `post_login_bootstrap`
- `clear_sync_session`
- `restore_sync_session`

## Device Sync (~40 commands) - Feature: device-sync

File: `apps/tauri/src/commands/device_sync/mod.rs` (70KB+)

- `enroll_device`
- `get_device`
- `list_devices`
- `update_device`
- `delete_device`
- `revoke_device`
- `initialize_team_keys`
- `commit_initialize_team_keys`
- `rotate_team_keys`
- `commit_rotate_team_keys`
- `reset_team_sync`
- `sync_engine_status`
- `device_sync_bootstrap_overwrite_check`
- `sync_trigger_cycle`
- `device_sync_start_background_engine`
- `device_sync_stop_background_engine`
- `device_sync_generate_snapshot_now`
- `device_sync_cancel_snapshot_upload`
- `device_sync_engine_status`
- `device_sync_pairing_source_status`
- `device_sync_reconcile_ready_state`
- `device_sync_bootstrap_snapshot_if_needed`
- `create_pairing`
- `get_pairing`
- `approve_pairing`
- `complete_pairing`
- `cancel_pairing`
- `claim_pairing`
- `get_pairing_messages`
- `complete_pairing_with_transfer`
- `confirm_pairing_with_bootstrap`
- `begin_pairing_confirm`
- `get_pairing_flow_state`
- `approve_pairing_overwrite`
- `cancel_pairing_flow`
- `confirm_pairing`

# PART 3 - MCP Agent Tools (crates/agent-tools)

Tools implement the `AgentTool` trait in `crates/agent-tools/src/tools/`.
Grouped by access level. Commit/import tools are MCP-only.

## 3.1 Read-Only Tools (16 tools)

| Tool | Scope | Description |
|---|---|---|
| `get_holdings` | HoldingsRead | Portfolio holdings with view_mode (table/treemap/both) |
| `get_accounts` | AccountsRead | List accounts with optional archived filter |
| `get_cash_balances` | HoldingsRead | Per-account per-currency cash positions |
| `search_activities` | ActivitiesRead | Paginated activity search with filters |
| `get_goals` | FinancialPlanningRead | Goals with progress |
| `get_valuation_history` | HoldingsRead | Valuation points over date range |
| `get_income` | PerformanceRead | Income summary (YTD/LAST_YEAR/ALL) |
| `get_asset_allocation` | HoldingsRead | Allocation by class/sector/region/risk/type |
| `get_performance` | PerformanceRead | Performance (1M/3M/6M/YTD/1Y/ALL) |
| `get_health_status` | HealthRead | Cached portfolio health status |
| `list_categorization_context` | ActivitiesRead | Taxonomies, examples, unproposed rows |
| `list_asset_taxonomies` | ClassificationRead | List taxonomies |
| `get_asset_taxonomy_assignments` | ClassificationRead | Asset-to-category assignments |
| `get_portfolios` | HoldingsRead | Named account groups |
| `get_net_worth` | HoldingsRead | Balance sheet + history |
| `get_contribution_limits` | FinancialPlanningRead | Limit, used, remaining amounts |

## 3.2 Draft / Suggest Tools (5 tools)

| Tool | Scope | Access | Description |
|---|---|---|---|
| `record_activity` | ActivitiesDraft | Draft | Single activity draft from natural language |
| `record_activities` | ActivitiesDraft | Draft | Batch activity drafts |
| `propose_categories` | ActivitiesRead + ActivitiesDraft | Suggest | Two-pass: deterministic rules + AI proposals |
| `create_categorization_rule` | ActivitiesRead | Suggest | Create draft categorization rule |
| `prepare_asset_classification` | ClassificationRead + ClassificationSuggest | Suggest | Validate and compute classification changes |

## 3.3 Commit Tools (MCP-only, 3 tools)

| Tool | Scopes | Access | Description |
|---|---|---|---|
| `commit_activity_draft` | ActivitiesDraft + ActivitiesWrite | Write | Commit single activity draft |
| `commit_activity_drafts` | ActivitiesDraft + ActivitiesWrite | Write | Commit up to 100 drafts in one call |
| `commit_asset_classification_draft` | ClassificationWrite + ClassificationSuggest | Write | Commit classification changes |

## 3.4 Import Tools (MCP-only, 3 tools)

| Tool | Scopes | Access | Description |
|---|---|---|---|
| `get_import_mapping` | ActivitiesRead | Read | Get account import mapping |
| `prepare_activity_import` | ActivitiesDraft | Draft | Preview import (max 1000 rows) |
| `commit_activity_import` | ActivitiesWrite | Write | Commit imported activities |

## 3.5 AgentScope and Access Levels

**AgentScope enum:**

| Scope | Description |
|---|---|
| AccountsRead | Read accounts |
| HoldingsRead | Read holdings, allocations, valuations |
| PerformanceRead | Read performance data |
| ActivitiesRead | Read activities |
| ActivitiesDraft | Create activity drafts |
| ActivitiesWrite | Commit/write activities |
| FinancialPlanningRead | Read goals, limits |
| HealthRead | Read health status |
| ClassificationRead | Read taxonomies, assignments |
| ClassificationSuggest | Propose classifications |
| ClassificationWrite | Commit classifications |

**AgentToolAccess enum:**

| Access Level | Description |
|---|---|
| Read | Read-only data retrieval |
| Draft | Create draft/preview entities (user confirms before commit) |
| Suggest | Propose changes (applied after user confirms via UI) |
| Write | Directly commit/write data to database |

# PART 4 - Shared Infrastructure

## 4.1 Portfolio Job Pipeline

File: `apps/server/src/api/shared.rs` (329 lines)

| Function | Description |
|---|---|
| `enqueue_portfolio_job(state, job)` | Enqueue a portfolio update job |
| `process_portfolio_job(state, job)` | Process a portfolio job synchronously |
| `portfolio_job_config()` | Configure job retry/timeout |
| `emit_sse_event(state, event)` | Emit SSE event to connected clients |

## 4.2 EventBus + SSE

File: `apps/server/src/events.rs`

Event types emitted via SSE at `GET /api/v1/events/stream`:
- Portfolio update progress
- Market data sync events
- Broker sync events

## 4.3 Secret Store

Directory: `apps/server/src/secrets/`

OS keyring-based secret store for secrets, addon secrets, and cloud sync tokens.

## 4.4 Agent Environment

File: `crates/agent-tools/src/env.rs`

Shared `AgentEnvironment` trait for dependency injection into all agent tools.

## 4.5 AccountScopeInput (IPC Boundary)

Struct for handling discriminated union serialization across Tauri IPC boundary.
Used by portfolio, performance, and other commands that accept scoped queries.

# PART 5 - Reusability, Migration Difficulty and Code Risk

| Module | Reusability | Migration Difficulty | Risk | Notes |
|---|---|---|---|---|
| HTTP API (auth) | Low | Medium | Low | Tightly coupled to Axum framework |
| HTTP API (business routes) | Medium | High | Medium | Axum-specific but delegates to crates/core |
| Tauri IPC (commands) | Low | Very High | Medium | Tightly coupled to Tauri v2 runtime |
| Tauri IPC (device_sync) | None | Very High | High | 70KB+ of E2EE + pairing protocol |
| MCP tools (agent-tools) | Medium | Medium | Low | Clean trait-based design, good abstraction |
| crates/core | High | Medium | Low | Pure business logic, framework-agnostic |
| crates/storage-sqlite | High | Medium | Low | Diesel ORM, can be swapped for another DB |
| crates/market-data | Medium | Medium | Medium | Provider-specific adapters required |
| crates/connect | Low | High | High | Cloud sync with complex state management |
| crates/device-sync | Low | Very High | High | E2EE crypto + multi-step pairing protocol |
| crates/ai | Medium | Medium | Low | LLM provider abstraction layer |
