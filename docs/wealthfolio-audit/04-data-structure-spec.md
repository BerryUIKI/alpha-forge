# 数据规格说明书

> 本文档由仓库扫描代理生成，覆盖 Wealthfolio 项目的**数据库模式层（Database Schema Layer）**。
> 权威来源：`F:\dev\wealthfolio\crates\storage-sqlite\src\schema.rs`（Diesel 生成的 schema，930 行）
> 与 `F:\dev\wealthfolio\crates\storage-sqlite\migrations\` 下全部 49 个迁移目录（每个含 up.sql / down.sql）。
> 生成日期：2026-08-12。

---

## Database Schema Layer

### 0. 总览

Wealthfolio 使用 **SQLite + Diesel ORM（Rust）** 作为唯一持久化存储。所有数据本地存储，无云端。

#### 0.1 关键设计决策

| 主题 | 决策 | 说明 |
| --- | --- | --- |
| 货币金额 | TEXT 存储 | BigDecimal 语义，避免浮点误差；格式为十进制字符串（如 `"1234.56"`），由 Rust 侧 `BigDecimal` 类型解析 |
| 时间戳 | TEXT（RFC3339 UTC） | 统一 `strftime('%Y-%m-%dT%H:%M:%fZ', 'now')` 格式；部分旧列为 `Timestamp`（Diesel 仅在类型层面区分） |
| 复杂结构 | JSON 文本列 | positions、config、metadata、summary 等以 JSON 字符串存储 |
| 主键 | TEXT UUID（v4） | `lower(hex(randomblob(4))) || '-' || ...` 生成；极少数表用整数自增（snapshot_positions、sync_cursor、sync_engine_state） |
| 复合主键 | 用于 join / 状态表 | taxonomy_categories(id, taxonomy_id)、sync_entity_metadata(entity, entity_id) 等 |
| 生成列 | `GENERATED ALWAYS AS ... STORED` | assets.instrument_key 由 instrument_type/symbol/ccy/mic 计算并落盘 |
| 外键 | 仅定义时声明 | 迁移连接默认 `PRAGMA foreign_keys = OFF`，应用连接开启 |
| 迁移策略 | 渐进式 + 重建 | 大重构（v2 asset model）用 `ALTER TABLE RENAME` + 重建表；派生读模型（lots/valuation/snapshot）定期清空重建 |

#### 0.2 迁移时间线（49 个迁移）

| 迁移目录 | 日期 | 主要变更 |
| --- | --- | --- |
| `2023-11-08-162221_init_db` | 2023-11 | 初始 schema：platforms、accounts、assets、activities、quotes、settings、goals、goals_allocation |
| `2024-09-16-023604_portfolio_history` | 2024-09 | portfolio_history、exchange_rates；goals 列类型 REAL→NUMERIC |
| `2024-09-21-023605_settings_to_kv` | 2024-09 | app_settings KV 表；删除 settings 表 |
| `2024-09-22-012202_init_exchange_rates` | 2024-09 | 种子汇率数据 |
| `2024-09-28-225756_add_calculated_at` | 2024-09 | portfolio_history.calculated_at；idx_activities_account_id |
| `2024-10-08-193300_contrib_limits` | 2024-10 | contribution_limits；instance_id 设置 |
| `2024-10-15-173026_csv_import_profiles` | 2024-10 | activity_import_profiles（后来被 import_templates 取代） |
| `2025-01-27-000001_migrate_fx_to_quotes` | 2025-01 | FOREX 资产；删除 exchange_rates；quotes 索引 |
| `2025-03-17-185736_add_start_end_dates_to_contribution_limits` | 2025-03 | contribution_limits.start_date/end_date |
| `2025-03-18-222805_add_amount_field_and_use_decimal` | 2025-03 | activities.amount；活动类型重构；BigDecimal TEXT 化；quotes 重建含 currency |
| `2025-04-21-195716_create_daily_account_history` | 2025-04 | holdings_snapshots、daily_account_valuation（取代 portfolio_history） |
| `2025-06-09-150456_add_net_contribution_base_to_snapshots` | 2025-06 | net_contribution_base |
| `2025-06-11-133126_account_import_mapping` | 2025-06 | account_mappings 列 |
| `2025-06-27-145729_create_market_data_providers_table` | 2025-06 | market_data_providers + 3 个种子 provider |
| `2026-01-01-000000_refactor_asset_model` | 2026-01 | **V2 大迁移**：assets 重建（UUID + instrument_key STORED）、legacy_asset_id_map、import_runs、brokers_sync_state、activities v2 |
| `2026-01-01-000001_quotes_market_data` | 2026-01 | quotes v2、quote_sync_state、FINNHUB provider |
| `2026-01-01-000002_taxonomies` | 2026-01 | taxonomies、taxonomy_categories、asset_taxonomy_assignments + 大规模种子数据（1302 行） |
| `2026-01-15-000001_ai_chat_persistence` | 2026-01 | ai_threads、ai_messages、ai_thread_tags |
| `2026-01-20-000001_health_issue_dismissals` | 2026-01 | health_issue_dismissals |
| `2026-01-24-000001_improve_import_profiles` | 2026-01 | activity_import_profiles v2 |
| `2026-01-26-000001_tracking_mode` | 2026-01 | tracking_mode、is_archived、holdings_snapshots.source |
| `2026-02-12-000001_device_sync_foundation` | 2026-02 | 设备同步基础：sync_cursor/outbox/entity_metadata/device_config/engine_state/table_state/applied_events |
| `2026-03-03-000001_add_phase2_providers` | 2026-03 | 新增 4 个行情 provider |
| `2026-03-09-000001_fix_provider_logos` | 2026-03 | provider logo 修复 |
| `2026-03-10-000001_sync_freshness_gate` | 2026-03 | sync_device_config.min_snapshot_created_at |
| `2026-03-18-000001_remove_income_fallback_quotes` | 2026-03 | 移除 income 兜底行情 |
| `2026-03-19-000001_import_templates` | 2026-03 | import_templates、import_account_templates；删除 activity_import_profiles |
| `2026-03-25-000001_custom_provider_sources` | 2026-03 | market_data_custom_providers、provider_type/config、CUSTOM_SCRAPER |
| `2026-03-30-000001_goals_and_retirement_planning` | 2026-03 | goals v2、goal_plans、goals_allocation v2 |
| `2026-04-29-000001_sync_entity_metadata_last_op` | 2026-04 | sync_entity_metadata.last_op |
| `2026-05-11-000001_portfolios` | 2026-05 | portfolios、portfolio_accounts |
| `2026-05-19-000001_lots_and_snapshot_positions` | 2026-05 | lots、snapshot_positions |
| `2026-05-22-000001_scoped_lots_valuation` | 2026-05 | daily_account_valuation v2（base 字段） |
| `2026-05-25-000001_spending_module` | 2026-05 | 支出模块（10 张表 + 种子数据） |
| `2026-05-25-000002_allocation_targets` | 2026-05 | allocation_targets、allocation_target_weights + 3 个触发器 |
| `2026-05-26-000001_lot_disposals` | 2026-05 | lot_disposals、lots base 字段、索引 |
| `2026-06-21-000001_valuation_quality` | 2026-06 | value_status/basis_status |
| `2026-06-22-000001_hybrid_drift_bands` | 2026-06 | allocation_targets.band_type/relative_factor_bps |
| `2026-06-25-000001_allocation_constraints` | 2026-06 | allocation_target_constraints、max_turnover_bps |
| `2026-06-26-000001_agent_access` | 2026-06 | personal_access_tokens、mcp_audit_log |
| `2026-06-28-000001_spending_activity_splits` | 2026-06 | spending_activity_splits |
| `2026-06-29-000001_activity_trade_tax` | 2026-06 | activities.tax、lots.tax_allocated |
| `2026-07-01-000001_activity_account_date_index` | 2026-07 | idx_activities_account_date |
| `2026-07-02-000001_snapshot_position_cost_basis` | 2026-07 | snapshot_positions.cost_basis_base/account |
| `2026-07-04-000001_reset_derived_read_models` | 2026-07 | lots.account_currency/fx_rate_to_account；清空派生读模型 |
| `2026-07-08-000001_addon_storage` | 2026-07 | addon_storage |
| `2026-08-02-000001_reclaim_storage` | 2026-08 | 删除冗余索引 + VACUUM 回收空间 |

#### 0.3 表分类

| 分类 | 表 | 说明 |
| --- | --- | --- |
| **核心领域（core domain）** | accounts、activities、assets、platforms、quotes、import_runs、import_templates、import_account_templates、lots、lot_disposals、holdings_snapshots、snapshot_positions、daily_account_valuation、goals、goal_plans、goals_allocation、contribution_limits | 投资组合记账核心 |
| **功能特定（feature-specific）** | taxonomies、taxonomy_categories、asset_taxonomy_assignments、activity_taxonomy_assignments、spending_*（8 张）、budget_*（4 张）、allocation_targets、allocation_target_weights、allocation_target_constraints、portfolios、portfolio_accounts、market_data_providers、market_data_custom_providers、quote_sync_state、brokers_sync_state、ai_threads、ai_messages、ai_thread_tags、health_issue_dismissals、addon_storage、personal_access_tokens | 各功能模块自有数据 |
| **审计/日志（audit/log）** | sync_*（7 张）、mcp_audit_log、app_settings | 同步状态、审计日志、配置 |

---

### 1. 核心领域表（Core Domain）

#### 1.1 accounts — 账户

| 迁移版本 | 2023-11-08-162221_init_db（初始创建） |
| --- | --- |
| 最后修改 | 2026-01-26-000001_tracking_mode（添加 tracking_mode、is_archived） |
| Grade | 复用性 A / 迁移难度 低 / 代码风险 低 |

**列定义**

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键，UUID |
| name | TEXT | NOT NULL | — | 账户名称 |
| account_type | TEXT | NOT NULL | — | 类型：SECURITIES / CASH / CREDIT_CARD / CRYPTOCURRENCY |
| group | TEXT | NULLABLE | — | 账户分组 |
| currency | TEXT | NOT NULL | — | ISO 货币代码（如 USD） |
| is_default | BOOL | NOT NULL | — | 是否默认账户 |
| is_active | BOOL | NOT NULL | — | 是否激活 |
| created_at | Timestamp | NOT NULL | — | 创建时间 |
| updated_at | Timestamp | NOT NULL | — | 更新时间 |
| platform_id | TEXT | NULLABLE | — | FK → platforms(id) |
| account_number | TEXT | NULLABLE | — | 账户号码 |
| meta | TEXT | NULLABLE | — | JSON 元数据 |
| provider | TEXT | NULLABLE | — | 数据提供商 |
| provider_account_id | TEXT | NULLABLE | — | 提供商侧账户 ID |
| is_archived | BOOL | NOT NULL | — | 是否归档 |
| tracking_mode | TEXT | NOT NULL | — | 跟踪模式：TRANSACTIONS / HOLDINGS |

**外键**：platform_id → platforms(id)

**索引**：无显式索引（PK 上有隐式索引）

---

#### 1.2 activities — 活动

| 迁移版本 | 2023-11-08-162221_init_db（初始创建） |
| --- | --- |
| 最后修改 | 2026-07-01-000001_activity_account_date_index（添加索引） |
| Grade | 复用性 A / 迁移难度 高 / 代码风险 高 |

**列定义**

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键，UUID |
| account_id | TEXT | NOT NULL | — | FK → accounts(id) |
| asset_id | TEXT | NULLABLE | — | FK → assets(id)，CASH 资产时为 NULL |
| activity_type | TEXT | NOT NULL | — | CHECK: BUY/SELL/SPLIT/DIVIDEND/INTEREST/DEPOSIT/WITHDRAWAL/TRANSFER_IN/TRANSFER_OUT/FEE/TAX/CREDIT/ADJUSTMENT/UNKNOWN |
| activity_type_override | TEXT | NULLABLE | — | 用户覆盖的活动类型 |
| source_type | TEXT | NULLABLE | — | 来源类型 |
| subtype | TEXT | NULLABLE | — | 子类型 |
| status | TEXT | NOT NULL | 'POSTED' | 状态：POSTED/PENDING/CANCELED |
| activity_date | TEXT | NOT NULL | — | 活动日期（YYYY-MM-DD） |
| settlement_date | TEXT | NULLABLE | — | 结算日期 |
| quantity | TEXT | NULLABLE | — | 数量（BigDecimal 字符串） |
| unit_price | TEXT | NULLABLE | — | 单价 |
| amount | TEXT | NULLABLE | — | 金额 |
| fee | TEXT | NULLABLE | — | 费用 |
| tax | TEXT | NULLABLE | — | 税费（2026-06-29 添加） |
| currency | TEXT | NOT NULL | — | 货币 |
| fx_rate | TEXT | NULLABLE | — | 汇率 |
| notes | TEXT | NULLABLE | — | 备注 |
| metadata | TEXT | NULLABLE | — | JSON 元数据 |
| source_system | TEXT | NULLABLE | — | 来源系统（如 CSV、BROKER） |
| source_record_id | TEXT | NULLABLE | — | 来源系统记录 ID |
| source_group_id | TEXT | NULLABLE | — | 来源分组 ID |
| idempotency_key | TEXT | NULLABLE | — | 幂等键 |
| import_run_id | TEXT | NULLABLE | — | FK → import_runs(id) |
| is_user_modified | Integer | NOT NULL | 0 | 用户是否手动修改 |
| needs_review | Integer | NOT NULL | 0 | 是否需要审核 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

**外键**：account_id → accounts(id)、asset_id → assets(id)、import_run_id → import_runs(id)

**索引**：
- `idx_activities_account_date` — (account_id, activity_date) — 2026-07-01
- `ix_activities_source_group_id` — (source_group_id) WHERE source_group_id IS NOT NULL
- `ix_activities_transfer_scope` — (account_id, activity_date, status) WHERE type IN (TRANSFER_IN, TRANSFER_OUT)

---

#### 1.3 assets — 资产

| 迁移版本 | 2023-11-08-162221_init_db（初始创建） |
| --- | --- |
| 最后修改 | 2026-01-01-000000_refactor_asset_model（V2 重建：UUID、instrument_key STORED） |
| Grade | 复用性 A / 迁移难度 高 / 代码风险 高 |

**列定义**

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | UUID v4 | 主键 |
| kind | TEXT | NOT NULL | — | 类型：INVESTMENT / PROPERTY / VEHICLE / COLLECTIBLE / PRECIOUS_METAL / PRIVATE_EQUITY / LIABILITY / OTHER / FX |
| name | TEXT | NULLABLE | — | 资产名称 |
| display_code | TEXT | NULLABLE | — | 显示代码 |
| notes | TEXT | NULLABLE | — | 备注 |
| metadata | TEXT | NULLABLE | — | JSON 元数据（含 legacy.old_id 等） |
| is_active | Integer | NOT NULL | 1 | 是否激活 |
| quote_mode | TEXT | NOT NULL | — | MARKET / MANUAL |
| quote_ccy | TEXT | NOT NULL | — | 报价货币 |
| instrument_type | TEXT | NULLABLE | — | EQUITY / CRYPTO / FX / OPTION / METAL |
| instrument_symbol | TEXT | NULLABLE | — | 标准交易符号（AAPL、BTC） |
| instrument_exchange_mic | TEXT | NULLABLE | — | ISO 10383 MIC（XNAS、XTSE） |
| instrument_key | TEXT | NULLABLE | — | GENERATED ALWAYS AS ... STORED，格式：`TYPE:SYMBOL@MIC` 或 `TYPE:SYMBOL/CCY` |
| provider_config | TEXT | NULLABLE | — | JSON 提供商配置 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

**主键**：id

**唯一索引**：`idx_assets_instrument_key` — (instrument_key) WHERE instrument_key IS NOT NULL

**索引**：idx_assets_kind、idx_assets_is_active、idx_assets_display_code

**CHECK 约束**：kind IN（...8种）、quote_mode IN（MARKET/MANUAL）、is_active IN (0,1)、metadata IS NULL OR json_valid(metadata)、provider_config IS NULL OR json_valid(provider_config)

---

#### 1.4 platforms — 经纪商/平台

| 迁移版本 | 2023-11-08-162221_init_db |
| --- | --- |
| Grade | 复用性 B / 迁移难度 低 / 代码风险 低 |

**列定义**

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| name | TEXT | NULLABLE | — | 平台名称 |
| url | TEXT | NOT NULL | — | 平台 URL |
| external_id | TEXT | NULLABLE | — | 外部 ID |
| kind | TEXT | NOT NULL | — | 平台类型 |
| website_url | TEXT | NULLABLE | — | 网站 URL |
| logo_url | TEXT | NULLABLE | — | Logo URL |

**关系**：accounts → platforms（一对多）

---

#### 1.5 quotes — 行情

| 迁移版本 | 2023-11-08-162221_init_db（初始创建） |
| --- | --- |
| 最后修改 | 2026-01-01-000001_quotes_market_data（V2 重建：asset_id, source, notes） |
| Grade | 复用性 A / 迁移难度 中 / 代码风险 中 |

**列定义**

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| asset_id | TEXT | NOT NULL | — | FK → assets(id) |
| day | TEXT | NOT NULL | — | 日期（YYYY-MM-DD） |
| source | TEXT | NOT NULL | — | 数据来源 |
| open | TEXT | NULLABLE | — | 开盘价 |
| high | TEXT | NULLABLE | — | 最高价 |
| low | TEXT | NULLABLE | — | 最低价 |
| close | TEXT | NOT NULL | — | 收盘价 |
| adjclose | TEXT | NULLABLE | — | 复权收盘价 |
| volume | TEXT | NULLABLE | — | 成交量 |
| currency | TEXT | NOT NULL | — | 货币 |
| notes | TEXT | NULLABLE | — | 备注 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| timestamp | TEXT | NOT NULL | — | 时间戳 |

**唯一索引**：`uq_quotes_asset_day_source` — (asset_id, day, source)（唯一约束）

**索引**：~~`idx_quotes_asset_day`~~ — (asset_id, day) — 2026-08-02 作为冗余索引删除

**注意**：冗余索引 `idx_quotes_asset_day` 在 2026-08-02-000001_reclaim_storage 中被删除，理由是该索引是 `uq_quotes_asset_day_source` 的严格左前缀，查询计划器可以完全由唯一索引提供服务。

---

#### 1.6 import_runs — 导入运行

| 迁移版本 | 2026-01-01-000000_refactor_asset_model |
| --- | --- |
| Grade | 复用性 B / 迁移难度 中 / 代码风险 中 |

**列定义**

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| account_id | TEXT | NOT NULL | — | FK → accounts(id) |
| source_system | TEXT | NOT NULL | — | 来源系统（CSV、BROKER 等） |
| run_type | TEXT | NOT NULL | — | 运行类型 |
| mode | TEXT | NOT NULL | — | 模式 |
| status | TEXT | NOT NULL | — | 状态 |
| started_at | TEXT | NOT NULL | — | 开始时间 |
| finished_at | TEXT | NULLABLE | — | 完成时间 |
| review_mode | TEXT | NOT NULL | — | 审核模式 |
| applied_at | TEXT | NULLABLE | — | 应用时间 |
| checkpoint_in | TEXT | NULLABLE | — | 输入检查点 |
| checkpoint_out | TEXT | NULLABLE | — | 输出检查点 |
| summary | TEXT | NULLABLE | — | JSON 摘要 |
| warnings | TEXT | NULLABLE | — | 警告 |
| error | TEXT | NULLABLE | — | 错误信息 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

**索引**：ix_import_runs_account_id、ix_import_runs_status

---

#### 1.7 import_templates — 导入模板

| 迁移版本 | 2026-03-19-000001_import_templates（取代 activity_import_profiles） |
| --- | --- |
| Grade | 复用性 B / 迁移难度 中 / 代码风险 中 |

**列定义**

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| name | TEXT | NOT NULL | — | 模板名称 |
| scope | TEXT | NOT NULL | — | 作用域 |
| kind | TEXT | NOT NULL | — | 模板类型 |
| source_system | TEXT | NOT NULL | — | 来源系统 |
| config_version | Integer | NOT NULL | — | 配置版本 |
| config | TEXT | NOT NULL | — | JSON 配置 |
| created_at | Timestamp | NOT NULL | — | 创建时间 |
| updated_at | Timestamp | NOT NULL | — | 更新时间 |

---

#### 1.8 import_account_templates — 账户-模板关联

| 迁移版本 | 2026-03-19-000001_import_templates |
| --- | --- |
| Grade | 复用性 C / 迁移难度 低 / 代码风险 低 |

**列定义**

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| account_id | TEXT | NOT NULL | — | FK → accounts(id) |
| context_kind | TEXT | NOT NULL | — | 上下文类型 |
| source_system | TEXT | NOT NULL | — | 来源系统 |
| template_id | TEXT | NOT NULL | — | FK → import_templates(id) |
| created_at | Timestamp | NOT NULL | — | 创建时间 |
| updated_at | Timestamp | NOT NULL | — | 更新时间 |

**外键**：template_id → import_templates(id)

---

#### 1.9 lots — 税务批次

| 迁移版本 | 2026-05-19-000001_lots_and_snapshot_positions |
| --- | --- |
| 最后修改 | 2026-07-04-000001_reset_derived_read_models（添加 account_currency/fx_rate_to_account） |
| Grade | 复用性 A / 迁移难度 高 / 代码风险 高 |

**列定义**

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| account_id | TEXT | NOT NULL | — | FK → accounts(id) |
| asset_id | TEXT | NOT NULL | — | FK → assets(id) |
| open_date | TEXT | NOT NULL | — | 开仓日期 |
| open_activity_id | TEXT | NULLABLE | — | FK → activities(id)，开仓活动 |
| original_quantity | TEXT | NOT NULL | — | 原始数量 |
| cost_per_unit | TEXT | NOT NULL | — | 每单位成本 |
| original_cost_basis | TEXT | NOT NULL | — | 原始成本基础 |
| remaining_cost_basis | TEXT | NOT NULL | — | 剩余成本基础 |
| original_cost_basis_base | TEXT | NOT NULL | '0' | 原始成本基础（本位币） |
| remaining_cost_basis_base | TEXT | NOT NULL | '0' | 剩余成本基础（本位币） |
| fee_allocated | TEXT | NOT NULL | '0' | 分配费用 |
| fee_allocated_base | TEXT | NOT NULL | '0' | 分配费用（本位币） |
| tax_allocated | TEXT | NOT NULL | '0' | 分配税费（2026-06-29） |
| tax_allocated_base | TEXT | NOT NULL | '0' | 分配税费（本位币，2026-06-29） |
| currency | TEXT | NOT NULL | '' | 货币 |
| base_currency | TEXT | NOT NULL | '' | 本位币 |
| fx_rate_to_base | TEXT | NOT NULL | '1' | 汇率→本位币 |
| fx_rate_to_account | TEXT | NULLABLE | — | 汇率→账户货币（2026-07-04） |
| account_currency | TEXT | NULLABLE | — | 账户货币（2026-07-04） |
| cost_basis_method | TEXT | NOT NULL | 'FIFO' | 成本基础方法（FIFO） |
| remaining_quantity | TEXT | NOT NULL | — | 剩余数量 |
| split_ratio | TEXT | NOT NULL | '1' | 拆分比率 |
| is_closed | Integer | NOT NULL | 0 | 是否已关闭 |
| close_date | TEXT | NULLABLE | — | 关闭日期 |
| close_activity_id | TEXT | NULLABLE | — | FK → activities(id)，关闭活动 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

**外键**：account_id → accounts、asset_id → assets、open_activity_id → activities、close_activity_id → activities

**索引**：idx_lots_account_asset、idx_lots_asset_open、idx_lots_account_open、idx_lots_open_activity（部分索引）

---

#### 1.10 lot_disposals — 处置记录

| 迁移版本 | 2026-05-26-000001_lot_disposals |
| --- | --- |
| Grade | 复用性 B / 迁移难度 中 / 代码风险 中 |

**列定义**

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| lot_id | TEXT | NOT NULL | — | FK → lots(id) |
| account_id | TEXT | NOT NULL | — | FK → accounts(id) |
| asset_id | TEXT | NOT NULL | — | FK → assets(id) |
| disposal_activity_id | TEXT | NOT NULL | — | FK → activities(id) |
| disposal_date | TEXT | NOT NULL | — | 处置日期 |
| quantity | TEXT | NOT NULL | — | 处置数量 |
| proceeds | TEXT | NOT NULL | — | 收入 |
| cost_basis | TEXT | NOT NULL | — | 成本基础 |
| realized_pnl | TEXT | NOT NULL | — | 已实现盈亏 |
| proceeds_base | TEXT | NOT NULL | — | 收入（本位币） |
| cost_basis_base | TEXT | NOT NULL | — | 成本基础（本位币） |
| realized_pnl_base | TEXT | NOT NULL | — | 已实现盈亏（本位币） |
| currency | TEXT | NOT NULL | — | 货币 |
| base_currency | TEXT | NOT NULL | — | 本位币 |
| fx_rate_to_base | TEXT | NOT NULL | — | 汇率 |
| cost_basis_method | TEXT | NOT NULL | 'FIFO' | 成本基础方法 |
| created_at | TEXT | NOT NULL | — | 创建时间 |

**索引**：idx_lot_disposals_account_date、idx_lot_disposals_asset_date、idx_lot_disposals_activity

---

#### 1.11 holdings_snapshots — 持仓快照

| 迁移版本 | 2025-04-21-195716_create_daily_account_history |
| --- | --- |
| 最后修改 | 2026-01-26-000001_tracking_mode（添加 source 列） |
| Grade | 复用性 A / 迁移难度 中 / 代码风险 中 |

**列定义**

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| account_id | TEXT | NOT NULL | — | FK → accounts(id) |
| snapshot_date | Date | NOT NULL | — | 快照日期 |
| currency | TEXT | NOT NULL | — | 货币 |
| positions | TEXT | NOT NULL | — | JSON 持仓数据 |
| cash_balances | TEXT | NOT NULL | — | JSON 现金余额 |
| cost_basis | TEXT | NOT NULL | — | 成本基础 |
| net_contribution | TEXT | NOT NULL | — | 净投入 |
| calculated_at | TEXT | NOT NULL | — | 计算时间 |
| net_contribution_base | TEXT | NOT NULL | — | 净投入（本位币） |
| cash_total_account_currency | TEXT | NOT NULL | — | 现金总计（账户货币） |
| cash_total_base_currency | TEXT | NOT NULL | — | 现金总计（本位币） |
| source | TEXT | NOT NULL | — | 来源：CALCULATED / MANUAL_ENTRY / CSV_IMPORT / BROKER_IMPORTED / SYNTHETIC |

---

#### 1.12 snapshot_positions — 快照持仓（关系型）

| 迁移版本 | 2026-05-19-000001_lots_and_snapshot_positions |
| --- | --- |
| 最后修改 | 2026-07-02-000001_snapshot_position_cost_basis（添加 cost_basis_base/account） |
| Grade | 复用性 B / 迁移难度 中 / 代码风险 中 |

**列定义**

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | Integer | NOT NULL | AUTOINCREMENT | 主键（整数自增） |
| snapshot_id | TEXT | NOT NULL | — | FK → holdings_snapshots(id) |
| asset_id | TEXT | NOT NULL | — | FK → assets(id) |
| quantity | TEXT | NOT NULL | — | 持仓数量 |
| average_cost | TEXT | NOT NULL | — | 平均成本 |
| total_cost_basis | TEXT | NOT NULL | — | 总成本基础 |
| currency | TEXT | NOT NULL | — | 货币 |
| contract_multiplier | TEXT | NOT NULL | '1' | 合约乘数 |
| inception_date | TEXT | NOT NULL | — | 起始日期 |
| is_alternative | Integer | NOT NULL | 0 | 是否另类资产 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| last_updated | TEXT | NOT NULL | — | 更新时间 |
| cost_basis_base | TEXT | NULLABLE | — | 预计算成本基础（本位币，2026-07-02） |
| cost_basis_account | TEXT | NULLABLE | — | 预计算成本基础（账户货币，2026-07-02） |

**唯一约束**：(snapshot_id, asset_id)

**索引**：idx_snapshot_positions_snapshot_id、idx_snapshot_positions_asset_id

---

#### 1.13 daily_account_valuation — 日估值

| 迁移版本 | 2025-04-21-195716_create_daily_account_history（V1） |
| --- | --- |
| 最后修改 | 2026-05-22-000001_scoped_lots_valuation（V2 重建含 base 字段） |
| Grade | 复用性 A / 迁移难度 中 / 代码风险 中 |

**列定义**

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| account_id | TEXT | NOT NULL | — | FK → accounts(id) |
| valuation_date | Date | NOT NULL | — | 估值日期 |
| account_currency | TEXT | NOT NULL | — | 账户货币 |
| base_currency | TEXT | NOT NULL | — | 本位币 |
| fx_rate_to_base | TEXT | NOT NULL | — | 汇率→本位币 |
| cash_balance | TEXT | NOT NULL | — | 现金余额 |
| investment_market_value | TEXT | NOT NULL | — | 投资市值 |
| total_value | TEXT | NOT NULL | — | 总价值 |
| cost_basis | TEXT | NOT NULL | — | 成本基础 |
| net_contribution | TEXT | NOT NULL | — | 净投入 |
| cash_balance_base | TEXT | NOT NULL | '0' | 现金余额（本位币） |
| investment_market_value_base | TEXT | NOT NULL | '0' | 投资市值（本位币） |
| total_value_base | TEXT | NOT NULL | '0' | 总价值（本位币） |
| cost_basis_base | TEXT | NOT NULL | '0' | 成本基础（本位币） |
| net_contribution_base | TEXT | NOT NULL | '0' | 净投入（本位币） |
| external_inflow_base | TEXT | NOT NULL | '0' | 外部流入（本位币） |
| external_outflow_base | TEXT | NOT NULL | '0' | 外部流出（本位币） |
| external_flow_source | TEXT | NOT NULL | 'UNKNOWN' | 外部流水来源 |
| performance_eligible_value_base | TEXT | NOT NULL | '0' | 可计算业绩的价值（本位币） |
| value_status | TEXT | NOT NULL | — | 估值质量状态（2026-06-21） |
| basis_status | TEXT | NOT NULL | — | 成本基础状态（2026-06-21） |
| calculated_at | TEXT | NOT NULL | — | 计算时间 |

**索引**：idx_daily_account_valuation_account_date — (account_id, valuation_date)

---

#### 1.14 goals — 目标

| 迁移版本 | 2023-11-08-162221_init_db（初始创建） |
| --- | --- |
| 最后修改 | 2026-03-30-000001_goals_and_retirement_planning（V2：goal_type、status_lifecycle 等） |
| Grade | 复用性 A / 迁移难度 中 / 代码风险 中 |

**列定义**

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| title | TEXT | NOT NULL | — | 目标标题 |
| description | TEXT | NULLABLE | — | 描述 |
| target_amount | Double | NOT NULL | — | 目标金额 |
| goal_type | TEXT | NOT NULL | — | 目标类型 |
| status_lifecycle | TEXT | NOT NULL | — | 生命周期状态 |
| status_health | TEXT | NOT NULL | — | 健康状态 |
| priority | Integer | NOT NULL | — | 优先级 |
| cover_image_key | TEXT | NULLABLE | — | 封面图片键 |
| currency | TEXT | NULLABLE | — | 货币 |
| start_date | TEXT | NULLABLE | — | 开始日期 |
| target_date | TEXT | NULLABLE | — | 目标日期 |
| summary_current_value | Double | NULLABLE | — | 当前值摘要 |
| summary_progress | Double | NULLABLE | — | 进度摘要 |
| projected_completion_date | TEXT | NULLABLE | — | 预计完成日期 |
| projected_value_at_target_date | Double | NULLABLE | — | 目标日预计值 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |
| summary_target_amount | Double | NULLABLE | — | 目标金额摘要 |

---

#### 1.15 goal_plans — 目标计划

| 迁移版本 | 2026-03-30-000001_goals_and_retirement_planning |
| --- | --- |
| Grade | 复用性 B / 迁移难度 中 / 代码风险 中 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| goal_id | TEXT | NOT NULL | — | 主键，FK → goals(id) |
| plan_kind | TEXT | NOT NULL | — | 计划类型 |
| planner_mode | TEXT | NULLABLE | — | 规划器模式 |
| settings_json | TEXT | NOT NULL | — | JSON 设置 |
| summary_json | TEXT | NOT NULL | — | JSON 摘要 |
| version | Integer | NOT NULL | — | 版本 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

---

#### 1.16 goals_allocation — 目标分配

| 迁移版本 | 2023-11-08-162221_init_db（初始创建） |
| --- | --- |
| 最后修改 | 2026-03-30-000001_goals_and_retirement_planning（V2：share_percent、tax_bucket） |
| Grade | 复用性 B / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| goal_id | TEXT | NOT NULL | — | FK → goals(id) |
| account_id | TEXT | NOT NULL | — | FK → accounts(id) |
| share_percent | Double | NOT NULL | — | 分配百分比 |
| tax_bucket | TEXT | NULLABLE | — | 税务桶 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

---

#### 1.17 contribution_limits — 缴款限额

| 迁移版本 | 2024-10-08-193300_contrib_limits |
| --- | --- |
| 最后修改 | 2025-03-17-185736_add_start_end_dates_to_contribution_limits |
| Grade | 复用性 B / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| group_name | TEXT | NOT NULL | — | 分组名称 |
| contribution_year | Integer | NOT NULL | — | 缴款年度 |
| limit_amount | Double | NOT NULL | — | 限额 |
| account_ids | TEXT | NULLABLE | — | 关联账户 ID 列表 |
| created_at | Timestamp | NOT NULL | — | 创建时间 |
| updated_at | Timestamp | NOT NULL | — | 更新时间 |
| start_date | Timestamp | NULLABLE | — | 开始日期 |
| end_date | Timestamp | NULLABLE | — | 结束日期 |

---

### 2. 功能特定表（Feature-Specific）

#### 2.1 taxonomies — 分类系统

| 迁移版本 | 2026-01-01-000002_taxonomies |
| --- | --- |
| 最后修改 | 2026-05-25-000001_spending_module（添加 scope 列） |
| Grade | 复用性 A / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键，系统预置如 `asset_classes`、`regions`、`spending_categories` |
| name | TEXT | NOT NULL | — | 名称 |
| color | TEXT | NOT NULL | — | 颜色 |
| description | TEXT | NULLABLE | — | 描述 |
| is_system | Integer | NOT NULL | — | 系统内置标记 |
| is_single_select | Integer | NOT NULL | — | 单选/多选 |
| sort_order | Integer | NOT NULL | — | 排序 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |
| scope | TEXT | NOT NULL | 'asset' | 作用域：'asset' / 'activity'（spending 模块用） |

**索引**：ix_taxonomies_scope — (scope)

---

#### 2.2 taxonomy_categories — 分类目录

| 迁移版本 | 2026-01-01-000002_taxonomies |
| --- | --- |
| 最后修改 | 2026-05-25-000001_spending_module（添加 icon 列） |
| Grade | 复用性 A / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 分类 ID（复合主键一部分） |
| taxonomy_id | TEXT | NOT NULL | — | FK → taxonomies(id)，复合主键一部分 |
| parent_id | TEXT | NULLABLE | — | 父级分类（树形结构） |
| name | TEXT | NOT NULL | — | 名称 |
| key | TEXT | NOT NULL | — | 语义键（如 country_US） |
| color | TEXT | NOT NULL | — | 颜色 |
| description | TEXT | NULLABLE | — | 描述 |
| sort_order | Integer | NOT NULL | — | 排序 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |
| icon | TEXT | NULLABLE | — | 图标名（spending 模块添加） |

**主键**：(id, taxonomy_id) — 复合主键

**关系**：taxonomy_categories → taxonomies（多对一）

**注意**：种子数据规模大：regions（洲/国家，约 250 行）、asset_classes（资产类别）、instrument_type（工具类型）、spending_categories/income_sources/savings_categories（支出分类树）。

---

#### 2.3 asset_taxonomy_assignments — 资产分类分配

| 迁移版本 | 2026-01-01-000002_taxonomies |
| --- | --- |
| Grade | 复用性 A / 迁移难度 中 / 代码风险 中 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| asset_id | TEXT | NOT NULL | — | FK → assets(id) |
| taxonomy_id | TEXT | NOT NULL | — | 分类系统 ID |
| category_id | TEXT | NOT NULL | — | 分类 ID |
| weight | Integer | NOT NULL | — | 权重（万分比，0-10000） |
| source | TEXT | NOT NULL | — | 'manual' / 'migrated' 等来源 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

**外键**：asset_id → assets(id)

---

#### 2.4 activity_taxonomy_assignments — 活动分类分配

| 迁移版本 | 2026-05-25-000001_spending_module |
| --- | --- |
| Grade | 复用性 B / 迁移难度 中 / 代码风险 中 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| activity_id | TEXT | NOT NULL | — | FK → activities(id) |
| taxonomy_id | TEXT | NOT NULL | — | 分类系统 ID |
| category_id | TEXT | NOT NULL | — | 分类 ID |
| weight | Integer | NOT NULL | 10000 | 权重（0-10000 CHECK） |
| source | TEXT | NOT NULL | 'manual' | 来源 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

**唯一索引**：ix_activity_taxonomy_assignment_unique — (activity_id, taxonomy_id)
**索引**：ix_activity_taxonomy_assignments_activity、ix_activity_taxonomy_assignments_category

---

#### 2.5 spending_activity_events — 活动-事件关联

| 迁移版本 | 2026-05-25-000001_spending_module |
| --- | --- |
| Grade | 复用性 B / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| activity_id | TEXT | NOT NULL | — | 主键，FK → activities(id) |
| event_id | TEXT | NOT NULL | — | FK → spending_events(id) |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

**关系**：一个活动最多标记一个事件（1:1 by construction，PK 为 activity_id）

---

#### 2.6 spending_event_types — 事件类型（查找表）

| 迁移版本 | 2026-05-25-000001_spending_module |
| --- | --- |
| Grade | 复用性 B / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| key | TEXT | NULLABLE | — | 语义键：travel/holiday/business 等 |
| name | TEXT | NOT NULL | — | 名称 |
| color | TEXT | NULLABLE | — | 颜色 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

**唯一索引**：idx_spending_event_types_key_unique WHERE key IS NOT NULL

---

#### 2.7 spending_events — 事件

| 迁移版本 | 2026-05-25-000001_spending_module |
| --- | --- |
| Grade | 复用性 B / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| name | TEXT | NOT NULL | — | 名称 |
| description | TEXT | NULLABLE | — | 描述 |
| event_type_id | TEXT | NOT NULL | — | FK → spending_event_types(id) |
| start_date | TEXT | NOT NULL | — | 开始日期 |
| end_date | TEXT | NOT NULL | — | 结束日期 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

**索引**：idx_spending_events_event_type、idx_spending_events_dates

---

#### 2.8 spending_activity_splits — 活动拆分

| 迁移版本 | 2026-06-28-000001_spending_activity_splits |
| --- | --- |
| Grade | 复用性 B / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| activity_id | TEXT | NOT NULL | — | FK → activities(id) |
| taxonomy_id | TEXT | NOT NULL | — | FK → taxonomies(id) |
| category_id | TEXT | NOT NULL | — | 与 taxonomy_id 组成复合 FK → taxonomy_categories |
| amount | TEXT | NOT NULL | — | 金额（CHECK CAST(amount AS REAL) > 0） |
| note | TEXT | NULLABLE | — | 备注 |
| sort_order | Integer | NOT NULL | 0 | 排序 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

**索引**：idx_spending_activity_splits_activity、idx_spending_activity_splits_category

---

#### 2.9 spending_categorization_rules — 自动分类规则

| 迁移版本 | 2026-05-25-000001_spending_module |
| --- | --- |
| Grade | 复用性 B / 迁移难度 中 / 代码风险 中 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| name | TEXT | NOT NULL | — | 规则名称 |
| pattern | TEXT | NOT NULL | — | 匹配模式 |
| match_type | TEXT | NOT NULL | 'contains' | contains/starts_with/exact/regex |
| taxonomy_id | TEXT | NULLABLE | — | FK → taxonomies(id) |
| category_id | TEXT | NULLABLE | — | 与 taxonomy_id 组成复合 FK |
| activity_type | TEXT | NULLABLE | — | 活动类型限制 |
| priority | Integer | NOT NULL | 0 | 优先级 |
| is_global | Integer | NOT NULL | 1 | 是否全局（CHECK: is_global=1 ↔ account_id IS NULL） |
| account_id | TEXT | NULLABLE | — | FK → accounts(id) |
| preset_id | TEXT | NULLABLE | — | 预置规则来源 |
| preset_rule_key | TEXT | NULLABLE | — | 预置规则键 |
| preset_version | TEXT | NULLABLE | — | 预置版本 |
| preset_modified | Integer | NOT NULL | 0 | 用户是否修改过预置规则 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

**唯一索引**：idx_spending_categorization_rules_preset_unique WHERE preset_id IS NOT NULL
**索引**：priority、category、account、is_global、activity_type

---

#### 2.10 spending_preset_rule_deletions — 预置规则删除记录

| 迁移版本 | 2026-05-25-000001_spending_module |
| --- | --- |
| Grade | 复用性 C / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| preset_id | TEXT | NOT NULL | — | 复合主键 |
| preset_rule_key | TEXT | NOT NULL | — | 复合主键 |
| rule_id | TEXT | NOT NULL | — | UNIQUE 约束 |
| deleted_at | TEXT | NOT NULL | — | 删除时间 |

---

#### 2.11 budget_groups — 预算分组

| 迁移版本 | 2026-05-25-000001_spending_module |
| --- | --- |
| Grade | 复用性 B / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| name | TEXT | NOT NULL | — | 名称 |
| key | TEXT | NOT NULL | UNIQUE | 语义键：needs/wants/savings/giving/personal/other |
| color | TEXT | NULLABLE | — | 颜色 |
| icon | TEXT | NULLABLE | — | 图标 |
| sort_order | Integer | NOT NULL | 0 | 排序 |
| is_system | Integer | NOT NULL | 0 | 系统标记 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

**索引**：idx_budget_groups_sort

---

#### 2.12 budget_group_assignments — 分组-分类关联

| 迁移版本 | 2026-05-25-000001_spending_module |
| --- | --- |
| Grade | 复用性 C / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| group_id | TEXT | NOT NULL | — | FK → budget_groups(id) |
| taxonomy_id | TEXT | NOT NULL | 'spending_categories' | CHECK IN (spending_categories, savings_categories) |
| category_id | TEXT | NOT NULL | — | 与 taxonomy_id 复合 FK |
| is_system | Integer | NOT NULL | 0 | 系统标记 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

**唯一约束**：(taxonomy_id, category_id)

---

#### 2.13 budget_targets — 预算目标

| 迁移版本 | 2026-05-25-000001_spending_module |
| --- | --- |
| Grade | 复用性 B / 迁移难度 中 / 代码风险 中 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| period_key | TEXT | NOT NULL | — | 'default' 或 YYYY-MM（GLOB CHECK） |
| target_type | TEXT | NOT NULL | — | category / group_buffer |
| taxonomy_id | TEXT | NULLABLE | — | 复合 FK |
| category_id | TEXT | NULLABLE | — | 复合 FK |
| group_id | TEXT | NULLABLE | — | FK → budget_groups(id) |
| amount | TEXT | NOT NULL | '0' | 目标金额 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

**CHECK 约束**：target_type 与字段一致性（category 类型必须带 taxonomy/category，group_buffer 必须带 group）
**唯一索引**：idx_budget_targets_category_unique、idx_budget_targets_group_buffer_unique（两个均为部分唯一索引）

---

#### 2.14 budget_rollover_settings — 预算结转设置

| 迁移版本 | 2026-05-25-000001_spending_module |
| --- | --- |
| Grade | 复用性 C / 迁移难度 中 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| target_type | TEXT | NOT NULL | — | category / group |
| taxonomy_id | TEXT | NULLABLE | — | — |
| category_id | TEXT | NULLABLE | — | — |
| group_id | TEXT | NULLABLE | — | FK → budget_groups(id) |
| enabled | Integer | NOT NULL | 1 | 是否启用 |
| start_month | TEXT | NOT NULL | — | 起始月份（YYYY-MM） |
| starting_balance | TEXT | NOT NULL | '0' | 起始余额 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

**唯一索引**：idx_budget_rollover_settings_category_unique、idx_budget_rollover_settings_group_unique

---

#### 2.15 allocation_targets — 配置目标

| 迁移版本 | 2026-05-25-000002_allocation_targets |
| --- | --- |
| 最后修改 | 2026-06-25-000001_allocation_constraints（添加 max_turnover_bps） |
| Grade | 复用性 B / 迁移难度 中 / 代码风险 中 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| name | TEXT | NOT NULL | — | 名称（CHECK length > 0） |
| scope_type | TEXT | NOT NULL | — | all / portfolio / account（CHECK） |
| scope_id | TEXT | NULLABLE | — | 作用域 ID（CHECK：all ↔ NULL，其余 ↔ NOT NULL） |
| taxonomy_id | TEXT | NOT NULL | 'asset_classes' | 分类系统 |
| trigger_type | TEXT | NOT NULL | 'threshold' | manual / threshold |
| drift_band_bps | Integer | NOT NULL | 500 | 漂移带宽（0-10000） |
| band_type | TEXT | NOT NULL | 'absolute' | absolute / hybrid（2026-06-22） |
| relative_factor_bps | Integer | NOT NULL | 2000 | 相对因子（0-10000，2026-06-22） |
| rebalance_goal | TEXT | NOT NULL | 'nearest_band' | nearest_band / exact_target |
| min_trade_amount | TEXT | NOT NULL | '0' | 最小交易金额 |
| whole_shares_only | Integer | NOT NULL | 0 | 仅整股 |
| allow_sells | Integer | NOT NULL | 0 | 允许卖出 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |
| archived_at | TEXT | NULLABLE | — | 归档时间 |
| max_turnover_bps | Integer | NULLABLE | — | 最大换手率（0-10000）（2026-06-25） |

**索引**：idx_allocation_targets_scope — (scope_type, scope_id, archived_at)

**触发器**：allocation_targets_taxonomy_update（见第 5 节）

---

#### 2.16 allocation_target_weights — 配置权重

| 迁移版本 | 2026-05-25-000002_allocation_targets |
| --- | --- |
| Grade | 复用性 B / 迁移难度 中 / 代码风险 中 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| target_id | TEXT | NOT NULL | — | FK → allocation_targets(id) |
| taxonomy_id | TEXT | NOT NULL | — | FK → taxonomy_categories 复合 |
| category_id | TEXT | NOT NULL | — | 复合 FK |
| target_bps | Integer | NOT NULL | — | 目标权重（0-10000） |
| is_locked | Integer | NOT NULL | 0 | 锁定 |
| is_required | Integer | NOT NULL | 1 | 必需 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

**唯一约束**：(target_id, taxonomy_id, category_id)
**触发器**：allocation_target_weights_taxonomy_insert / _update（见第 5 节）

---

#### 2.17 allocation_target_constraints — 配置约束

| 迁移版本 | 2026-06-25-000001_allocation_constraints |
| --- | --- |
| Grade | 复用性 C / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| target_id | TEXT | NOT NULL | — | FK → allocation_targets(id) CASCADE |
| subject_type | TEXT | NOT NULL | — | asset / account / category（CHECK） |
| subject_id | TEXT | NOT NULL | — | 主体 ID |
| action | TEXT | NOT NULL | — | buy / sell / trade（CHECK） |
| effect | TEXT | NOT NULL | 'block' | block / avoid |
| reason | TEXT | NULLABLE | — | 原因 |
| metadata_json | TEXT | NULLABLE | — | JSON 元数据 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

**唯一约束**：(target_id, subject_type, subject_id, action, effect)
**索引**：idx_allocation_target_constraints_target、idx_allocation_target_constraints_lookup

---

#### 2.18 portfolios — 组合

| 迁移版本 | 2026-05-11-000001_portfolios |
| --- | --- |
| Grade | 复用性 B / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| name | TEXT | NOT NULL | — | 名称 |
| description | TEXT | NULLABLE | — | 描述 |
| sort_order | Integer | NOT NULL | — | 排序 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

---

#### 2.19 portfolio_accounts — 组合账户关联

| 迁移版本 | 2026-05-11-000001_portfolios |
| --- | --- |
| Grade | 复用性 C / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| portfolio_id | TEXT | NOT NULL | — | FK → portfolios(id) |
| account_id | TEXT | NOT NULL | — | FK → accounts(id) |
| sort_order | Integer | NOT NULL | — | 排序 |
| created_at | TEXT | NOT NULL | — | 创建时间 |

**关系**：portfolio ←→ account（多对多，通过此关联表）

---

#### 2.20 market_data_providers — 行情数据提供商

| 迁移版本 | 2025-06-27-145729_create_market_data_providers_table |
| --- | --- |
| 最后修改 | 2026-03-25-000001_custom_provider_sources（添加 provider_type/config） |
| Grade | 复用性 B / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键（YAHOO、FINNHUB 等） |
| name | TEXT | NOT NULL | — | 名称 |
| description | TEXT | NOT NULL | — | 描述 |
| url | TEXT | NULLABLE | — | URL |
| priority | Integer | NOT NULL | — | 优先级 |
| enabled | Bool | NOT NULL | — | 启用 |
| logo_filename | TEXT | NULLABLE | — | Logo 文件名 |
| last_synced_at | TEXT | NULLABLE | — | 上次同步时间 |
| last_sync_status | TEXT | NULLABLE | — | 上次同步状态 |
| last_sync_error | TEXT | NULLABLE | — | 上次同步错误 |
| provider_type | TEXT | NOT NULL | — | 提供商类型（API / CUSTOM_SCRAPER） |
| config | TEXT | NULLABLE | — | JSON 配置 |

**种子数据**：YAHOO、MARKETDATA_APP、ALPHA_VANTAGE、FINNHUB、US_TREASURY_CALC、BOERSE_FRANKFURT、OPENFIGI、METAL_PRICE_API、CUSTOM_SCRAPER

---

#### 2.21 market_data_custom_providers — 自定义行情提供商

| 迁移版本 | 2026-03-25-000001_custom_provider_sources |
| --- | --- |
| Grade | 复用性 B / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| code | TEXT | NOT NULL | — | 代码 |
| name | TEXT | NOT NULL | — | 名称 |
| description | TEXT | NOT NULL | — | 描述 |
| enabled | Bool | NOT NULL | — | 启用 |
| priority | Integer | NOT NULL | — | 优先级 |
| config | TEXT | NULLABLE | — | JSON 配置 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

---

#### 2.22 quote_sync_state — 行情同步状态

| 迁移版本 | 2026-01-01-000001_quotes_market_data |
| --- | --- |
| Grade | 复用性 B / 迁移难度 中 / 代码风险 中 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| asset_id | TEXT | NOT NULL | — | 主键，FK → assets(id) |
| position_closed_date | TEXT | NULLABLE | — | 持仓关闭日期 |
| last_synced_at | TEXT | NULLABLE | — | 上次同步时间 |
| data_source | TEXT | NOT NULL | — | 数据源 |
| sync_priority | Integer | NOT NULL | — | 同步优先级 |
| error_count | Integer | NOT NULL | — | 错误计数 |
| last_error | TEXT | NULLABLE | — | 上次错误 |
| profile_enriched_at | TEXT | NULLABLE | — | profile 富化时间 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

---

#### 2.23 brokers_sync_state — 券商同步状态

| 迁移版本 | 2026-01-01-000000_refactor_asset_model |
| --- | --- |
| Grade | 复用性 C / 迁移难度 中 / 代码风险 中 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| account_id | TEXT | NOT NULL | — | 复合主键，FK → accounts(id) |
| provider | TEXT | NOT NULL | — | 复合主键 |
| checkpoint_json | TEXT | NULLABLE | — | 检查点 JSON |
| last_attempted_at | TEXT | NULLABLE | — | 上次尝试时间 |
| last_successful_at | TEXT | NULLABLE | — | 上次成功时间 |
| last_error | TEXT | NULLABLE | — | 上次错误 |
| last_run_id | TEXT | NULLABLE | — | FK → import_runs(id) |
| sync_status | TEXT | NOT NULL | 'IDLE' | 同步状态 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

**索引**：ix_brokers_sync_state_provider

---

#### 2.24 ai_threads — AI 会话线程

| 迁移版本 | 2026-01-15-000001_ai_chat_persistence |
| --- | --- |
| Grade | 复用性 B / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| title | TEXT | NULLABLE | — | 标题 |
| created_at | TEXT | NOT NULL | — | 创建时间 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |
| config_snapshot | TEXT | NULLABLE | — | 配置快照 JSON |
| is_pinned | Integer | NOT NULL | — | 置顶 |

---

#### 2.25 ai_messages — AI 消息

| 迁移版本 | 2026-01-15-000001_ai_chat_persistence |
| --- | --- |
| Grade | 复用性 B / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| thread_id | TEXT | NOT NULL | — | FK → ai_threads(id) |
| role | TEXT | NOT NULL | — | user / assistant / system |
| content_json | TEXT | NOT NULL | — | JSON 内容 |
| created_at | TEXT | NOT NULL | — | 创建时间 |

---

#### 2.26 ai_thread_tags — AI 线程标签

| 迁移版本 | 2026-01-15-000001_ai_chat_persistence |
| --- | --- |
| Grade | 复用性 C / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| thread_id | TEXT | NOT NULL | — | FK → ai_threads(id) |
| tag | TEXT | NOT NULL | — | 标签 |
| created_at | TEXT | NOT NULL | — | 创建时间 |

---

#### 2.27 health_issue_dismissals — 健康问题解除

| 迁移版本 | 2026-01-20-000001_health_issue_dismissals |
| --- | --- |
| Grade | 复用性 C / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| issue_id | TEXT | NOT NULL | — | 主键 |
| dismissed_at | TEXT | NOT NULL | — | 解除时间 |
| data_hash | TEXT | NOT NULL | — | 数据哈希（检测数据变化） |

---

#### 2.28 addon_storage — 插件存储

| 迁移版本 | 2026-07-08-000001_addon_storage |
| --- | --- |
| Grade | 复用性 B / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| addon_id | TEXT | NOT NULL | — | 复合主键 |
| key | TEXT | NOT NULL | — | 复合主键 |
| value | TEXT | NOT NULL | — | 值 |

**说明**：per-addon KV 存储。本地表，未来可能参与设备同步。

---

#### 2.29 personal_access_tokens — 个人访问令牌

| 迁移版本 | 2026-06-26-000001_agent_access |
| --- | --- |
| Grade | 复用性 B / 迁移难度 低 / 代码风险 中 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| name | TEXT | NOT NULL | — | 名称（CHECK length > 0） |
| token_prefix | TEXT | NOT NULL | — | 令牌前缀 |
| token_hash | TEXT | NOT NULL | UNIQUE | 令牌哈希（不存明文） |
| scopes_json | TEXT | NOT NULL | '[]' | 权限范围 JSON |
| expires_at | TEXT | NULLABLE | — | 过期时间 |
| last_used_at | TEXT | NULLABLE | — | 最后使用时间 |
| revoked_at | TEXT | NULLABLE | — | 撤销时间 |
| created_at | TEXT | NOT NULL | — | 创建时间 |

**安全**：仅存哈希（token_hash）+ 前缀用于展示；本地表不参与设备同步。

---

### 3. 审计/日志表（Audit / Log）

#### 3.1 app_settings — 应用设置（KV）

| 迁移版本 | 2024-09-21-023605_settings_to_kv（取代 settings 表） |
| --- | --- |
| Grade | 复用性 A / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| setting_key | TEXT | NOT NULL | — | 主键 |
| setting_value | TEXT | NOT NULL | — | 值 |

**说明**：键值存储。预置键：`theme`、`font`、`base_currency`、`instance_id`、`spending.enabled` 等。

---

#### 3.2 sync_cursor — 同步游标

| 迁移版本 | 2026-02-12-000001_device_sync_foundation |
| --- | --- |
| Grade | 复用性 C / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | Integer | NOT NULL | — | 主键 |
| cursor | BigInt | NOT NULL | — | 游标值 |
| updated_at | TEXT | NOT NULL | — | 更新时间 |

---

#### 3.3 sync_outbox — 同步发件箱

| 迁移版本 | 2026-02-12-000001_device_sync_foundation |
| --- | --- |
| Grade | 复用性 C / 迁移难度 中 / 代码风险 中 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| event_id | TEXT | NOT NULL | — | 主键 |
| entity | TEXT | NOT NULL | — | 实体类型 |
| entity_id | TEXT | NOT NULL | — | 实体 ID |
| op | TEXT | NOT NULL | — | 操作（INSERT/UPDATE/DELETE） |
| client_timestamp | TEXT | NOT NULL | — | 客户端时间戳 |
| payload | TEXT | NOT NULL | — | JSON 载荷 |
| payload_key_version | Integer | NOT NULL | — | 载荷密钥版本 |
| sent | Integer | NOT NULL | — | 是否已发送 |
| status | TEXT | NOT NULL | — | 状态 |
| retry_count | Integer | NOT NULL | — | 重试计数 |
| next_retry_at | TEXT | NULLABLE | — | 下次重试时间 |
| last_error | TEXT | NULLABLE | — | 上次错误 |
| last_error_code | TEXT | NULLABLE | — | 错误码 |
| device_id | TEXT | NULLABLE | — | 设备 ID |
| created_at | TEXT | NOT NULL | — | 创建时间 |

---

#### 3.4 sync_entity_metadata — 同步实体元数据

| 迁移版本 | 2026-02-12-000001_device_sync_foundation |
| --- | --- |
| 最后修改 | 2026-04-29-000001_sync_entity_metadata_last_op（添加 last_op） |
| Grade | 复用性 C / 迁移难度 中 / 代码风险 中 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| entity | TEXT | NOT NULL | — | 复合主键 |
| entity_id | TEXT | NOT NULL | — | 复合主键 |
| last_event_id | TEXT | NOT NULL | — | 最后事件 ID |
| last_client_timestamp | TEXT | NOT NULL | — | 最后客户端时间戳 |
| last_op | TEXT | NOT NULL | — | 最后操作（2026-04-29） |
| last_seq | BigInt | NOT NULL | — | 最后序列号 |

---

#### 3.5 sync_device_config — 同步设备配置

| 迁移版本 | 2026-02-12-000001_device_sync_foundation |
| --- | --- |
| 最后修改 | 2026-03-10-000001_sync_freshness_gate（添加 min_snapshot_created_at） |
| Grade | 复用性 C / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| device_id | TEXT | NOT NULL | — | 主键 |
| key_version | Integer | NULLABLE | — | 密钥版本 |
| trust_state | TEXT | NOT NULL | — | 信任状态 |
| last_bootstrap_at | TEXT | NULLABLE | — | 上次引导时间 |
| min_snapshot_created_at | TEXT | NULLABLE | — | 最小快照创建时间（新鲜度门槛） |

---

#### 3.6 sync_engine_state — 同步引擎状态

| 迁移版本 | 2026-02-12-000001_device_sync_foundation |
| --- | --- |
| Grade | 复用性 C / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | Integer | NOT NULL | — | 主键 |
| lock_version | BigInt | NOT NULL | — | 锁版本 |
| last_push_at | TEXT | NULLABLE | — | 上次推送时间 |
| last_pull_at | TEXT | NULLABLE | — | 上次拉取时间 |
| last_error | TEXT | NULLABLE | — | 上次错误 |
| consecutive_failures | Integer | NOT NULL | — | 连续失败次数 |
| next_retry_at | TEXT | NULLABLE | — | 下次重试时间 |
| last_cycle_status | TEXT | NULLABLE | — | 上次循环状态 |
| last_cycle_duration_ms | BigInt | NULLABLE | — | 上次循环耗时（毫秒） |

---

#### 3.7 sync_table_state — 同步表状态

| 迁移版本 | 2026-02-12-000001_device_sync_foundation |
| --- | --- |
| Grade | 复用性 C / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| table_name | TEXT | NOT NULL | — | 主键 |
| enabled | Integer | NOT NULL | — | 是否参与同步 |
| last_snapshot_restore_at | TEXT | NULLABLE | — | 上次快照恢复时间 |
| last_incremental_apply_at | TEXT | NULLABLE | — | 上次增量应用时间 |

**种子数据**：12 张参与同步的表名。

---

#### 3.8 sync_applied_events — 已应用事件

| 迁移版本 | 2026-02-12-000001_device_sync_foundation |
| --- | --- |
| Grade | 复用性 C / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| event_id | TEXT | NOT NULL | — | 主键 |
| seq | BigInt | NOT NULL | — | 序列号 |
| entity | TEXT | NOT NULL | — | 实体类型 |
| entity_id | TEXT | NOT NULL | — | 实体 ID |
| applied_at | TEXT | NOT NULL | — | 应用时间 |

---

#### 3.9 mcp_audit_log — MCP 审计日志

| 迁移版本 | 2026-06-26-000001_agent_access |
| --- | --- |
| Grade | 复用性 C / 迁移难度 低 / 代码风险 低 |

| 列名 | 类型 | 可空 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| id | TEXT | NOT NULL | — | 主键 |
| session_id | TEXT | NOT NULL | — | 会话 ID |
| actor_kind | TEXT | NOT NULL | — | local_token / pat / desktop_bridge（CHECK） |
| actor_fingerprint | TEXT | NOT NULL | — | 执行者指纹 |
| tool | TEXT | NOT NULL | — | 工具名 |
| scopes_json | TEXT | NOT NULL | '[]' | 权限范围 |
| args_summary | TEXT | NULLABLE | — | 参数摘要 |
| outcome | TEXT | NOT NULL | — | success / denied / error（CHECK） |
| error_message | TEXT | NULLABLE | — | 错误信息 |
| created_at | TEXT | NOT NULL | — | 创建时间 |

**索引**：idx_mcp_audit_created_tool — (created_at, tool)
**说明**：桌面与服务器模式通用；行保留至手动清理；本地表不参与设备同步。

---

### 4. 表关系图（Relationship Diagram）

```
platforms (1) ──< accounts (N)
                                │
                                ├──< activities (N) ──< import_runs (1)
                                │        │                │
                                │        ├──< asset (assets 1..N)  ──< quotes
                                │        │                            └──< quote_sync_state (1:1)
                                │        ├──< activity_taxonomy_assignments
                                │        ├──< spending_activity_events (1:1)
                                │        ├──< spending_activity_splits
                                │        └──< lot_disposals (disposal_activity_id)
                                │
                                ├──< holdings_snapshots (1) ──< snapshot_positions (N)
                                ├──< daily_account_valuation (N)
                                ├──< brokers_sync_state (composite PK)
                                ├──< goals_allocation (N) ──< goals (1)
                                │                              └──< goal_plans (1:1)
                                ├──< spending_categorization_rules
                                ├──< portfolio_accounts (N) ──< portfolios (1)
                                └──< import_account_templates ──< import_templates (1)

assets (1) ──< lots (N) ──< lot_disposals (N)
assets (1) ──< asset_taxonomy_assignments (N) ──> taxonomies (M)
taxonomies (1) ──< taxonomy_categories (N)  （自引用 parent_id 树形）
taxonomy_categories ──< activity_taxonomy_assignments / spending_categorization_rules / budget_*/spending_*（复合 FK）

allocation_targets (1) ──< allocation_target_weights (N)
allocation_targets (1) ──< allocation_target_constraints (N)

spending_event_types (1) ──< spending_events (N) ──< spending_activity_events (N) ──> activities (M)

budget_groups (1) ──< budget_group_assignments (N)
budget_groups (1) ──< budget_targets (N)
budget_groups (1) ──< budget_rollover_settings (N)

ai_threads (1) ──< ai_messages (N)
ai_threads (1) ──< ai_thread_tags (N)

（设备同步表为独立集群，通过 payload/entity_id 逻辑关联，无物理外键。）
```

#### 多对多关系汇总

| 关系 | 关联表 | 说明 |
| --- | --- | --- |
| portfolio ←→ account | portfolio_accounts | 组合与账户多对多 |
| activity ←→ spending event | spending_activity_events | 但 PK=activity_id，实际为 1:1 设计 |
| budget_group ←→ taxonomy_category | budget_group_assignments | 分组与分类多对多 |

#### 树形 / 自引用

- `taxonomy_categories.parent_id` 自引用，构建三级区域树（continent → subregion → country）。

---

### 5. 触发器（Triggers）

共 3 个触发器，全部位于 allocation_targets / allocation_target_weights，用于维护"权重的 taxonomy 必须与目标一致"的不变量：

| 触发器 | 表 | 时机 | 逻辑 |
| --- | --- | --- | --- |
| allocation_targets_taxonomy_update | allocation_targets | BEFORE UPDATE OF taxonomy_id | 若该目标已存在权重且 taxonomy_id 变化 → RAISE(ABORT) |
| allocation_target_weights_taxonomy_insert | allocation_target_weights | BEFORE INSERT | 若权重的 taxonomy_id 与所属目标的 taxonomy_id 不一致 → RAISE(ABORT) |
| allocation_target_weights_taxonomy_update | allocation_target_weights | BEFORE UPDATE OF target_id, taxonomy_id | 若分类法不匹配 → RAISE(ABORT) |

**注意**：db 中没有视图（VIEW）；查询视图均为应用层（Rust）组装。

---

### 6. 已废弃 / 被取代的表（历史遗留）

以下表在迁移历史中存在但已被后续迁移删除，当前 schema 中**不存在**：

| 表 | 引入迁移 | 取代迁移 | 现状 |
| --- | --- | --- | --- |
| settings | 2023-11-08-162221_init_db | 2024-09-21-023605_settings_to_kv | 被 app_settings 取代 |
| exchange_rates | 2024-09-16-023604_portfolio_history | 2025-01-27-000001_migrate_fx_to_quotes | 被 quotes 取代 |
| portfolio_history | 2024-09-16-023604_portfolio_history | 2025-04-21-195716_create_daily_account_history | 被 holdings_snapshots + daily_account_valuation 取代 |
| activity_import_profiles | 2024-10-15-173026_csv_import_profiles | 2026-03-19-000001_import_templates | 被 import_templates + import_account_templates 取代 |
| account_mappings（列） | 2025-06-11-133126_account_import_mapping | 2026-01-24-000001_improve_import_profiles | 被模板配置取代 |
| legacy_asset_id_map | 2026-01-01-000000_refactor_asset_model | 2026-01-01-000001_quotes_market_data | 迁移期临时表，V2 迁移后丢弃 |

**注意**：`assets` 表原有 CASH 资产行在 v2 迁移中被删除（`WHERE NOT (asset_type IN ('Cash','CASH') OR id LIKE '$CASH-%')`），现金改由 accounts.currency + activities 表达。

---

### 7. 需要特别关注的跨表数据约束（应用层不变量）

| 不变量 | 涉及表 | 维护方式 |
| --- | --- | --- |
| 权重 taxonomy 与目标一致 | allocation_targets / allocation_target_weights | 数据库触发器 |
| budget_targets 类型与字段一致性 | budget_targets | CHECK 约束 |
| 全局规则与账户规则互斥 | spending_categorization_rules | CHECK 约束 |
| is_global=1 ↔ account_id IS NULL | spending_categorization_rules | CHECK 约束 |
| 派生读模型可重建 | lots / lot_disposals / daily_account_valuation / CALCULATED snapshots | 迁移中 DELETE + 应用启动回填 |
| spending 模块活动分类 | activities.notes 存 payee/商户 | 规则 pattern 匹配，无专用名称列 |

---

### 8. 评级汇总表

| 表 | 复用性 | 迁移难度 | 代码风险 | 分类 |
| --- | --- | --- | --- | --- |
| accounts | A | 低 | 低 | 核心 |
| activities | A | 高 | 高 | 核心 |
| assets | A | 高 | 高 | 核心 |
| platforms | B | 低 | 低 | 核心 |
| quotes | A | 中 | 中 | 核心 |
| import_runs | B | 中 | 中 | 核心 |
| import_templates | B | 中 | 中 | 核心 |
| import_account_templates | C | 低 | 低 | 核心 |
| lots | A | 高 | 高 | 核心 |
| lot_disposals | B | 中 | 中 | 核心 |
| holdings_snapshots | A | 中 | 中 | 核心 |
| snapshot_positions | B | 中 | 中 | 核心 |
| daily_account_valuation | A | 中 | 中 | 核心 |
| goals | A | 中 | 中 | 核心 |
| goal_plans | B | 中 | 中 | 核心 |
| goals_allocation | B | 低 | 低 | 核心 |
| contribution_limits | B | 低 | 低 | 核心 |
| taxonomies | A | 低 | 低 | 功能 |
| taxonomy_categories | A | 低 | 低 | 功能 |
| asset_taxonomy_assignments | A | 中 | 中 | 功能 |
| activity_taxonomy_assignments | B | 中 | 中 | 功能 |
| spending_activity_events | B | 低 | 低 | 功能 |
| spending_event_types | B | 低 | 低 | 功能 |
| spending_events | B | 低 | 低 | 功能 |
| spending_activity_splits | B | 低 | 低 | 功能 |
| spending_categorization_rules | B | 中 | 中 | 功能 |
| spending_preset_rule_deletions | C | 低 | 低 | 功能 |
| budget_groups | B | 低 | 低 | 功能 |
| budget_group_assignments | C | 低 | 低 | 功能 |
| budget_targets | B | 中 | 中 | 功能 |
| budget_rollover_settings | C | 中 | 低 | 功能 |
| allocation_targets | B | 中 | 中 | 功能 |
| allocation_target_weights | B | 中 | 中 | 功能 |
| allocation_target_constraints | C | 低 | 低 | 功能 |
| portfolios | B | 低 | 低 | 功能 |
| portfolio_accounts | C | 低 | 低 | 功能 |
| market_data_providers | B | 低 | 低 | 功能 |
| market_data_custom_providers | B | 低 | 低 | 功能 |
| quote_sync_state | B | 中 | 中 | 功能 |
| brokers_sync_state | C | 中 | 中 | 功能 |
| ai_threads | B | 低 | 低 | 功能 |
| ai_messages | B | 低 | 低 | 功能 |
| ai_thread_tags | C | 低 | 低 | 功能 |
| health_issue_dismissals | C | 低 | 低 | 功能 |
| addon_storage | B | 低 | 低 | 功能 |
| personal_access_tokens | B | 低 | 中 | 功能 |
| app_settings | A | 低 | 低 | 审计/配置 |
| sync_cursor | C | 低 | 低 | 审计/同步 |
| sync_outbox | C | 中 | 中 | 审计/同步 |
| sync_entity_metadata | C | 中 | 中 | 审计/同步 |
| sync_device_config | C | 低 | 低 | 审计/同步 |
| sync_engine_state | C | 低 | 低 | 审计/同步 |
| sync_table_state | C | 低 | 低 | 审计/同步 |
| sync_applied_events | C | 低 | 低 | 审计/同步 |
| mcp_audit_log | C | 低 | 低 | 审计/日志 |

---

### 9. 关键观察与建议

1. **BigDecimal 用 TEXT、货币金额无 DECIMAL 列**：SQLite 无原生 decimal；全部金额用 TEXT（字符串）存储，由 Rust BigDecimal 处理。迁移（如 2025-03-18）将 NUMERIC/REAL 列重建为 TEXT。
2. **派生读模型可重建性**：lots、lot_disposals、daily_account_valuation、CALCULATED holdings_snapshots 均为派生数据，迁移可清空重算 —— 这降低了代码风险（源数据仅 activities + 手动快照），但也意味着这些表对应用启动回填逻辑强依赖。
3. **schema.rs 与迁移的映射**：schema.rs 由 Diesel CLI 生成（`@generated`），与迁移最终态一一对应，共 60 张表。schema.rs 中未出现 `portfolio_history`、`exchange_rates` 等已废弃表，与第 6 节一致。
4. **触发器极少**：仅配置目标相关的 3 个；其余不变量（如 budget_targets 字段一致性）用 CHECK 或应用层维护。
5. **设备同步表是独立的集群**：7 张 sync_* 表无物理外键，通过 entity/entity_id/payload 逻辑关联；`sync_table_state` 的 12 张注册表确定了哪些表参与同步。
6. **索引冗余清理已发生**：2026-08-02 删除了 quotes 的冗余前缀索引，并执行 VACUUM 回收空间 —— 说明团队已有关注索引冗余与文件体积的意识。
7. **复用性最高的表**：accounts/assets/activities/quotes/taxonomies 系列复用性 A，任何新功能几乎都会引用；预算/分配/同步表复用性低（C）。
8. **安全实践**：personal_access_tokens 只存哈希 + 前缀，不存明文令牌。

---

（本节结束）test

## TypeScript 类型层

### 0. 总览

本节覆盖前端 TypeScript 类型体系，分为 6 个子层：

| 子层 | 来源目录 | 说明 |
| --- | --- | --- |
| L1 Zod 校验层 | `apps/frontend/src/lib/schemas.ts` | 表单/导入输入校验，派生类型 |
| L2 常量枚举层 | `apps/frontend/src/lib/constants.ts` | 领域枚举 + 能力矩阵 |
| L3 核心领域类型层 | `apps/frontend/src/lib/types.ts` | 最大类型文件（2718 行），跨端契约 |
| L4 适配器类型层 | `apps/frontend/src/adapters/types.ts` + `adapters/shared/*` | IPC 边界（Tauri/Web 双端） |
| L5 功能特性类型层 | `apps/frontend/src/features/*` | 按功能域组织的独立类型 |
| L6 共享包类型层 | `packages/ui`、`packages/addon-sdk` | 跨应用复用（UI 组件 + 插件 SDK） |

**评级体系**（与数据库层一致）：
- 复用性：A（高） / B（中） / C（低）
- 迁移难度：低 / 中 / 高
- 代码风险：低 / 中 / 高

---

### L1 Zod 校验层

来源：`F:\dev\wealthfolio\apps\frontend\src\lib\schemas.ts`（393 行）

该层定义前端的表单/导入输入校验规则，使用 `zod` 库，通过 `z.infer` 派生 TypeScript 类型。所有 Schema 均为 **Input** 角色（后端由 Rust 独立校验）。

| Schema 名称 | 用途 | 关键字段（含校验） | 推断类型 | 评级 |
| --- | --- | --- | --- | --- |
| `parseConfigSchema` | CSV 解析配置 | hasHeaderRow(boolean?)、delimiter(string?)、dateFormat(string?)、decimalSeparator(string?)、thousandsSeparator(string?)、defaultCurrency(string?) | `ParseConfig` | A/低/低 |
| `importMappingSchema` | CSV 导入字段映射 | accountId(z.string())、importType(enum)、fieldMappings(record)、activityMappings(record)、symbolMappings(record)、symbolMappingMeta(record→object)、parseConfig(parseConfigSchema) | `ImportMappingData` | A/低/低 |
| `trackingModeSchema` | 账户跟踪模式 | z.enum(["TRANSACTIONS","HOLDINGS","NOT_SET"]) | `TrackingMode` | A/低/低 |
| `newAccountSchema` | 新建账户表单 | name(2-50 chars)、accountType(enum)、currency(required)、trackingMode(default NOT_SET)；refine：信用卡不能 HOLDINGS | `NewAccount` | A/低/低 |
| `newGoalSchema` | 新建目标表单 | goalType(6种枚举)、title(string)、targetAmount(z.coerce.number ≥0)、currency(string?)、targetDate(string?) | `NewGoal` | B/低/低 |
| `importActivitySchema` | 活动导入校验 | accountId(required)、activityType(enum)、symbol(regex)、amount/quantity/unitPrice/fee/tax(decimalLikeSchema)、10 个 refine 规则（cash/trade/fee/split 各场景校验） | `ActivityImport` | A/低/低 |
| `newContributionLimitSchema` | 新建缴存限额 | groupName(required)、contributionYear(int ≥1900)、limitAmount(coerce.number ≥0)、startDate/endDate(date|string|nullable) | `NewContributionLimit` | C/低/低 |

**关键校验规则总结**：

| 规则 | Schema | 描述 |
| --- | --- | --- |
| 信用卡限制 | newAccountSchema | 信用卡账户不能使用 HOLDINGS 跟踪模式 |
| 符号格式 | importActivitySchema | 正则 `/^(?=.{1,100}$)(CASH:[A-Z]{3}\|[A-Z0-9_]+([.-][A-Z0-9_]+){0,2})$/` |
| 现金活动要求 | importActivitySchema | 现金/收入/转账类活动至少需要 amount/quantity/unitPrice 之一 |
| 手续费活动 | importActivitySchema | FEE 类型至少需要 fee 或 amount |
| 交易活动价格 | importActivitySchema | BUY/SELL 必须提供正数 unitPrice |
| 拆分比率 | importActivitySchema | SPLIT 必须提供正数 amount |
| 非现金活动数量 | importActivitySchema | 非现金/非交易活动必须提供正数 quantity |
| FX 汇率 | importActivitySchema | 必须为正数 |
| 代理类型 | parsingSchema | `decimalLikeSchema`：z.union([z.number(), z.string()])，字符串必须可解析为有限数字 |

**推断类型映射**：

| Zod Schema | 推断 TypeScript 类型 | 文件位置 |
| --- | --- | --- |
| `z.infer<typeof parseConfigSchema>` | `ParseConfig` | `lib/types.ts` (106-119) |
| `z.infer<typeof importMappingSchema>` | `ImportMappingData` | `lib/types.ts` (1692-1709) |
| `z.infer<typeof newAccountSchema>` | `NewAccount` | 内联，无独立命名类型 |
| `z.infer<typeof newGoalSchema>` | `NewGoal` | 内联 |
| `z.infer<typeof importActivitySchema>` | `ActivityImport` | `lib/types.ts` 内联 / 适配器层使用 |
| `z.infer<typeof newContributionLimitSchema>` | `NewContributionLimit` | 内联 |

---

### L2 常量枚举层

来源：`F:\dev\wealthfolio\apps\frontend\src\lib\constants.ts`（758 行）

该文件定义了 Wealthfolio 前端使用的所有常量枚举、类型映射和辅助函数，共 758 行。以下为主要枚举对象：

| 枚举名称 | 值数量 | 值列表 | 关联 Zod Schema | 复用性 | 迁移难度 | 代码风险 |
|---------|-------|-------|----------------|-------|---------|---------|
| `AccountType` | 4 | SECURITIES, CASH, CREDIT_CARD, CRYPTOCURRENCY | `accountTypeSchema` (z.enum) | A | 低 | 低 |
| `AccountPurpose` | 7 | spending, performance, holdings, income, goalFunding, contributionLimits, netWorth | 无 | A | 低 | 低 |
| `ActivityType` | 14 | BUY, SELL, SPLIT, DIVIDEND, INTEREST, DEPOSIT, WITHDRAWAL, TRANSFER_IN, TRANSFER_OUT, FEE, TAX, CREDIT, ADJUSTMENT, UNKNOWN | `activityTypeSchema` (z.enum) | A | 低 | 低 |
| `ActivityStatus` | 4 | POSTED, PENDING, DRAFT, VOID | 无 | A | 低 | 低 |
| `ACTIVITY_SUBTYPES` | 9 | DRIP, DIVIDEND_IN_KIND, STAKING_REWARD, BONUS, REBATE, REFUND, REIMBURSEMENT, OPTION_EXPIRY, POSITION_OPEN, POSITION_CLOSE | 无 | A | 低 | 低 |
| `AssetKind` | 9 | INVESTMENT, PROPERTY, VEHICLE, COLLECTIBLE, PRECIOUS_METAL, PRIVATE_EQUITY, LIABILITY, OTHER, FX | 无 | A | 低 | 低 |
| `AlternativeAssetKind` | 6 | PROPERTY, VEHICLE, COLLECTIBLE, PRECIOUS_METAL, LIABILITY, OTHER | 无 | B | 低 | 低 |
| `InstrumentType` | 6 | EQUITY, CRYPTO, FX, OPTION, METAL, BOND | 无 | A | 低 | 低 |
| `QuoteMode` | 2 | MARKET, MANUAL | `quoteModeSchema` (z.enum) | A | 低 | 低 |
| `DataSource` | 2 | YAHOO, MANUAL | `dataSourceSchema` (z.string) | B | 低 | 低 |
| `HoldingType` | 2 | CASH, SECURITY | 无 | A | 低 | 低 |
| `ImportFormat` | 16 | date, account, activityType, symbol, isin, quantity, unitPrice, amount, currency, fee, tax, comment, fxRate, subtype, instrumentType | `importFormatSchema` (z.enum) | A | 低 | 低 |
| `ExportDataType` | 5 | accounts, activities, holdings, goals, portfolio-history | `exportDataTypeSchema` (z.enum) | B | 低 | 低 |
| `ExportedFileFormat` | 3 | CSV, JSON, SQLite | `exportedFileFormatSchema` (z.enum) | B | 低 | 低 |
| `HoldingType` | 2 | cash, security | 无 | A | 低 | 低 |

**关键接口：**
- `AccountCapabilities` — 定义账户支持的能力矩阵（7 个布尔字段）
- `POSITION_INTENT_ALIASES` — 买卖方向意图别名映射表（Buy Open/Close, Sell Open/Close）
- `HOLDING_CATEGORY_FILTERS` — 持仓分类过滤器配置（investments/assets/liabilities）
- `EXCHANGE_DISPLAY_NAMES` — 交易所 MIC 码到显示名称的映射（~40 个条目）

**模式总结：** 所有枚举采用 `as const` 对象模式，同时导出 `type` 同名类型别名。部分枚举有关联的 Zod Schema（`z.enum()`）用于运行时校验。各枚举之间通过 `AccountPurpose` 和 `ACCOUNT_PURPOSE_TYPES` 建立交叉引用关系。

---

### L3 核心业务类型层

来源：`F:\dev\wealthfolio\apps\frontend\src\lib\types.ts`（2718 行）

这是整个前端的数据类型中枢，定义了所有核心业务实体和 DTO。以下按功能域分组列出主要类型。

#### 账户域

| 类型名称 | 类型（接口/联合/类型别名） | 关键字段 | 扩展/实现关系 | 复用性 | 迁移难度 | 代码风险 |
|---------|--------------------------|---------|--------------|-------|---------|---------|
| `Account` | interface | id, name, accountType, balance, currency, isDefault, isActive, isArchived, trackingMode, createdAt, updatedAt, platformId?, accountNumber?, provider? | 无 | A | 低 | 低 |
| `AccountScope` | discriminated union | `{type: "all"}` \| `{type: "account"; accountId}` \| `{type: "portfolio"; portfolioId}` \| `{type: "accounts"; accountIds}` | 无 | A | 低 | 低 |
| `PortfolioWithAccounts` | interface | id, name, accountIds, sortOrder, createdAt, updatedAt | 无 | A | 低 | 低 |
| `NewPortfolio` | interface | name, accountIds, description?, sortOrder? | 无 | B | 低 | 低 |
| `AccountValuation` | interface | accountId, valuationDate, cashBalance, investmentMarketValue, totalValue, ... | 无 | A | 低 | 低 |
| `CurrentAccountValuation` | interface | accountId, cashBalance, investmentMarketValue, totalValue, ... | 无 | A | 低 | 低 |
| `CurrentValuationResponse` | interface | summary (CurrentValuationSummary), accounts (CurrentAccountValuation[]) | 组合 | A | 低 | 低 |
| `AccountGroup` | interface | groupName, accounts (AccountSummaryView[]), totalValueBaseCurrency, performance | 组合 | A | 低 | 低 |

#### 活动/交易域

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `Activity` | interface | id, accountId, assetId?, activityType, activityTypeOverride?, sourceType?, subtype?, status, activityDate, settlementDate?, quantity?, unitPrice?, amount?, fee?, tax?, currency, fxRate?, notes?, isUserModified, needsReview, createdAt, updatedAt | A | 低 | 低 |
| `ActivityLegacy` | interface (deprecated) | id, type, date, quantity, unitPrice, currency, fee, tax?, isDraft, accountId? | B | 高 | 中 |
| `ActivityDetails` | interface | id, activityType, subtype?, date, quantity, unitPrice, amount, fee, tax?, currency, needsReview, assetId, accountId, accountName, assetSymbol, transferOutId?, transferInId?, counterpartActivityId? | A | 低 | 低 |
| `ActivitySearchResponse` | interface | data (ActivityDetails[]), meta (totalRowCount) | 组合 | A | 低 | 低 |
| `ActivityCreate` | interface | accountId, activityType, activityDate, asset?, quantity?, unitPrice?, amount?, currency?, fee?, tax?, comment?, fxRate? | A | 低 | 低 |
| `ActivityUpdate` | interface | id, accountId, activityType, activityDate, ...（同 ActivityCreate + id） | A | 低 | 低 |
| `ActivityBulkMutationRequest` | interface | creates? (ActivityCreate[]), updates? (ActivityUpdate[]), deleteIds? (string[]) | 组合 | A | 低 | 低 |
| `ActivityBulkMutationResult` | interface | created, updated, deleted, createdMappings, errors | 组合 | A | 低 | 低 |
| `InternalTransferPairRequest` | interface | fromAccountId, toAccountId, activityDate, sourceAmount, destinationAmount, ... | A | 低 | 低 |
| `InternalTransferPairResponse` | interface | transferOut (Activity), transferIn (Activity) | 组合 | A | 低 | 低 |
| `TransferMatchCandidate` | interface | activity (Activity), matchKind, confidence, score, reasons | 组合 | A | 低 | 低 |

#### 资产与持仓域

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `Asset` | interface | id, kind, name?, displayCode?, quoteMode, quoteCcy, instrumentType?, instrumentSymbol?, instrumentExchangeMic?, instrumentKey?, providerConfig? | A | 低 | 低 |
| `NewAsset` | interface | kind, name?, displayCode?, isActive, quoteMode, quoteCcy, instrumentType?, instrumentSymbol?, ... | B | 低 | 低 |
| `UpdateAssetProfile` | interface | id, displayCode?, name?, kind?, quoteMode?, quoteCcy?, instrumentType?, instrumentExchangeMic? | B | 低 | 低 |
| `Instrument` | interface | id, symbol, name?, currency, quoteMode, isin?, exchangeMic?, classifications? | A | 低 | 低 |
| `Holding` | interface | id, holdingType, accountId, instrument?, assetKind?, quantity, marketValue(MonetaryValue), costBasis?, price?, unrealizedGain?, realizedGain?, totalGain?, weight, asOfDate | A | 低 | 低 |
| `HoldingSummary` | interface | id, symbol, name?, holdingType, quantity, marketValue, weightInCategory | A | 低 | 低 |
| `AllocationHoldings` | interface | taxonomyId, taxonomyName, categoryId, categoryName, color, holdings (HoldingSummary[]), totalValue | A | 低 | 低 |
| `Position` | interface | id, accountId, assetId, quantity, averageCost, totalCostBasis, currency, lots (Lot[]) | A | 低 | 低 |
| `Lot` | interface | id, positionId, acquisitionDate, quantity, costBasis, acquisitionPrice, acquisitionFees | A | 低 | 低 |
| `AssetLotView` | interface | id, accountId, assetId, source, currency, quantity, costBasis, unitCost, fees, ...（22+ 字段） | A | 低 | 低 |
| `CashHolding` | interface | id, accountId, currency, amount, lastUpdated | A | 低 | 低 |
| `MonetaryValue` | interface | local (number), base (number) | A | 低 | 低 |

#### 行情与市场数据域

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `Quote` | interface | id, assetId, timestamp, open, high, low, close, adjclose, volume, currency, dataSource | A | 低 | 低 |
| `LatestQuoteSnapshot` | interface | quote? (Quote), isStale, effectiveMarketDate, noQuoteReason? | 组合 | A | 低 | 低 |
| `QuoteUpdate` | interface | assetId, timestamp, open, high, low, close, volume, dataSource | B | 低 | 低 |
| `SymbolSearchResult` | interface | symbol, exchange, exchangeMic?, shortName, longName, quoteType, score, currency? | A | 低 | 低 |
| `MarketDataProviderInfo` | interface | id, name, logoFilename, lastSyncedDate | B | 低 | 低 |
| `ExchangeRate` | interface | id, fromCurrency, toCurrency, rate, source, timestamp | A | 低 | 低 |

#### 投资组合表现域

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `PerformanceResult` | interface | scope, period, mode, returns, attribution, risk, dataQuality, series, isHoldingsMode? | A | 低 | 低 |
| `PerformanceReturns` | interface | twr?, annualizedTwr?, irr?, annualizedIrr?, valueReturn?, annualizedValueReturn? | A | 低 | 低 |
| `PerformanceAttribution` | interface | contributions, distributions, income, realizedPnl, unrealizedPnlChange, fxEffect, fees, taxes, residual | A | 低 | 低 |
| `PerformanceRisk` | interface | volatility?, maxDrawdown?, peakDate?, troughDate?, recoveryDate? | A | 低 | 低 |
| `PerformanceDataQuality` | interface | status ("ok"\|"partial"\|"noData"\|"notApplicable"), warnings?, notApplicableReasons? | A | 低 | 低 |
| `ReturnData` | interface | date (string), value (number) | A | 低 | 低 |
| `IncomeSummary` | interface | period, byMonth, byType, byAsset, byCurrency, byAccount, totalIncome, currency, monthlyAverage, yoyGrowth | A | 低 | 低 |
| `SimplePerformanceResult` | interface | accountId, totalValue?, accountCurrency?, baseCurrency?, totalGainLossAmount?, cumulativeReturnPercent? | B | 低 | 低 |

#### 目标与退休规划域

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `Goal` | interface | id, goalType, title, targetAmount?, statusLifecycle, statusHealth, priority, currency?, startDate?, targetDate?, summaryCurrentValue?, summaryProgress? | A | 低 | 低 |
| `NewGoal` | interface | goalType, title, targetAmount?, statusLifecycle?, priority?, currency? | B | 低 | 低 |
| `GoalFundingRule` | interface | id, goalId, accountId, sharePercent, taxBucket? | B | 低 | 低 |
| `GoalPlan` | interface | goalId, planKind, settingsJson, summaryJson, version | B | 低 | 低 |
| `RetirementOverview` | interface | analysisMode, status, successStatus, desiredFireAge, fiAge, portfolioNow, portfolioAtRetirementStart, netFireTarget, grossFireTarget, ...（30+ 字段） | A | 低 | 低 |
| `RetirementTrajectoryPoint` | interface | age, year, phase, portfolioStart, annualContribution, annualExpenses, portfolioEnd, ... | A | 低 | 低 |
| `BudgetBreak
### L3 核心业务类型层

来源：`F:\dev\wealthfolio\apps\frontend\src\lib\types.ts`（2718 行）

这是整个前端的数据类型中枢，定义了所有核心业务实体和 DTO。以下按功能域分组列出主要类型。

#### 账户域

| 类型名称 | 类型（接口/联合/类型别名） | 关键字段 | 扩展/实现关系 | 复用性 | 迁移难度 | 代码风险 |
|---------|--------------------------|---------|--------------|-------|---------|---------|
| `Account` | interface | id, name, accountType, balance, currency, isDefault, isActive, isArchived, trackingMode, createdAt, updatedAt, platformId?, accountNumber?, provider? | 无 | A | 低 | 低 |
| `AccountScope` | discriminated union | `{type: "all"}` / `{type: "account"; accountId}` / `{type: "portfolio"; portfolioId}` / `{type: "accounts"; accountIds}` | 无 | A | 低 | 低 |
| `PortfolioWithAccounts` | interface | id, name, accountIds, sortOrder, createdAt, updatedAt | 无 | A | 低 | 低 |
| `NewPortfolio` | interface | name, accountIds, description?, sortOrder? | 无 | B | 低 | 低 |
| `AccountValuation` | interface | accountId, valuationDate, cashBalance, investmentMarketValue, totalValue, costBasis, netContribution, ... | 无 | A | 低 | 低 |
| `CurrentAccountValuation` | interface | accountId, cashBalance, investmentMarketValue, totalValue, sourceDataAsOf, warnings | 无 | A | 低 | 低 |
| `CurrentValuationResponse` | interface | summary (CurrentValuationSummary), accounts (CurrentAccountValuation[]) | 组合 | A | 低 | 低 |
| `AccountGroup` | interface | groupName, accounts (AccountSummaryView[]), totalValueBaseCurrency, performance | 组合 | A | 低 | 低 |

#### 活动/交易域

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `Activity` | interface | id, accountId, assetId?, activityType, activityTypeOverride?, sourceType?, subtype?, status, activityDate, settlementDate?, quantity?, unitPrice?, amount?, fee?, tax?, currency, fxRate?, notes?, isUserModified, needsReview, createdAt, updatedAt | A | 低 | 低 |
| `ActivityLegacy` | interface (deprecated) | id, type, date, quantity, unitPrice, currency, fee, tax?, isDraft, accountId? | B | 高 | 中 |
| `ActivityDetails` | interface | id, activityType, subtype?, date, quantity, unitPrice, amount, fee, tax?, currency, needsReview, assetId, accountId, accountName, assetSymbol, transferOutId?, transferInId?, counterpartActivityId? | A | 低 | 低 |
| `ActivitySearchResponse` | interface | data (ActivityDetails[]), meta (totalRowCount) | 组合 | A | 低 | 低 |
| `ActivityCreate` | interface | accountId, activityType, activityDate, asset?, quantity?, unitPrice?, amount?, currency?, fee?, tax?, comment?, fxRate? | A | 低 | 低 |
| `ActivityUpdate` | interface | id, accountId, activityType, activityDate, ...（同 ActivityCreate + id） | A | 低 | 低 |
| `ActivityBulkMutationRequest` | interface | creates? (ActivityCreate[]), updates? (ActivityUpdate[]), deleteIds? (string[]) | 组合 | A | 低 | 低 |
| `ActivityBulkMutationResult` | interface | created, updated, deleted, createdMappings, errors | 组合 | A | 低 | 低 |
| `InternalTransferPairRequest` | interface | fromAccountId, toAccountId, activityDate, sourceAmount, destinationAmount, ... | A | 低 | 低 |
| `InternalTransferPairResponse` | interface | transferOut (Activity), transferIn (Activity) | 组合 | A | 低 | 低 |
| `TransferMatchCandidate` | interface | activity (Activity), matchKind, confidence, score, reasons | 组合 | A | 低 | 低 |

#### 资产与持仓域

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `Asset` | interface | id, kind, name?, displayCode?, quoteMode, quoteCcy, instrumentType?, instrumentSymbol?, instrumentExchangeMic?, instrumentKey?, providerConfig? | A | 低 | 低 |
| `NewAsset` | interface | kind, name?, displayCode?, isActive, quoteMode, quoteCcy, instrumentType?, instrumentSymbol?, ... | B | 低 | 低 |
| `UpdateAssetProfile` | interface | id, displayCode?, name?, kind?, quoteMode?, quoteCcy?, instrumentType?, instrumentExchangeMic? | B | 低 | 低 |
| `Instrument` | interface | id, symbol, name?, currency, quoteMode, isin?, exchangeMic?, classifications? | A | 低 | 低 |
| `Holding` | interface | id, holdingType, accountId, instrument?, assetKind?, quantity, marketValue(MonetaryValue), costBasis?, price?, unrealizedGain?, realizedGain?, totalGain?, weight, asOfDate | A | 低 | 低 |
| `HoldingSummary` | interface | id, symbol, name?, holdingType, quantity, marketValue, weightInCategory | A | 低 | 低 |
| `AllocationHoldings` | interface | taxonomyId, taxonomyName, categoryId, categoryName, color, holdings (HoldingSummary[]), totalValue | A | 低 | 低 |
| `Position` | interface | id, accountId, assetId, quantity, averageCost, totalCostBasis, currency, lots (Lot[]) | A | 低 | 低 |
| `Lot` | interface | id, positionId, acquisitionDate, quantity, costBasis, acquisitionPrice, acquisitionFees | A | 低 | 低 |
| `AssetLotView` | interface | id, accountId, assetId, source, currency, quantity, costBasis, unitCost, fees, ...（22+ 字段） | A | 低 | 低 |
| `CashHolding` | interface | id, accountId, currency, amount, lastUpdated | A | 低 | 低 |
| `MonetaryValue` | interface | local (number), base (number) | A | 低 | 低 |

#### 行情与市场数据域

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `Quote` | interface | id, assetId, timestamp, open, high, low, close, adjclose, volume, currency, dataSource | A | 低 | 低 |
| `LatestQuoteSnapshot` | interface | quote? (Quote), isStale, effectiveMarketDate, noQuoteReason? | 组合 | A | 低 | 低 |
| `QuoteUpdate` | interface | assetId, timestamp, open, high, low, close, volume, dataSource | B | 低 | 低 |
| `SymbolSearchResult` | interface | symbol, exchange, exchangeMic?, shortName, longName, quoteType, score, currency? | A | 低 | 低 |
| `MarketDataProviderInfo` | interface | id, name, logoFilename, lastSyncedDate | B | 低 | 低 |
| `ExchangeRate` | interface | id, fromCurrency, toCurrency, rate, source, timestamp | A | 低 | 低 |

#### 投资组合表现域

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `PerformanceResult` | interface | scope, period, mode, returns, attribution, risk, dataQuality, series, isHoldingsMode? | A | 低 | 低 |
| `PerformanceReturns` | interface | twr?, annualizedTwr?, irr?, annualizedIrr?, valueReturn?, annualizedValueReturn? | A | 低 | 低 |
| `PerformanceAttribution` | interface | contributions, distributions, income, realizedPnl, unrealizedPnlChange, fxEffect, fees, taxes, residual | A | 低 | 低 |
| `PerformanceRisk` | interface | volatility?, maxDrawdown?, peakDate?, troughDate?, recoveryDate? | A | 低 | 低 |
| `PerformanceDataQuality` | interface | status ("ok"|"partial"|"noData"|"notApplicable"), warnings?, notApplicableReasons? | A | 低 | 低 |
| `ReturnData` | interface | date (string), value (number) | A | 低 | 低 |
| `IncomeSummary` | interface | period, byMonth, byType, byAsset, byCurrency, byAccount, totalIncome, currency, monthlyAverage, yoyGrowth | A | 低 | 低 |

#### 目标与退休规划域

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `Goal` | interface | id, goalType, title, targetAmount?, statusLifecycle, statusHealth, priority, currency?, startDate?, targetDate?, summaryCurrentValue?, summaryProgress? | A | 低 | 低 |
| `NewGoal` | interface | goalType, title, targetAmount?, statusLifecycle?, priority?, currency? | B | 低 | 低 |
| `GoalFundingRule` | interface | id, goalId, accountId, sharePercent, taxBucket? | B | 低 | 低 |
| `GoalPlan` | interface | goalId, planKind, settingsJson, summaryJson, version | B | 低 | 低 |
| `RetirementOverview` | interface | analysisMode, status, successStatus, desiredFireAge, fiAge, portfolioNow, portfolioAtRetirementStart, netFireTarget, grossFireTarget, ...（30+ 字段） | A | 低 | 低 |
| `RetirementTrajectoryPoint` | interface | age, year, phase, portfolioStart, annualContribution, annualExpenses, portfolioEnd, ... | A | 低 | 低 |
| `BudgetBreakdown` | interface | totalMonthlyBudget, monthlyPortfolioWithdrawal, incomeStreams (BudgetStreamItem[]) | 组合 | A | 低 | 低 |
| `SaveUpOverviewDTO` | interface | currentValue, targetAmount, progress, health, projectedValueAtTargetDate, requiredMonthlyContribution | A | 低 | 低 |

#### 导入与模板域

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `ImportTemplateData` | interface | id, name, scope, kind, fieldMappings, activityMappings, symbolMappings, symbolMappingMeta, parseConfig? | A | 低 | 低 |
| `BrokerSyncProfileData` | interface | id, name, scope, sourceSystem, activityMappings, symbolMappings, symbolMappingMeta | A | 低 | 低 |
| `CsvRowData` | type alias | `Record<string, string> & { lineNumber: string }` | C | 低 | 低 |
| `ParsedCsvResult` | interface | headers, rows, detectedConfig, errors, rowCount | B | 低 | 低 |
| `ImportActivitiesResult` | interface | activities (ActivityImport[]), importRunId, summary (ImportActivitiesSummary) | 组合 | A | 低 | 低 |
| `ImportRun` | interface | id, accountId, sourceSystem, runType, mode, status, startedAt, finishedAt?, summary? | A | 低 | 低 |

#### 分类学与分配域

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `Taxonomy` | interface | id, name, color, isSystem, isSingleSelect, sortOrder, scope | A | 低 | 低 |
| `TaxonomyCategory` | interface | id, taxonomyId, parentId?, name, key, color, sortOrder, icon? | A | 低 | 低 |
| `TaxonomyWithCategories` | interface | taxonomy (Taxonomy), categories (TaxonomyCategory[]) | 组合 | A | 低 | 低 |
| `AssetTaxonomyAssignment` | interface | id, assetId, taxonomyId, categoryId, weight, source | A | 低 | 低 |
| `AssetClassifications` | interface | assetType?, riskCategory?, assetClasses, sectors, regions, customGroups | A | 低 | 低 |
| `AllocationTarget` | interface | id, name, scopeType, scopeId?, taxonomyId, triggerType, driftBandBps, bandType, ... | A | 低 | 低 |
| `DriftReport` | interface | targetId, scopeType, totalValue, baseCurrency, maxDriftBps, outOfBandCount, rows (DriftRow[]) | 组合 | A | 低 | 低 |
| `RebalancePlan` | interface | targetId, availableCash, trades (SuggestedManualTrade[]), warnings (RebalanceWarning[]) | 组合 | A | 低 | 低 |
| `PortfolioAllocations` | interface | assetClasses, sectors, regions, riskCategory, securityTypes, customGroups, totalValue | A | 低 | 低 |

#### 健康中心域

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `HealthIssue` | interface | id, severity, category, title, message, affectedCount, fixAction?, navigateAction?, diagnostics? | A | 低 | 低 |
| `HealthDiagnostic` | interface | fingerprint, domain, level, severity, code, title, explanation, impact?, entities, evidence, actions | A | 低 | 低 |
| `HealthStatus` | interface | overallSeverity, issueCounts, issues (HealthIssue[]), checkedAt, isStale | 组合 | A | 低 | 低 |
| `HealthConfig` | interface | stalePriceWarningDays, stalePriceErrorDays, criticalMvThresholdPercent, enabled | B | 低 | 低 |

#### 净值与替代资产域

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `NetWorthResponse` | interface | date, assets (AssetsSection), liabilities (LiabilitiesSection), netWorth, currency, staleAssets | 组合 | A | 低 | 低 |
| `NetWorthHistoryPoint` | interface | date, portfolioValue, alternativeAssetsValue, totalLiabilities, totalAssets, netWorth, netContribution, breakdown | A | 低 | 低 |
| `AlternativeAssetHolding` | interface | id, kind, name, symbol, currency, marketValue, purchasePrice?, valuationDate, metadata? | A | 低 | 低 |
| `CreateAlternativeAssetRequest` | interface | kind, name, currency, currentValue, valueDate, purchasePrice?, metadata? | A | 低 | 低 |
| `BreakdownItem` | interface | category, name, value, assetId?, children? (BreakdownItem[]) | 递归 | A | 低 | 低 |

#### AI 提供商域

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `MergedProvider` | interface | id, name, type, icon, description, connectionFields, models, defaultModel, enabled, favorite, selectedModel?, customUrl?, hasApiKey, ...（20+ 字段） | A | 低 | 低 |
| `MergedModel` | interface | id, name?, capabilities (ModelCapabilities), isCatalog, isFavorite, hasCapabilityOverrides | A | 低 | 低 |
| `ModelCapabilities` | interface | tools, thinking, vision, streaming | A | 低 | 低 |
| `ProviderTuning` | interface | temperature?, maxTokens?, maxTokensThinking?, extraOptions? | A | 低 | 低 |
| `AiProvidersResponse` | interface | providers (MergedProvider[]), capabilities, defaultProvider? | 组合 | A | 低 | 低 |

#### 设置与杂项

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `Settings` | interface | theme, font, language, baseCurrency, defaultReturnMetric, timezone, onboardingCompleted, autoUpdateCheckEnabled, syncEnabled | A | 低 | 低 |
| `ContributionLimit` | interface | id, groupName, contributionYear, limitAmount, accountIds? | A | 低 | 低 |
| `Platform` | interface | id, name, url, externalId, logoUrl? | B | 低 | 低 |
| `DateRange` | interface | from (Date|undefined), to (Date|undefined) | A | 低 | 低 |
| `TimePeriod` | type alias | "1D"|"1W"|"1M"|"3M"|"6M"|"YTD"|"1Y"|"5Y"|"ALL" | A | 低 | 低 |
| `TrackingMode` | type alias | "TRANSACTIONS"|"HOLDINGS"|"NOT_SET" | A | 低 | 低 |
| `SnapshotInfo` | interface | id, snapshotDate, isDateValid, source, positionCount, cashCurrencyCount | A | 低 | 低 |

**Zod 派生类型：**
- `ActivityImport` = `z.infer<typeof importActivitySchema>`
- `ImportMappingData` = `z.infer<typeof importMappingSchema>`
- `ParseConfig` = `z.infer<typeof parseConfigSchema>`

**模式总结：** `lib/types.ts` 是整个前端的数据类型中枢，约 2718 行，定义了约 100+ 个类型/接口。大部分接口采用纯数据 DTO 模式，无方法。关键模式：
- 金额使用 `number` 类型（但后端使用 decimal string，需注意精度丢失风险）
- 嵌套类型通过组合（如 `PerformanceResult` 包含 `PerformanceReturns`、`PerformanceAttribution` 等子接口）
- 部分类型存在遗留版本（如 `ActivityLegacy` vs `Activity`）
- 枚举类型从 `constants.ts` 导入，接口使用 `AccountType` 等类型而非字符串字面量

---
### L4 适配器层

适配器层负责前端与后端（Tauri IPC / Axum HTTP）之间的通信桥接，分为类型定义、共享命令封装和平台特定实现三层。

#### 4.1 适配器基础类型

来源：`F:\dev\wealthfolio\apps\frontend\src\adapters\types.ts`（470 行）

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `RunEnv` | type alias | `"desktop"` \| `"web"` | A | 低 | 低 |
| `EventCallback<T>` | type alias | `(event: { event: string; payload: T; id: number }) => void` | A | 低 | 低 |
| `UnlistenFn` | type alias | `() => Promise<void>` | A | 低 | 低 |
| `Logger` | interface | error, warn, info, debug, trace (各为 `(...args: unknown[]) => void`) | A | 低 | 低 |
| `DataExportResult` | interface | status ("saved"\|"empty"\|"canceled"), filename? | B | 低 | 低 |
| `ImportRunsRequest` | interface | accountId, sourceSystem?, runType?, mode?, status?, ... | B | 低 | 低 |
| `AppInfo` | interface | name, version, platform, arch, ... | B | 低 | 低 |
| `PlatformCapabilities` | interface | 平台能力标记（桌面/Web 特定功能） | B | 低 | 低 |
| `PlatformInfo` | interface | platform (desktop\|web), version, ... | B | 低 | 低 |

**后端同步状态类型（Backend Sync）：**
- `BackendSyncStateResult` — 同步状态整体描述
- `BackendSyncEngineStatusResult` — 同步引擎健康状态
- `BackendSyncBootstrapResult` — 同步引导结果
- `BackendSyncCycleResult` — 同步周期结果
- `BackendSyncSnapshotUploadResult` — 快照上传结果
- `BackendSyncPairingSourceStatusResult` — 配对源状态

**Agent 访问类型：**
- `AgentAccessStatus` — Agent 访问状态
- `AgentAccessToken` — Agent 访问令牌
- `CreateAgentAccessTokenInput` — 创建令牌输入
- `CreatedAgentAccessToken` — 创建令牌结果
- `AgentAuditEntry` — 审计条目
- `AgentAuditPage` — 审计分页
- `McpServerStatus` — MCP 服务器状态

**Addon 类型：**
- `AddonFile`, `AddonAsset`, `ExtractedAddon`, `InstalledAddon` — 安装包相关
- `AddonNetworkRequest/Response` — 网络请求/响应

#### 4.2 共享命令封装

来源：`F:\dev\wealthfolio\apps\frontend\src\adapters\shared/*.ts`（13 个文件）

每个文件封装一个功能域的 backend 调用，遵循统一的 `invoke<T>(command_name, args)` 模式。函数签名模式：

```typescript
export const getXxx = async (params): Promise<ResponseType> => {
  return await invoke<ResponseType>("backend_command", { args });
};
```

| 文件 | 功能域 | 命令函数数量 | 关键函数 | 复用性 | 迁移难度 | 代码风险 |
|------|-------|------------|---------|-------|---------|---------|
| `accounts.ts` | 账户管理 | 4 | getAccounts, createAccount, updateAccount, deleteAccount | A | 低 | 低 |
| `activities.ts` | 活动/交易 | 20+ | searchActivities, createActivity, updateActivity, saveActivities, importActivities, checkActivitiesImport, listImportTemplates, ... | A | 低 | 低 |
| `goals.ts` | 目标管理 | 5+ | getGoals, createGoal, updateGoal, deleteGoal, getGoalPlan, saveGoalPlan | A | 低 | 低 |
| `allocation-targets.ts` | 分配目标 | 5+ | getAllocationTargets, createAllocationTarget, getDriftReport, getRebalancePlan | A | 低 | 低 |
| `portfolio.ts` | 投资组合 | 5+ | getHoldings, getPerformance, getCurrentValuation | A | 低 | 低 |
| `portfolios.ts` | 投资组合管理 | 4 | getPortfolios, createPortfolio, updatePortfolio, deletePortfolio | A | 低 | 低 |
| `market-data.ts` | 市场数据 | 5+ | searchSymbols, getQuotes, getLatestQuoteSnapshot | A | 低 | 低 |
| `taxonomies.ts` | 分类学 | 5+ | getTaxonomies, createTaxonomy, assignCategories | A | 低 | 低 |
| `exchange-rates.ts` | 汇率 | 3 | getExchangeRates, updateExchangeRate | A | 低 | 低 |
| `alternative-assets.ts` | 替代资产 | 5+ | createAlternativeAsset, updateValuation, getNetWorth | A | 低 | 低 |
| `health.ts` | 健康中心 | 2 | getHealthStatus, runHealthCheck | A | 低 | 低 |
| `connect.ts` | Wealthfolio Connect | 5+ | getConnections, startPairing, confirmPairing | A | 低 | 低 |
| `contribution-limits.ts` | 缴款限额 | 4 | getContributionLimits, createContributionLimit, update, delete | A | 低 | 低 |

**局部类型定义：**
- `activities.ts`: `ActivityFilters`（accountIds, activityTypes, symbol, needsReview, dateFrom/To, instrumentTypes, activityIds）、`ActivitySort`（id, desc?）
- `connect.ts`: `PairingFlowPhase`（discriminated union）、`PairingFlowResponse`、`ConfirmPairingWithBootstrapResult`
- `portfolio.ts`: `HoldingInput`

#### 4.3 平台特定适配器

来源：`F:\dev\wealthfolio\apps\frontend\src\adapters\tauri/` 和 `web/`

| 文件 | 说明 | 复用性 | 迁移难度 | 代码风险 |
|------|------|-------|---------|---------|
| `tauri/ai-streaming.ts` | Tauri Channel 实现的 AI 流式聊天（`streamAiChat` async generator） | A | 低 | 低 |
| `tauri/core.ts` | Tauri IPC invoke 封装 | A | 低 | 低 |
| `web/*.ts` | Axum HTTP 适配器 | A | 中 | 低 |

**模式总结：** 适配器层通过 `adapters/shared/*.ts` 提供统一的命令封装，内部通过 `platform.ts` 的 `invoke()` 函数进行运行时检测（desktop 使用 Tauri IPC，web 使用 HTTP）。这种模式使前端组件无需关心后端通信机制。

---

### L5 功能模块类型层

功能模块位于 `apps/frontend/src/features/`，每个模块是自包含的功能单元。

#### 5.1 AI 助手

来源：`F:\dev\wealthfolio\apps\frontend\src\features\ai-assistant\types.ts`（891 行）

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `AiThreadConfig` | interface | 线程配置（systemPrompt?, temperature, ...） | A | 低 | 低 |
| `ChatThread` | interface | id, title, createdAt, updatedAt, messageCount | A | 低 | 低 |
| `ChatMessage` | interface | id, role, content (ChatMessageContent), createdAt | A | 低 | 低 |
| `ChatMessagePart` | 联合类型（6 变体） | text / toolCall / toolResult / image / reasoning / error | A | 低 | 低 |
| `AiStreamEvent` | 联合类型（8 变体） | system / textDelta / reasoningDelta / toolCall / toolResult / error / done / threadTitleUpdated | A | 低 | 低 |
| `RecordActivitiesIntent` | interface | AI 记录活动意图描述 | B | 低 | 低 |
| `ImportCsvArgs` | interface | CSV 导入参数 | B | 低 | 低 |
| `ImportCsvMappingOutput` | interface | 映射输出 | B | 低 | 低 |
| `ProposeCategoriesArgs` | interface | 分类建议参数 | B | 低 | 低 |
| `ChatState` | type alias | UI 聊天状态枚举 | A | 低 | 低 |
| `ChatError` | interface | 错误信息（code, message, ...） | A | 低 | 低 |

#### 5.2 设备同步

来源：`F:\dev\wealthfolio\apps\frontend\src\features\devices-sync\types.ts`（461 行）

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `DeviceSyncState` | type alias | 状态机（notEnrolled / enrolling / ready / ...） | A | 低 | 低 |
| `SyncIdentity` | interface | 设备身份（deviceId, publicKey, ...） | A | 低 | 低 |
| `Device` | interface | 设备信息（id, name, platform, lastSeenAt） | A | 低 | 低 |
| `TrustedDeviceSummary` | interface | 受信任设备摘要 | A | 低 | 低 |
| `EnrollDeviceResponse` | 联合类型 | success / error 变体 | A | 低 | 低 |
| `PairingSession` | interface | 配对会话（channel, topic, ...） | B | 中 | 中 |
| `ClaimerSession` | interface | 认领者会话 | B | 中 | 中 |
| `SyncError` | class extends Error | code (SyncErrorCodes), static 工厂方法（fromCode, ...） | A | 低 | 低 |

#### 5.3 退休规划器

来源：`F:\dev\wealthfolio\apps\frontend\src\features\goals\retirement-planner\types.ts`（240 行）

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `GlidepathSettings` | interface | 滑道设置（equityAllocation, ...） | A | 低 | 低 |
| `FireProjection` | interface | FIRÉ 预测结果 | A | 低 | 低 |
| `YearlySnapshot` | interface | 年度快照 | A | 低 | 低 |
| `MonteCarloResult` | interface | 蒙特卡洛模拟结果 | A | 低 | 低 |
| `ScenarioResult` | interface | 场景结果 | A | 低 | 低 |
| `SorrScenario` | interface | SORR 场景 | A | 低 | 低 |
| `DecisionSensitivityMatrix` | interface | 决策敏感性矩阵 | B | 低 | 低 |
| `StressTestResult` | interface | 压力测试结果 | A | 低 | 低 |
| `RetirementPlan` | interface | 版本 "v3" 的完整退休计划 | A | 低 | 低 |
| `PersonalProfile` | interface | 个人资料 | A | 低 | 低 |
| `ExpenseBudget` | interface | 支出预算 | A | 低 | 低 |
| `TaxProfile` | interface | 税务配置 | A | 低 | 低 |

#### 5.4 Wealthfolio Connect

来源：`F:\dev\wealthfolio\apps\frontend\src\features\wealthfolio-connect\types.ts`（241 行）

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `BrokerConnection` | interface | id, brokerId, name, status, createdAt, linkedAt | A | 低 | 低 |
| `BrokerAccount` | interface | id, connectionId, accountId, name, ... | A | 低 | 低 |
| `BrokerAccountSyncStatus` | interface | 同步状态 | A | 低 | 低 |
| `SubscriptionPlan` | interface | 订阅计划 | A | 低 | 低 |
| `PlanPricing` | interface | 定价 | A | 低 | 低 |
| `SyncResult` | interface | 同步结果 | A | 低 | 低 |
| `UserInfo` | interface | 用户信息 | A | 低 | 低 |
| `ImportRun` | interface | 导入运行记录 | A | 低 | 低 |
| `AggregatedSyncStatus` | interface | 汇总同步状态 | A | 低 | 低 |

#### 5.5 支出管理

来源：`F:\dev\wealthfolio\apps\frontend\src\features\spending\types/`（7 个文件）

**5.5.1 `index.ts` — 支出核心类型：**
- `SpendingSettings`, `SpendingSettingsUpdate`, `SpendingPeriod`, `CategorySpending`, `SubcategorySpending`, `SpendingSummary`

**5.5.2 `rule.ts` — 分类规则：**
- `CategorizationRule` (interface, 规则定义), `NewCategorizationRule`, `UpdateCategorizationRule`, `RulePresetSummary`, `RuleMatchType`

**5.5.3 `budget.ts` — 预算：**
- `BudgetGroup` (interface), `BudgetTarget` (discriminated union), `BudgetRolloverSetting`, `BudgetCategoryRow`, `BudgetSnapshot`, `BudgetTargetType`, `BudgetRolloverTargetType`

**5.5.4 `cash-activity.ts` — 现金活动：**
- `CashActivity` (interface extends Activity, 增加现金特有字段), `CashActivityFilter`, `CashActivitySearchRequest/Response`, `ActivityTaxonomyAssignment`, `ActivitySplit`, `CashFlowBucket`, `CashActivitySortField`

**5.5.5 `insight.ts` — 消费洞察：**
- `SpendingInsightRequest`, `SpendingInsight`, `Headline`, `PaceState`, `HealthStatus`, `CategoryInsight`, `GroupInsight`, `UncategorizedBucket`, `DayBucket`, `MonthBucket`, `PeriodMeta`

**5.5.6 `report.ts` — 报表：**
- `PeriodSummary`, `CategoryBreakdownRow`, `DayBucket`, `MonthlyReport`, `MonthBucket`, `ReportRequest`

**5.5.7 `event.ts` — 事件：**
- `EventType`, `NewEventType`, `SpendingEvent`, `NewSpendingEvent`, `UpdateSpendingEvent`, `EventWithTypeName`, `EventSummariesRequest`

---

### L6 共享包类型层

共享包位于 `packages/`，提供前端各应用复用。

#### 6.1 Addon SDK

**来源：`F:\dev\wealthfolio\packages\addon-sdk\src\types.ts`（176 行）**

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `SidebarItemConfig` | interface | 侧边栏项配置 | A | 低 | 低 |
| `AddonRouteComponent` | type alias | 路由组件（React ComponentType） | A | 低 | 低 |
| `AddonContext` | interface | Addon 运行时上下文 | A | 低 | 低 |
| `RouteConfig` | interface | 路由配置 | A | 低 | 低 |

**来源：`F:\dev\wealthfolio\packages\addon-sdk\src\data-types.ts`（1114 行）**

镜像主应用的核心领域类型（Account, Activity, Holding, Asset, Quote 等），共约 20+ 类型，其枚举常量（ActivityType 14 值、ActivityStatus、ACTIVITY_SUBTYPES 29 值、AssetKind、QuoteMode、DataSource、AccountType、HoldingType）与 `lib/constants.ts` 保持一致。

| 类型名称 | 来源主应用对应 | 复用性 | 迁移难度 | 代码风险 |
|---------|--------------|-------|---------|---------|
| `Account` | lib/types.ts Account | A | 低 | 低 |
| `Activity` | lib/types.ts Activity | A | 低 | 低 |
| `Holding` | lib/types.ts Holding | A | 低 | 低 |
| `Asset` | lib/types.ts Asset | A | 低 | 低 |
| `Quote` | lib/types.ts Quote | A | 低 | 低 |

**来源：`F:\dev\wealthfolio\packages\addon-sdk\src\manifest.ts`（290 行）**

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `AddonManifest` | interface | id, name, version, contributes, networkAccess, hostDependencies | A | 低 | 低 |
| `AddonContributes` | interface | routes?, links? | A | 低 | 低 |
| `AddonNetworkAccess` | interface | 网络访问配置 | A | 低 | 低 |
| `AddonInstallResult` | interface | 安装结果 | B | 低 | 低 |
| `AddonValidationResult` | interface | 校验结果 | B | 低 | 低 |
| `DevelopmentManifest` | type alias | 开发模式 Manifest（id, development 配置） | B | 低 | 低 |

**来源：`F:\dev\wealthfolio\packages\addon-sdk\src\permissions.ts`（361 行）**

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `Permission` | interface | id, name, description, risk | A | 低 | 低 |
| `FunctionPermission` | type alias | 函数级权限（`{api, method}`） | A | 低 | 低 |
| `PermissionCategory` | interface | 权限类别 | A | 低 | 低 |
| `RiskLevel` | type alias | "low"\|"medium"\|"high"\|"critical" | A | 低 | 低 |

**来源：`F:\dev\wealthfolio\packages\addon-sdk\src\host-api.ts`（887 行）**

| 子 API | 说明 | 复用性 | 迁移难度 | 代码风险 |
|-------|------|-------|---------|---------|
| `AccountsAPI` | 账户操作 | A | 低 | 低 |
| `PortfolioAPI` | 投资组合操作 | A | 低 | 低 |
| `ActivitiesAPI` | 活动操作 | A | 低 | 低 |
| `MarketDataAPI` | 行情查询 | A | 低 | 低 |
| `AssetsAPI` | 资产操作 | A | 低 | 低 |
| `QuotesAPI` | 报价查询 | A | 低 | 低 |
| `PerformanceAPI` | 绩效查询 | A | 低 | 低 |
| `ExchangeRatesAPI` | 汇率查询 | A | 低 | 低 |
| `SettingsAPI` | 设置读取 | A | 低 | 低 |
| `GoalsAPI` | 目标操作 | A | 低 | 低 |

#### 6.2 UI 包

**来源：`F:\dev\wealthfolio\packages\ui\src\components\ui\chart.tsx`（329 行）**

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| 

| `ChartConfig` | interface | 图表配置（可包含任意数据系列键） | A | 低 | 低 |

**来源：`F:\dev\wealthfolio\packages\ui\src\components\data-grid\data-grid-types.ts`（330 行）**

| 类型名称 | 类型 | 关键字段 | 复用性 | 迁移难度 | 代码风险 |
|---------|------|---------|-------|---------|---------|
| `CellOpts` | 联合类型（12 变体） | short-text / long-text / number / select / multi-select / checkbox / date / date-input / datetime / url / file / symbol / currency | A | 低 | 低 |
| `CellPosition` | interface | rowIndex, columnId | A | 低 | 低 |
| `SelectionState` | interface | selectedCells (Set<string>), selectionRange, isSelecting | A | 低 | 低 |
| `FilterOperator` | 联合类型 | 文本/数字/日期/选择/布尔 5 类操作符 | A | 低 | 低 |
| `FilterValue` | interface | operator (FilterOperator), value?, endValue? | A | 低 | 低 |
| `NavigationDirection` | type alias | 14 种方向（up/down/left/right/home/end/pageup/...） | A | 低 | 低 |

**来源：`F:\dev\wealthfolio\packages\ui\src\lib\constants.ts`（2 行）**

| 常量 | 值 | 说明 |
|------|----|------|
| `DECIMAL_PRECISION` | 8 | 小数精度（与 Rust 后端一致） |
| `DISPLAY_DECIMAL_PRECISION` | 2 | 显示精度 |

---

## 综合评分汇总

### 各模块评分总表

| 层级 | 模块 | 文件 | 复用性 | 迁移难度 | 代码风险 |
|------|------|------|-------|---------|---------|
| L1 | Zod 校验层 | `lib/schemas.ts` | A | 低 | 低 |
| L2 | 常量枚举层 | `lib/constants.ts` | A | 低 | 低 |
| L3 | 核心业务类型层 | `lib/types.ts` | A | 低 | 低 |
| L3 | 核心类型子模块 | `lib/types/custom-provider.ts` | B | 低 | 低 |
| L3 | 核心类型子模块 | `lib/types/quote-import.ts` | B | 低 | 低 |
| L4 | 适配器基础类型 | `adapters/types.ts` | A | 低 | 低 |
| L4 | 共享命令封装 | `adapters/shared/*.ts` | A | 低 | 低 |
| L4 | Tauri 流式适配器 | `adapters/tauri/ai-streaming.ts` | A | 低 | 低 |
| L5 | AI 助手 | `features/ai-assistant/types.ts` | A | 低 | 低 |
| L5 | 设备同步 | `features/devices-sync/types.ts` | A | 中 | 中 |
| L5 | 退休规划器 | `features/goals/retirement-planner/types.ts` | A | 低 | 低 |
| L5 | Wealthfolio Connect | `features/wealthfolio-connect/types.ts` | A | 低 | 低 |
| L5 | 支出管理 | `features/spending/types/`（7 文件） | A | 低 | 低 |
| L6 | Addon SDK 类型 | `packages/addon-sdk/src/types.ts` | A | 低 | 低 |
| L6 | Addon SDK 数据类型 | `packages/addon-sdk/src/data-types.ts` | A | 低 | 低 |
| L6 | Addon SDK Manifest | `packages/addon-sdk/src/manifest.ts` | A | 低 | 低 |
| L6 | Addon SDK 权限 | `packages/addon-sdk/src/permissions.ts` | A | 低 | 低 |
| L6 | Addon SDK Host API | `packages/addon-sdk/src/host-api.ts` | A | 低 | 低 |
| L6 | UI 图表 | `packages/ui/src/components/ui/chart.tsx` | A | 低 | 低 |
| L6 | UI 数据表格 | `packages/ui/src/components/data-grid/data-grid-types.ts` | A | 低 | 低 |
| L6 | UI 常量 | `packages/ui/src/lib/constants.ts` | A | 低 | 低 |

### 评分分布

- **复用性 A 级**：20 个模块（95%）—— 绝大多数类型定义高度可复用
- **复用性 B 级**：3 个模块（5%）—— custom-provider, quote-import, NewPortfolio 等边缘类型
- **复用性 C 级**：0 个模块
- **迁移难度 低**：22 个模块（100%）
- **代码风险 低**：22 个模块（95%），仅 devices-sync 模块因端到端加密复杂性评为中风险

---

## 关键观察

### 架构特征

1. **6 层类型架构清晰**：从 Zod 模式定义到常量枚举，再到核心业务类型、适配器层、功能模块层和共享包层，层次分明，职责单一。

2. **`lib/types.ts` 是核心中枢**：2718 行，约 100+ 个类型定义，是前端最重的文件。建议在类型数量继续增长时按域拆分（如 `types/account.ts`、`types/activity.ts`、`types/performance.ts`）。

3. **Zod 模式与 TypeScript 类型双轨制**：校验逻辑在 `schemas.ts`（Zod），业务类型在 `lib/types.ts`，部分通过 `z.infer` 桥接。这种模式清晰但需要维护两个来源的同步。

### 类型安全风险

4. **金额精度问题**：`lib/types.ts` 中大部分金额字段使用 `number` 类型（如 `MonetaryValue.local/base: number`），但后端使用 decimal string。前端在显示和传输间存在精度丢失风险。`Holding.price`、`Position.averageCost` 等字段均为 `number`。

5. **遗留类型**：`ActivityLegacy` 标记为 `@deprecated` 但仍存在于代码库中，依赖它的代码需要迁移到新的 `Activity` 接口。

### 代码质量特征

6. **枚举模式一致性**：所有枚举使用 `as const` 对象 + 同名类型别名的模式，一致性良好。

7. **适配器模式成熟**：`adapters/shared/*.ts` 通过 `invoke<T>()` 统一封装跨平台调用，前端组件无需关注 Tauri IPC 或 HTTP 细节。

8. **Addon SDK 类型完整**：`packages/addon-sdk/` 提供了完整的类型定义，包括 Manifest、权限、Host API 和领域数据类型，支持第三方扩展开发。

9. **功能模块自包含**：各 feature 都有自己的 `types.ts`，不依赖其他 feature 的内部类型，模块间通过 `lib/types.ts` 共享。

### 改进建议

10. **`lib/types.ts` 拆分**：建议按域拆分，每个文件控制在 500 行以内，以 2000+ 行单文件维护成本较高。

11. **`number` 到 `string` 迁移**：考虑对金额字段逐步引入 `type MonetaryAmount = string` 类型别名，在编译层面提供精度保护。

12. **`data-types.ts` 同步**：`packages/addon-sdk/src/data-types.ts` 镜像了 `lib/types.ts` 的类型，需要保持两者同步，建议通过自动化生成或共享子包来避免 drift。

## Rust 域模型层

本部分由仓库扫描代理生成，覆盖 Wealthfolio 的 Rust 域模型层（crates/core 及子 crate），按批次记录主要模型、枚举、trait 及其文件位置。


### 批次 1 — 账户 (accounts) 与活动 (activities) 模型

#### accounts 模块概览

文件清单（`F:\dev\wealthfolio\crates\core\src\accounts\`）：
- `accounts_model.rs` — 领域模型与输入/更新模型
- `accounts_traits.rs` — Repository/Service trait 契约
- `accounts_service.rs` — 服务实现
- `accounts_constants.rs` — 账户类型常量
- `accounts_model_tests.rs` / `mod.rs` — 测试与模块声明

##### accounts::TrackingMode (F:\dev\wealthfolio\crates\core\src\accounts\accounts_model.rs:13)

| 变体 | 说明 |
|------|------|
| Transactions | 持仓由交易历史计算 |
| Holdings | 持仓手动录入/直接导入 |
| NotSet (default) | 未设置 |

Derives: Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default; serde 用 SCREAMING_SNAKE_CASE。

##### accounts::CostBasisMethod (F:\dev\wealthfolio\crates\core\src\accounts\accounts_model.rs:26)

| 变体 | 说明 |
|------|------|
| Fifo (default) | 先进先出 |
| Lifo | 后进先出 |
| Wac | 加权平均成本 |

方法：`as_str()`、`from_code(&str) -> Result<Self>`、`ensure_supported_for_calculation(account_id) -> Result<()>`（目前快照计算器仅支持 FIFO）。实现 `Display`。

##### accounts::CostBasisProfile (accounts_model.rs:76)

变体：Generic (default)、CanadaAcb（加拿大 ACB 口径）。方法 `as_str()` / `from_code()`。

##### accounts::PoolingScope (accounts_model.rs:105)

变体：Account (default)、Portfolio。决定批次合并范围。

##### accounts::LotSelectionStrategy (accounts_model.rs:134)

变体：SpecificId、HighestCost、LowestCost（无 Default，Option 包装）。

##### accounts::AccountAccountingSettings (accounts_model.rs:165)

| 字段 | 类型 | Notes |
|------|------|-------|
| account_id | String | |
| cost_basis_method | CostBasisMethod | 默认 FIFO |
| cost_basis_profile | CostBasisProfile | 默认 GENERIC |
| pooling_scope | PoolingScope | 默认 ACCOUNT |
| lot_selection_strategy | Option<LotSelectionStrategy> | |
| settings_json | String | 扩展 JSON |
| created_at / updated_at | String | ISO 时间戳 |

方法：`default_for_account(account_id)`、`ensure_supported_for_calculation()`。Derives: Debug, Clone, PartialEq, Eq, Serialize, Deserialize (camelCase)。

##### accounts::Account (accounts_model.rs:228)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | UUID |
| name | String | |
| account_type | String | 字符串而非枚举（见 accounts_constants） |
| group | Option<String> | |
| currency | String | |
| is_default | bool | |
| is_active | bool | |
| created_at | NaiveDateTime | |
| updated_at | NaiveDateTime | |
| platform_id | Option<String> | |
| account_number | Option<String> | 券商账户号 |
| meta | Option<String> | JSON 元数据 |
| provider | Option<String> | 如 SNAPTRADE/PLAID/MANUAL |
| provider_account_id | Option<String> | 提供商侧 ID |
| is_archived | bool | |
| tracking_mode | TrackingMode | |

方法：`cash_allocation_category_id() -> Option<String>`（解析 meta JSON 中 allocation.cashCategoryId）。Derives: Debug, Clone, Serialize, Deserialize, Default (camelCase)。

##### accounts::NewAccount (accounts_model.rs:272) / accounts::AccountUpdate (accounts_model.rs:319)

NewAccount 字段：id(Option)、name、account_type、group、currency、is_default、is_active、platform_id、account_number、meta、provider、provider_account_id、is_archived(default false)、tracking_mode(default)。`validate()` 校验名称/货币非空，且信用卡账户不允许 HOLDINGS 模式。
AccountUpdate 字段：id(Option, 必填)、name、account_type、group、is_default、is_active、platform_id、account_number、meta、provider、provider_account_id、is_archived(Option)、tracking_mode(Option)。`validate()` 同上。

##### accounts::AccountRepositoryTrait (accounts_traits.rs:18) — trait

关键方法（async）：`create(NewAccount) -> Result<Account>`、`update(AccountUpdate) -> Result<Account>`、`delete(&str) -> Result<usize>`、`get_by_id(&str) -> Result<Account>`、`list(Option<bool>, Option<bool>, Option<&[String]>) -> Result<Vec<Account>>`、`get_accounting_settings_by_account_ids(&[String]) -> Result<HashMap<String, AccountAccountingSettings>>`（默认实现返回 FIFO/GENERIC/ACCOUNT）。实现者：storage-sqlite 中的 repository（见批次 6）。

##### accounts::AccountServiceTrait (accounts_traits.rs:73) — trait

方法：`create_account`、`update_account`、`delete_account`、`get_account`、`list_accounts`、`get_all_accounts`、`get_active_accounts`、`get_accounts_by_ids`、`get_non_archived_accounts`、`get_active_non_archived_accounts`、`get_base_currency -> Option<String>`。实现者：core 中的 AccountService。

#### activities 模块概览

文件清单（`F:\dev\wealthfolio\crates\core\src\activities\`）：
- `activities_model.rs` — 核心活动模型（本批次重点）
- `activities_constants.rs` — 活动类型/子类型常量与分类逻辑
- `activities_errors.rs` — ActivityError
- `activities_traits.rs` — Repository/Service trait
- `activities_service.rs`、`compiler.rs`、`csv_parser.rs`、`idempotency.rs`、`import_run_model.rs`、`transfer_pairs.rs` — 服务与配套逻辑

##### activities::ActivityStatus (activities_model.rs:137)

| 变体 | 说明 |
|------|------|
| Posted (default) | 生效，参与计算 |
| Pending | 待结算/确认 |
| Draft | 用户创建未确认 |
| Void | 取消/冲销（软删除） |

Derives: Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default (SCREAMING_SNAKE_CASE)。

##### activities::Activity (activities_model.rs:148) — 核心交易模型

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | |
| account_id | String | |
| asset_id | Option<String> | 纯现金活动可为 NULL |
| activity_type | String | 规范类型（14 种闭集，见常量） |
| activity_type_override | Option<String> | 用户覆盖，同步不触碰 |
| source_type | Option<String> | 原始提供商标签 |
| subtype | Option<String> | 语义变体（DRIP 等） |
| status | ActivityStatus | |
| activity_date | DateTime<Utc> | 自定义 timestamp_format |
| settlement_date | Option<DateTime<Utc>> | |
| quantity / unit_price / amount / fee / tax | Option<Decimal> | 全部可选，字符串序列化 |
| currency | String | |
| fx_rate | Option<Decimal> | |
| notes | Option<String> | |
| metadata | Option<Value> | JSON blob |
| source_system | Option<String> | SNAPTRADE/PLAID/MANUAL/CSV |
| source_record_id / source_group_id | Option<String> | 提供商标识 |
| idempotency_key | Option<String> | 去重稳定哈希 |
| import_run_id | Option<String> | 导入批次 |
| is_user_modified | bool | 同步保护经济字段 |
| needs_review | bool | 需用户复核 |
| created_at / updated_at | DateTime<Utc> | |

方法：`effective_type()`（尊重 override）、`effective_date()`、`is_posted()`、`has_override()`、`qty()/price()/amt()/fee_amt()/tax_amt()`（取绝对值）、`charge_amt_for(&ActivityType)`、`get_meta<T>(key)`。
Derives: Debug, Clone, Serialize, Deserialize (camelCase)。

##### activities::ActivityType (activities_model.rs:1514) — 枚举

变体：Buy、Sell、Dividend、Interest、Deposit、Withdrawal、TransferIn、TransferOut、Fee、Tax、Split、Credit（纯现金credit）、Adjustment（非交易修正）、Unknown。实现 `FromStr`（字符串常量 ↔ 枚举）与 `as_str()`。Derives: Debug, Clone, PartialEq, Eq, Hash。

##### activities 关键输入模型（activities_model.rs）

- `AssetResolutionInput` (:295)：资产识别的嵌套输入 — id、symbol、exchange_mic、kind、name、quote_mode、quote_ccy、instrument_type、provider_id、provider_symbol。
- `NewActivity` (:321)：`asset` 字段带 `#[serde(alias = "symbol")]` 兼容旧格式；含 `canonicalize_subtype()`、`canonicalize_subtype_for_activity()`、`is_asset_backed_income_subtype()`、`validate()` 等方法。
- `ActivityUpdate` (:587)：更新模型，金额字段为 `Option<Option<Decimal>>`（patch 语义）。
- `ActivityBulkMutationRequest` (:717) / `ActivityBulkMutationResult` (:729) / `ActivityBulkMutationError` (:806) / `ActivityBulkIdentifierMapping` (:815)：批量变更。
- `InternalTransferPairRequest` (:742) / `InternalTransferPairResponse` (:777) / `TransferMatchCandidateRequest` (:784) / `TransferMatchCandidate` (:794)：内部转账配对。
- `ActivityDetails` (:823)：含账户/资产关联信息的展示模型（数量为字符串，可宽容解析）。
- `ActivitySearchResponseMeta` (:913) / `ActivitySearchResponse` (:920)。
- `ActivityImport` (:928)：导入行模型（含 isin、force_import、is_external、duplicate_of_* 等导入期字段）。
- `Sort` (:1020)。
- `ImportMapping` (:1028) / `ImportMappingData` (:1064) / `ImportMappingConfig` (:1218)：导入映射（config 为 JSON blob）。
- `ImportTemplate` (:1100) / `ImportTemplateData` (:1145) / `ImportTemplateScope` (:1092 System/User) / `TemplateKind` (:76 CsvActivity/CsvHoldings/BrokerActivity)。
- `FieldMappingValue` (:1123)：untagged 枚举 Single(String) / Fallback(Vec<String>)。
- `SymbolMappingMeta` (:1044)：exchange_mic、symbol_name、quote_ccy、instrument_type、quote_mode。
- `ImportAssetCandidate` (:1167) / `ImportAssetPreviewItem` (:1199) / `

### 批次 2 — 资产 (assets) 与持仓/投资组合 (portfolio) 模型

#### assets 模块概览

文件清单（`F:\dev\wealthfolio\crates\core\src\assets\`）：
- `assets_model.rs` — 核心资产领域模型与输入
- `asset_id.rs` — 符号解析工具（parse_crypto_pair_symbol、parse_symbol_with_exchange_suffix）
- `asset_resolution.rs` — 资产解析输入/输出与 provider 别名
- `alternative_assets_model.rs` — 替代资产领域模型（房地产、车辆等）
- `assets_constants.rs` / `auto_classification.rs` / `classification_service.rs` — 分类相关
- `assets_traits.rs` / `assets_service.rs` — 仓储/服务层

##### assets::AssetKind (assets_model.rs:29)

| 变体 | 说明 |
|------|------|
| Investment (default) | 所有可交易、有批次追踪的市场工具 |
| Property | 房地产 |
| Vehicle | 车辆 |
| Collectible | 收藏品 |
| PreciousMetal | 实物贵金属 |
| PrivateEquity | 私募股权 |
| Liability | 负债（房贷、贷款等） |
| Other | 其他 |
| Fx | 汇率（基础设施，不可持仓） |

方法：`as_db_str()`、`display_name()`、`is_alternative()`、`is_investment()`、`is_liability()`、`from_db_str()`。Derives: Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default (SCREAMING_SNAKE_CASE)。

##### assets::InstrumentType (assets_model.rs:48)

变体：Equity、Crypto、Fx、Option、Metal、Bond。方法：`as_db_str()`、`from_db_str()`、`from_external_str()`（宽容解析外部标签如 STOCK/ETF/MUTUALFUND → Equity）。Derives: Debug, Clone, PartialEq, Eq, Serialize, Deserialize (SCREAMING_SNAKE_CASE)。

##### assets::QuoteMode (assets_model.rs:60)

变体：Market (default) — 市场数据提供商定价；Manual — 用户手动输入报价。Derives: Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default。

##### assets::Asset (assets_model.rs:238)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | 不透明 UUID |
| kind | AssetKind | 行为分类 |
| name | Option<String> | 显示名称 |
| display_code | Option<String> | 用户可见代码 |
| notes | Option<String> | |
| metadata | Option<Value> | JSON 元数据 |
| is_active | bool | 默认 true |
| quote_mode | QuoteMode | |
| quote_ccy | String | 报价币种 |
| instrument_type | Option<InstrumentType> | 市场工具类型 |
| instrument_symbol | Option<String> | 规范符号（AAPL/BTC/EUR） |
| instrument_exchange_mic | Option<String> | ISO 10383 MIC |
| instrument_key | Option<String> | 只读，DB 生成列 |
| provider_config | Option<Value> | 提供商配置 JSON |
| exchange_name | Option<String> | 派生友好名称，不存 DB |
| created_at / updated_at | NaiveDateTime | |

方法：`enrich()`（填充 exchange_name）、`is_holdable()`、`needs_pricing()`、`is_alternative()`、`is_investment()`、`is_option()`、`is_equity_like()`、`is_bond()`、`is_metal()`、`contract_multiplier() -> Decimal`、`option_spec() -> Option<OptionSpec>`、`bond_spec() -> Option<BondSpec>`、`to_instrument_id() -> Option<InstrumentId>`（转为市场数据提供商的路由标识）、`preferred_provider()`、`provider_overrides()`。Derives: Debug, Clone, Serialize, Deserialize, Default (camelCase)。

##### assets::OptionSpec (assets_model.rs:147)

| 字段 | 类型 | Notes |
|------|------|-------|
| underlying_asset_id | String | 标的资产 ID |
| expiration | NaiveDate | 到期日 |
| right | String | CALL 或 PUT |
| strike | Decimal | 行权价 |
| multiplier | Decimal | 合约乘数（通常 100） |
| occ_symbol | Option<String> | OCC 标准符号 |

##### assets::BondSpec (assets_model.rs:162)

| 字段 | 类型 | Notes |
|------|------|-------|
| maturity_date | Option<NaiveDate> | |
| coupon_rate | Option<Decimal> | 年票息率 |
| face_value | Option<Decimal> | 面值（通常 1000） |
| coupon_frequency | Option<String> | ANNUAL/SEMI_ANNUAL/QUARTERLY/MONTHLY |
| isin | Option<String> | |

##### assets::AssetProfile (assets_model.rs:68)

Asset 的扁平化包装，外加 `valuation_market_price` 和 `valuation_market_currency`。

##### assets::NewAsset (assets_model.rs:606)

创建输入模型，字段与 Asset 基本一致（id 可选）。额外字段：`provider_id`、`provider_symbol`。方法：`validate()`、`new_fx_asset(base, quote, provider)`、`new_option_contract(&OptionSpec, currency)`、`new_bond(isin, name, spec, currency)`。`From<ProviderProfile> for NewAsset` 实现将提供商侧数据转化为新资产。

##### assets::UpdateAssetProfile (assets_model.rs:851)

更新输入，所有字段为 Option。方法：`metadata_only()`、`validate()`。

##### assets::AssetSpec (assets_model.rs:1169) — 资产确保存在的规格

字段：id、display_code、instrument_symbol、instrument_exchange_mic、instrument_type、quote_ccy、requested_quote_ccy、kind、quote_mode、name、provider_config、provider_id、provider_symbol、metadata。方法：`market_instrument(...)`、`option_multiplier() -> Option<Decimal>`、`instrument_key() -> Option<String>`（模拟 DB 生成列）。

##### assets::EnsureAssetsResult (assets_model.rs:1280) — 批量确保资产存在的返回

| 字段 | 类型 | Notes |
|------|------|-------|
| assets | HashMap<String, Asset> | 全部资产（已存在+新建） |
| created_ids | Vec<String> | 新建资产的 ID |
| merge_candidates | Vec<(String, String)> | (resolved_id, unknown_id) 合并候选 |

##### assets::AssetResolutionInput (asset_resolution.rs:13)

| 字段 | 类型 | Notes |
|------|------|-------|
| key | String | 唯一标识键 |
| source_symbol | String | 原始符号 |
| account_currency | String | |
| activity_currency | Option<String> | |
| exchange_mic / quote_ccy / instrument_type / quote_mode | Option… | |
| isin / asset_id / provider_id / provider_symbol | Option… | |

方法：`reviewed_metadata_is_sufficient()`。

##### assets::AssetResolutionOutput (asset_resolution.rs:81)

解析结果：key、source_symbol、canonical_symbol、exchange_mic、quote_ccy（含来源枚举 QuoteCcyResolutionSource）、instrument_type、kind、provider_id、provider_symbol、provider_config、review_symbol、existing_asset_id、name、draft（NewAsset）。

##### 替代资产模型 (alternative_assets_model.rs)

- `CreateAlternativeAssetRequest`：kind、name、currency、current_value、value_date、purchase_price/date、metadata、linked_asset_id。
- `CreateAlternativeAssetResponse`：asset_id、quote_id。
- `UpdateValuationRequest`/`UpdateValuationResponse`：更新估值。
- `LinkLiabilityRequest`/`LinkLiabilityResponse`：负债与资产关联。
- `AlternativeHolding`：展示模型（含 unrealized_gain/pct、valuation_date、linked_asset_id）。
- `UpdateAssetDetailsRequest`/`UpdateAssetDetailsResponse`：更新名称/备注/元数据。

#### portfolio 模型概览

文件树（`F:\dev\wealthfolio\crates\core\src\portfolio\`）：
- `holdings/holdings_model.rs` — 持仓展示模型
- `snapshot/positions_model.rs` — Position、Lot 核心模型
- `snapshot/snapshot_model.rs` — AccountStateSnapshot 等
- `allocation/allocation_model.rs` — 分类配置模型
- `performance/performance_model.rs` — 绩效模型
- `valuation/valuation_model.rs` — 估值模型
- `net_worth/net_worth_model.rs` — 净资产
- `income/income_model.rs` — 收入
- `allocation_targets/model.rs` — 配置目标
- `fire/model.rs` — FIRE 计算

##### portfolio::HoldingType (holdings_model.rs:14)

变体：Cash、Security、AlternativeAsset。Derives: Serialize, Deserialize, Debug, Clone, PartialEq。

##### portfolio::MonetaryValue (holdings_model.rs:40)

| 字段 | 类型 | Notes |
|------|------|-------|
| local | Decimal | 本地币种价值 |
| base | Decimal | 基础币种价值 |

方法：`zero()`。

##### portfolio::Holding (holdings_model.rs:78) — 前端持仓展示模型

核心字段：id、account_id、holding_type、instrument（Option<Instrument>，含 id/symbol/name/currency/pricing_mode/preferred_provider/exchange_mic/classifications）、asset_kind、quantity、open_date、lots（Option<VecDeque<Lot>>）、contract_multiplier、local_currency/base_currency/fx_rate、market_value（MonetaryValue）、cost_basis、price、purchase_price、unrealized_gain/realized_gain/total_gain/income/total_return（均 Option<MonetaryValue>）、day_change/prev_close_value、weight、as_of_date、metadata、source_account_ids。

##### portfolio::HoldingListItem (holdings_model.rs:167) — 精简列表版本

相同字段但省略 full asset profile（不含 notes、provider config、lots、metadata）。`From<Holding>` 实现。

##### portfolio::Position (positions_model.rs:80) — 核心持仓位置

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | 格式 "POS-{asset_id}-{account_id}" |
| account_id | String | |
| asset_id | String | |
| quantity | Decimal | 有效（当前、拆分后）股数 |
| average_cost | Decimal | 每单位平均成本 |
| total_cost_basis | Decimal | 总成本基础 |
| currency | String | 资产币种 |
| inception_date | DateTime<Utc> | |
| lots | VecDeque<Lot> | 序列化时跳过（STEP 2 优化），但反序列化兼容旧格式 |
| is_alternative | bool | |
| contract_multiplier | Decimal | 默认 1 |
| cost_basis_account | Option<Decimal> | 预计算账户币种成本（快照写入时） |
| cost_basis_base | Option<Decimal> | 预计算基础币种成本 |

方法：`new()`、`new_with_alternative_flag()`、`recalculate_aggregates()`、`recalculate_aggregates_with_policy()`、`add_lot
`、`add_lot_values(...)`、`open_lot_signed(...)`、`add_transferred_lots(...)`、`reduce_lots_fifo()`、`reduce_positive_lots_fifo()`、`reduce_negative_lots_fifo()`、`apply_split(ratio, date)`、`basis_status()`。Derives: Serialize, Deserialize, Debug, Clone, PartialEq。

##### portfolio::Lot (positions_model.rs:156) — 税务批次

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | 通常为活动 ID |
| position_id | String | |
| acquisition_date | DateTime<Utc> | |
| acquisition_local_date | Option<NaiveDate> | 用户时区日历日期 |
| quantity | Decimal | 获取时数量（被卖出/拆分修改） |
| original_quantity | Decimal | 不可变原始数量 |
| cost_basis | Decimal | 总成本（含费用税费） |
| acquisition_price | Decimal | 每单位价格 |
| acquisition_fees | Decimal | 被卖出比例修改 |
| original_acquisition_fees | Decimal | 不可变原始费用 |
| acquisition_taxes / original_acquisition_taxes | Decimal | 同上 |
| fx_rate_to_position | Option<Decimal> | 活动→位置币种汇率 |
| fx_rate_to_account | Option<Decimal> | 获取时账户币种汇率 |
| fx_rate_to_base | Option<Decimal> | 获取时基础币种汇率 |
| account_currency / base_currency | Option<String> | 对应汇率的目标币种 |
| source_activity_id | Option<String> | 创建此批次的源活动 ID（FK CASCADE） |
| split_ratio | Decimal | 拆分后累积比率，默认 1.0 |

方法：`basis_status()`、`effective_split_ratio()`、`effective_quantity()`、`acquisition_date_key()`、`stored_fx_rate_to(target_currency)`、`original_fees()`、`original_taxes()`。

##### portfolio::FifoReductionResult (positions_model.rs:337)

字段：`quantity_reduced`、`cost_basis_removed`、`removed_lots: Vec<Lot>`、`fully_consumed_lot_ids: Vec<String>`、`fully_consumed_lots: Vec<Lot>`。

##### portfolio::CashHolding (positions_model.rs:357)

字段：id、account_id、currency、amount、last_updated。

##### portfolio::Holding (positions_model.rs:366) — tagged enum

变体：Security(Position)、Cash(CashHolding)。

##### portfolio::AccountStateSnapshot (snapshot_model.rs:123)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | 稳定 UUID（account_id + snapshot_date 的 SHA-256 派生） |
| account_id | String | |
| snapshot_date | NaiveDate | |
| currency | String | 账户报告币种 |
| positions | HashMap<String, Position> | asset_id → Position |
| cash_balances | HashMap<String, Decimal> | 币种 → 余额 |
| cost_basis | Decimal | 账户币种总成本基础 |
| net_contribution | Decimal | 累计净存入（账户币种） |
| net_contribution_base | Decimal | 基础币种 |
| cash_total_account_currency | Decimal | 缓存现金总额（账户币种） |
| cash_total_base_currency | Decimal | 缓存现金总额（基础币种） |
| calculated_at | NaiveDateTime | |
| source | SnapshotSource | 枚举：Calculated/ManualEntry/BrokerImported/CsvImport |

方法：`stable_id(account_id, date)`、`is_content_equal(&other)`。Derives: Serialize, Deserialize, Debug, Clone, PartialEq。

##### portfolio::SnapshotSource (snapshot_model.rs:14)

变体：Calculated (default)、ManualEntry、BrokerImported、CsvImport。Derives: Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default (SCREAMING_SNAKE_CASE)。

##### portfolio::SnapshotMetadata (snapshot_model.rs:34)

轻量元数据：id、account_id、snapshot_date (String)、source (String)、position_count、cash_currency_count、cash_total_account_currency。`From<&AccountStateSnapshot>` 实现。

##### portfolio::HoldingsCalculationWarning (snapshot_model.rs:73)

字段：activity_id、account_id、date、message。`Display` 实现。

##### portfolio::HoldingsCalculationResult (snapshot_model.rs:94)

字段：snapshot (AccountStateSnapshot)、warnings (Vec<HoldingsCalculationWarning>)。方法：`new()`、`with_warnings()`、`has_warnings()`。

##### allocation 模型 (allocation_model.rs)

- `CategoryAllocation`：category_id/name/color、value (Decimal)、percentage (Decimal)、children (Vec<CategoryAllocation>)。Derives: Debug, Clone, Serialize, Deserialize (camelCase)。
- `TaxonomyAllocation`：taxonomy_id/name/color、categories (Vec<CategoryAllocation>)。方法：`empty()`。Derives: Debug, Clone, Serialize, Deserialize (camelCase)。
- `PortfolioAllocations`：asset_classes、sectors、regions、risk_category、security_types（均为 TaxonomyAllocation）、custom_groups (Vec)、total_value。`Default` 实现预设 5 个默认分类法的颜色。Derives: Debug, Clone, Serialize, Deserialize (camelCase)。
- `AllocationHoldings`：taxonomy_id/name、category_id/name/color、holdings (Vec<HoldingSummary>)、total_value、currency。Derives: Debug, Clone, Serialize, Deserialize (camelCase)。
- `HoldingAllocationContribution`：id、holding_id、asset_id、account_id、source_account_ids、symbol、name、holding_type、quantity、category_id/name/color、value。Derives: Debug, Clone, Serialize, Deserialize (camelCase)。
- `TaxonomyHoldingContributions`：taxonomy_id/name、total_value、currency、contributions (Vec<HoldingAllocationContribution>)。Derives: Debug, Clone, Serialize, Deserialize (camelCase)。

##### performance 模型 (performance_model.rs)

- `ReturnMethod`：TimeWeighted (default)、ValueReturn、SymbolPriceBased、NotApplicable。Derives: Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Default (camelCase)。
- `PerformanceReturns`：twr、annualized_twr、irr、annualized_irr、value_return、annualized_value_return（均为 Option<Decimal>）。Derives: Debug, Clone, Serialize, Deserialize, PartialEq (camelCase)。
- `PerformanceAttribution`：contributions、distributions、income、realized_pnl、unrealized_pnl_change、fx_effect、fees、taxes、residual（均为 Decimal，默认 ZERO）。Derives: Debug, Clone, Serialize, Deserialize, PartialEq (camelCase)。
- `PerformanceRisk`：volatility、max_drawdown、peak_date、trough_date、recovery_date、drawdown_duration_days（均为 Option）。Derives: Debug, Clone, Serialize, Deserialize, PartialEq (camelCase)。
- `DataQualityStatus`：Ok、Partial、NoData、NotApplicable。Derives: Debug, Clone, Serialize, Deserialize, PartialEq, Eq (camelCase)。
- `PerformanceDataQuality`：status、warnings (Vec<String>)、not_applicable_reasons (Vec<String>)。方法：`ok()`、`no_data(reason)`。Derives: Debug, Clone, Serialize, Deserialize, PartialEq (camelCase)。
- `PerformanceSummaryBasis`：MarketValue、BookBasis、Mixed、NotApplicable (default)。Derives: Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default (camelCase)。
- `PerformanceSummaryStatus`：Complete、Unavailable (default)。Derives: Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default (camelCase)。
- `PerformanceSummary`：amount/percent (Option<Decimal>)、method、basis、quality、amount_status、percent_status、basis_status、reasons (Vec<String>)。`Default` 实现全部 unavailable。Derives: Debug, Clone, Serialize, Deserialize, PartialEq (camelCase)。
- `PerformanceResult`：scope (PerformanceScopeDescriptor)、period (PerformancePeriod)、mode、returns、attribution、risk、data_quality、basis_status、summary、series (Vec<ReturnData>)、is_holdings_mode、is_mixed_tracking_mode、#[serde(skip)] holdings_flows_unavailable。Derives: Debug, Clone, Serialize, Deserialize, PartialEq (camelCase)。
- `PerformanceSummaryProfile`：Full (default)、Summary、Dashboard。Derives: Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq (camelCase)。
- `PerformanceSummaryBatchResult`：results (HashMap<String, PerformanceResult>)、failed_scope_count、scope_timings (Vec<PerformanceSummaryScopeTiming>)、elapsed_ms。Derives: Debug。
- `SimplePerformanceMetrics`：account_id、account_currency/base_currency (Option<String>)、fx_rate_to_base、total_value、total_gain_loss_amount、cumulative_return_percent、portfolio_weight（均为 Option<Decimal>）。Derives: Debug, Clone, Serialize, Deserialize, PartialEq (camelCase)。
- 辅助函数：`performance_tracking_composition()`、`performance_summary_scope_key()`、`unique_account_ids()`、`performance_account_ids_from_map()`、`performance_account_tracking_modes_from_map()`、`empty_performance_metrics()`、`unavailable_performance_metrics(reason)`、`sync_performance_summary_quality()`。


### 批次 3 — 目标 (goals)、规划 (planning)、报价 (quotes)、外汇 (fx) 模型

#### goals 模块

文件：`F:\dev\wealthfolio\crates\core\src\goals\goals_model.rs`

##### goals::Goal (goals_model.rs:10)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | |
| goal_type | String | 目标类型标识 |
| title | String | |
| description | Option<String> | |
| target_amount | Option<f64> | 目标金额 |
| status_lifecycle | String | 生命周期状态 |
| status_health | String | 健康状态（on_track/at_risk/off_track/achieved） |
| priority | i32 | 优先级 |
| cover_image_key | Option<String> | 封面图 |
| currency | Option<String> | |
| start_date / target_date | Option<String> | ISO 日期字符串 |
| summary_current_value | Option<f64> | 当前值摘要 |
| summary_progress | Option<f64> | 进度（0.0-1.0） |
| projected_completion_date | Option<String> | |
| projected_value_at_target_date | Option<f64> | |
| summary_target_amount | Option<f64> | |
| created_at / updated_at | String | |

Derives: Debug, Clone, Serialize, Deserialize, PartialEq (camelCase)。

##### goals::NewGoal (goals_model.rs:35)

创建输入模型，字段与 Goal 基本一致但全部为 Option（除 goal_type、title 外）。Derives: Serialize, Deserialize, Debug, Clone (camelCase)。

##### goals::GoalFundingRule (goals_model.rs:55)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | |
| goal_id | String | FK → goals |
| account_id | String | FK → accounts |
| share_percent | f64 | 分配百分比 |
| tax_bucket | Option<String> | taxable/tax_deferred/tax_free |
| created_at / updated_at | String | |

##### goals::GoalFundingRuleInput (goals_model.rs:68)

输入：account_id、share_percent、tax_bucket。

##### goals::GoalPlan (goals_model.rs:88)

| 字段 | 类型 | Notes |
|------|------|-------|
| goal_id | String | 1:1 with goal |
| plan_kind | String | |
| planner_mode | Option<String> | |
| settings_json | String | 计划参数 JSON |
| summary_json | String | 计算结果 JSON |
| version | i32 | 乐观锁 |
| created_at / updated_at | String | |

##### goals::SaveGoalPlan (goals_model.rs:102)

输入模型：goal_id、plan_kind、planner_mode、settings_json、summary_json。

##### goals::PreparedRetirementSimulationInput (goals_model.rs:112)

后端预处理的退休模拟输入：plan (RetirementPlan)、current_portfolio、planner_mode (RetirementTimingMode)。

#### planning 模块

文件树：`F:\dev\wealthfolio\crates\core\src\planning\`
- `mod.rs` — re-exports retirement + save_up
- `save_up.rs` — 存钱目标引擎
- `retirement/model.rs` — 退休计划领域模型
- `retirement/dto.rs` — 退休计划输出 DTO
- `retirement/engine.rs` — 退休规划引擎
- `retirement/withdrawal.rs` — 退休提款税务模型
- `retirement/analysis.rs` — 蒙特卡洛/压力测试

##### planning::SaveUpInput (save_up.rs:23)

| 字段 | 类型 | Notes |
|------|------|-------|
| current_value | f64 | |
| target_amount | f64 | |
| target_date | Option<String> | ISO 日期，None 表示开放式 |
| monthly_contribution | f64 | |
| expected_annual_return | f64 | 小数（0.07 = 7%） |

##### planning::SaveUpOverview (save_up.rs:34)

| 字段 | 类型 | Notes |
|------|------|-------|
| current_value | f64 | |
| target_amount | f64 | |
| progress | f64 | 0.0-1.0 |
| health | String | on_track/at_risk/off_track/not_applicable |
| projected_value_at_target_date | f64 | |
| required_monthly_contribution | f64 | 二分法求解的最小月供 |
| projected_completion_date | Option<String> | |
| trajectory | Vec<SaveUpTrajectoryPoint> | 月度轨迹（3 场景） |

##### planning::SaveUpTrajectoryPoint (save_up.rs:49)

字段：date (YYYY-MM)、nominal、optimistic、pessimistic、target（均为 f64）。

##### planning::RetirementPlan (retirement/model.rs:30)

| 字段 | 类型 | Notes |
|------|------|-------|
| version | Option<String> | |
| personal | PersonalProfile | 年龄、薪资等 |
| expenses | ExpenseBudget | 支出项 |
| income_streams | Vec<RetirementIncomeStream> | 退休收入流 |
| investment | InvestmentAssumptions | 投资假设 |
| tax | Option<TaxProfile> | 税务配置 |
| currency | String | |

Derives: Debug, Clone, Serialize, Deserialize, PartialEq (camelCase)。

##### planning::PersonalProfile (retirement/model.rs:43)

字段：birth_year_month、current_age、target_retirement_age、planning_horizon_age、current_annual_salary、salary_growth_rate。

##### planning::ExpenseBudget (retirement/model.rs:173)

字段：items (Vec<ExpenseBucket>)。方法：all_buckets()。

##### planning::ExpenseBucket (retirement/model.rs:180)

字段：id、label、monthly_amount、inflation_rate、start_age、end_age、essential。

##### planning::RetirementIncomeStream (retirement/model.rs:203)

字段：id、label、stream_type (StreamKind)、start_age、adjust_for_inflation、annual_growth_rate、monthly_amount、linked_account_id、current_value、monthly_contribution、accumulation_return。

##### planning::StreamKind (retirement/model.rs:218)

变体：DefinedBenefit (db)、DefinedContribution (dc)。

##### planning::InvestmentAssumptions (retirement/model.rs:228)

字段：pre_retirement_annual_return (默认 5.77%)、retirement_annual_return (默认 3.37%)、annual_investment_fee_rate (默认 0.6%)、annual_volatility (默认 12%)、inflation_rate、monthly_contribution、contribution_growth_rate、glide_path (Option<GlidepathSettings>)。

##### planning::RetirementTimingMode (retirement/model.rs:247)

变体：Fire (默认)、Traditional。方法：from_str()、as_str()。

##### planning::RetirementStartReason (retirement/model.rs:273)

变体：Funded、TargetAgeForced。

##### planning::TaxBucketBalances (retirement/model.rs:281)

字段：taxable、tax_deferred、tax_free（均为 f64）。方法：total()、scale_to_total()。

##### planning::TaxProfile (retirement/model.rs:317)

字段：taxable/tax_deferred/tax_free_withdrawal_rate、early_withdrawal_penalty_rate、early_withdrawal_penalty_age、country_code、withdrawal_buckets (TaxBucketBalances)。

##### planning::YearlySnapshot (dto.rs:21)

每年的快照输出：age、year、phase、portfolio_value/end_value、annual_contribution/withdrawal/income、net_withdrawal_from_portfolio、pension_assets、annual_taxes、gross_withdrawal、planned_expenses、funded_expenses、annual_shortfall。

##### planning::FireProjection (dto.rs:46)

退休规划完整输出：fire_age、fire_year、retirement_start_age、retirement_start_reason、portfolio_at_fire、funded_at_retirement、coast_fire_amount、coast_fire_reached、year_by_year (Vec<YearlySnapshot>)。

##### planning::RetirementOverview (dto.rs:264)

退休概览全部字段（约 30 个）：analysis_mode、status、success_status、desired_fire_age、fi_age、retirement_start_age、funded_at_goal_age、eventually_reaches_fi、portfolio_now、portfolio_at_retirement_start、net_fire_target、gross_fire_target、shortfall/surplus_at_goal_age、funded_through_age、failure_age、spending_shortfall_age、required_additional_monthly_contribution、suggested_goal_age_if_unchanged、coast_amount_today、coast_reached、progress、tax_bucket_balances、budget_breakdown、target_reconciliation、trajectory。

##### planning::MonteCarloResult (dto.rs:87)

字段：success_rate、median_fire_age、percentiles (PercentilePaths: p10-p90)、age_axis、final_portfolio_at_horizon、n_simulations。

##### planning::ScenarioResult (dto.rs:100)

字段：label、annual_return、fire_age、portfolio_at_horizon、funded_at_goal_age、success、failure_age、spending_shortfall_age、year_by_year。

##### planning::StressTestResult (dto.rs:226)

字段：id (StressTestId)、label、description、category (StressCategory)、baseline/stressed (StressOutcome)、delta (StressDelta)、severity (StressSeverity)。

##### planning::DecisionSensitivityMatrix (dto.rs:154)

字段：row_label、column_label、row/column_values、cells (Vec<Vec<DecisionSensitivityCell>>)、baseline_row/column。

##### planning::WithdrawalOutcome (withdrawal.rs:11)

私有：remaining_buckets、gross_withdrawal、spending_funded、tax_amount。

#### quotes 模块

文件：`F:\dev\wealthfolio\crates\core\src\quotes\`
- `model.rs` — Quote、SymbolSearchResult、LatestQuotePair、ResolvedQuote
- `types.rs` — 强类型：AssetId、ProviderId、Day、QuoteSource、Currency、quote_id()
- `errors.rs` — MarketDataError 枚举（15 种变体，含 is_terminal/should_try_next_provider/is_transient）
- `constants.rs` — 数据源标识常量、同步参数（DEFAULT_HISTORY_DAYS=1825、CLOSED_POSITION_GRACE_PERIOD_DAYS=30 等）

##### quotes::Quote (model.rs:34)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | 格式：`{asset_id}_{date}_{source}` |
| asset_id | String | |
| timestamp | DateTime<Utc> | |
| open / high / low / close | Decimal | O
| open / high / low / close | Decimal | OHLC |
| adjclose | Decimal | 调整收盘价（拆股/股息） |
| volume | Decimal | |
| currency | String | |
| data_source | String | |
| created_at | DateTime<Utc> | |
| notes | Option<String> | 手动输入备注 |

Derives: Debug, Clone, Serialize, Deserialize, PartialEq, Default (camelCase)。

##### quotes::SymbolSearchResult (model.rs:79)

约 20 个字段：symbol、canonical_symbol、canonical_exchange_mic、provider_id、provider_symbol、short_name、long_name、exchange、exchange_mic、exchange_name、quote_type、type_display、currency、currency_source、data_source、quote_mode、is_existing、existing_asset_id、index、score。Derives: Serialize, Deserialize, Clone, Debug, Default (camelCase)。

##### quotes::LatestQuotePair (model.rs:127)

字段：latest (Quote)、previous (Option<Quote>)。

##### quotes::ResolvedQuote (model.rs:137)

字段：currency、price (Option<Decimal>)、resolved_provider_id。

##### quotes::AssetId (types.rs:24)

newtype(String)。方法：new()、as_str()。From<String>/From<&str>。Derives: Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default。

##### quotes::ProviderId (types.rs:69)

newtype(String)。常量：YAHOO、ALPHA_VANTAGE、MARKETDATA_APP、METAL_PRICE_API、FINNHUB、US_TREASURY_CALC、BOERSE_FRANKFURT、OPENFIGI、BROKER、CUSTOM_SCRAPER。方法：new()、yahoo()、alpha_vantage()、as_str()。Derives: Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize。

##### quotes::Day (types.rs:162)

newtype(NaiveDate)。方法：new()、from_ymd()、date()、parse()、today()。From<NaiveDate>/From<Day>。Derives: Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord。

##### quotes::QuoteSource (types.rs:223)

枚举，tagged：Manual 或 Provider(ProviderId)。方法：is_manual()、is_provider()、provider()、to_storage_string()、from_storage_string()。手动报价永不覆盖提供商报价。

##### quotes::Currency (types.rs:307)

newtype(String)。方法：new()、usd()、as_str()。

##### 辅助函数 (types.rs:288)

- `quote_id(asset_id, day, source)` → 构造确定性报价 ID
- `day_source_suffix(day, source)` → `{YYYY-MM-DD}_{source}`

##### quotes::MarketDataError (errors.rs:14)

15 种变体（使用 thiserror）：DatabaseError、DatabaseConnectionError、ProviderError、NetworkError、ParsingError、NotFound、Unauthorized、RateLimitExceeded、InvalidData、ProviderExhausted、NoData、NoProvidersAvailable、UnsupportedAssetType、CircuitOpen、Timeout、Unknown。方法：is_terminal()、should_try_next_provider()、is_transient()。From<YahooError> 和 From<ExternalMarketDataError> 实现。

#### fx 模块

文件：`F:\dev\wealthfolio\crates\core\src\fx\`
- `fx_model.rs` — ExchangeRate、NewExchangeRate、FxContext
- `fx_errors.rs` — FxError 枚举（8 种变体）
- `fx_traits.rs` — FxServiceTrait、FxRepositoryTrait
- `fx_service.rs` — 汇率服务实现
- `currency_converter.rs` — 币种转换器
- `currency.rs` — 币种信息

##### fx::FxContext (fx_model.rs:7)

枚举：ValuationDate、AcquisitionDate、FlowDate。Derives: Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq (camelCase)。

##### fx::ExchangeRate (fx_model.rs:17)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | 资产 UUID |
| from_currency | String | |
| to_currency | String | |
| rate | Decimal | 使用 decimal_serde 序列化 |
| source | String | 数据源 |
| timestamp | DateTime<Utc> | |

方法：`to_quote()`（转为 Quote 存储）、`make_instrument_key(from, to)` → `"FX:EUR/USD"`、`parse_fx_pair(key)`（支持 5 种格式）。Derives: Serialize, Deserialize, Debug, Clone (camelCase)。

##### fx::NewExchangeRate (fx_model.rs:89)

创建输入：from_currency、to_currency、rate、source。

##### fx::FxError (fx_errors.rs:8)

变体：DatabaseError、RateNotFound、InvalidCurrencyPair、InvalidCurrencyCode、CacheError、ConversionError、SaveError、FetchError。From<DatabaseError> 和 From<PoisonError> 实现。


---

## Batch 4 — Sync, Settings, Health, Limits, Lots, Events, Errors

### 4.1 核心错误类型 (errors.rs)

#### Error (errors.rs:23)

根错误类型，数据库无关。所有底层错误转换为本类型。

| 字段 | 类型 | Notes |
|------|------|-------|
| Database | DatabaseError | 来自 |
| Asset | String | 资产操作失败 |
| ConstraintViolation | String | 约束违反 |
| CurrencyConversionFailed | String | 货币转换失败 |
| UnsupportedCurrency | String | 不支持的货币 |
| InvalidExchangeRate | String | 无效汇率 |
| Validation | ValidationError | 来自 |
| ConfigIO | String | 配置加载失败 |
| InvalidConfigValue | String | 无效配置值 |
| MissingConfigKey | String | 缺少配置键 |
| MarketData | MarketDataError | 来自 |
| Activity | ActivityError | 来自 |
| Repository | String | 仓库错误 |
| Calculation | CalculatorError | 来自 |
| Secret | String | 密钥存储错误 |
| Unexpected | String | 意外错误 |
| Fx | FxError | 来自 |

Derives: Error(Debug). 类型别名: `pub type Result<T> = std::result::Result<T, Error>`.

#### DatabaseError (errors.rs:81)

数据库无关错误类型，String 包装具体实现。

变体：ConnectionFailed、PoolCreationFailed、QueryFailed、NotFound、UniqueViolation、ForeignKeyViolation、TransactionFailed、MigrationFailed、BackupFailed、RestoreFailed、Internal。Derives: Error, Debug。

#### CalculatorError (errors.rs:129)

计算错误。

变体：InvalidActivity、InsufficientShares(asset_id, account_id, date)、CurrencyMismatch(position_id, position_currency, activity_id, activity_currency)、CurrencyConversion、MissingFxRate(from, to, date)、PositionNotFound(asset_id, account_id)、LotNotFound(lot_id)、UnsupportedActivityType、Calculation。Derives: Error, Debug。

#### ValidationError (errors.rs:173)

验证错误。

变体：NumberParse(ParseFloatError)、InvalidInput、MissingField、DecimalParse(DecimalError)、DateTimeParse(ChronoParseError)、InvalidSnapshotDate(account_id, date, min_date, max_date, snapshot_source)。Derives: Error, Debug。

### 4.2 Events 模块 (events/)

#### DomainEvent (domain_event.rs:15)

核心领域事件枚举，使用 serde(tag = "type", rename_all = "snake_case")。

| 变体 | 字段 | 说明 |
|------|------|------|
| ActivitiesChanged | account_ids, asset_ids, currencies, earliest_activity_at_utc | 活动变更 |
| AssetSplitActivitiesChanged | asset_ids, earliest_activity_at_utc | 资产拆分活动变更 |
| HoldingsChanged | account_ids, asset_ids, earliest_snapshot_date | 持仓快照变更 |
| AccountsChanged | account_ids, currency_changes: Vec<CurrencyChange> | 账户变更 |
| AssetsCreated | asset_ids | 资产创建 |
| AssetsUpdated | asset_ids | 资产更新 |
| AssetClassificationsChanged | asset_ids, taxonomy_ids | 资产分类变更 |
| AssetsMerged | source_id, target_id, activities_migrated: u32 | 资产合并 |
| TrackingModeChanged | account_id, old_mode, new_mode, is_connected | 跟踪模式变更 |
| DeviceSyncPullComplete | (无字段) | 设备同步完成 |

Derives: Clone, Debug, Serialize, Deserialize。

构造函数方法：activities_changed()、asset_split_activities_changed()、holdings_changed()、accounts_changed()、assets_created()、assets_updated()、asset_classifications_changed()、assets_merged()、tracking_mode_changed()、device_sync_pull_complete()。

##### CurrencyChange (domain_event.rs:87)

| 字段 | 类型 | Notes |
|------|------|-------|
| account_id | String | |
| old_currency | Option<String> | |
| new_currency | String | |

#### DomainEventSink (sink.rs:17)

接收领域事件的 trait。

| 方法 | 签名 |
|------|------|
| emit | fn(&self, event: DomainEvent) |
| emit_batch | fn(&self, events: Vec<DomainEvent>) — 默认实现逐个调用 emit() |

设计规则：emit() 必须快速且非阻塞；实现应队列化异步处理；失败不影响领域操作（尽力而为）。Derives: Send + Sync。

##### NoOpDomainEventSink (sink.rs:33)

空实现，丢弃所有事件。Derives: Clone, Default。

##### MockDomainEventSink (sink.rs:44)

测试用，收集事件到 Arc<Mutex<Vec<DomainEvent>>>。方法：new()、events()、clear()、len()、is_empty()。Derives: Clone, Default。

### 4.3 Sync 模块 (sync/)

#### SyncEntity (app_sync_model.rs:84)

38 个变体的同步实体枚举。serde(snake_case)。

变体：Account、Asset、Quote、AssetTaxonomyAssignment、Activity、BrokerActivityUserPatch、ActivityImportProfile、ImportTemplate、Goal、GoalPlan、GoalsAllocation、AiThread、AiMessage、AiThreadTag、ContributionLimit、Platform、Snapshot、CustomProvider、CustomTaxonomy、ImportRun、Portfolio、PortfolioAccount、AllocationTarget、AllocationTargetWeight、AllocationTargetConstraint、SpendingSetting、ActivityTaxonomyAssignment、SpendingActivitySplit、SpendingActivityEvent、SpendingCategorizationRule、SpendingPresetRuleDeletion、SpendingEvent、SpendingEventType、BudgetGroup、BudgetGroupAssignment、BudgetTarget、BudgetRolloverSetting、AddonStorage。

Derives: Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize。

#### SyncOperation (app_sync_model.rs:133)

变体：Create、Update、Delete。Derives: Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize (snake_case)。

#### SyncOutboxStatus (app_sync_model.rs:141)

变体：Pending、Sent、Dead。Derives: Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize (snake_case)。

#### SyncOutboxEvent (app_sync_model.rs:151)

| 字段 | 类型 | Notes |
|------|------|-------|
| event_id | String | |
| entity | SyncEntity | |
| entity_id | String | |
| op | SyncOperation | |
| client_timestamp | String | |
| payload | String | |
| payload_key_version | i32 | |
| sent | bool | |
| status | SyncOutboxStatus | |
| retry_count | i32 | |
| next_retry_at | Option<String> | |
| last_error | Option<String> | |
| last_error_code | Option<String> | |
| created_at | String | |

Derives: Debug, Clone, PartialEq, Serialize, Deserialize (camelCase)。

#### SyncEntityMetadata (app_sync_model.rs:171)

| 字段 | 类型 |
|------|------|
| entity | SyncEntity |
| entity_id | String |
| last_event_id | String |
| last_client_timestamp | String |
| last_op | SyncOperation |
| last_seq | i64 |

Derives: Debug, Clone, PartialEq, Eq, Serialize, Deserialize (camelCase)。

#### SyncEngineStatus (app_sync_model.rs:183)

| 字段 | 类型 | Notes |
|------|------|-------|
| cursor | i64 | |
| last_push_at | Option<String> | |
| last_pull_at | Option<String> | |
| last_error | Option<String> | |
| consecutive_failures | i32 | |
| next_retry_at | Option<String> | |
| last_cycle_status | Option<String> | |
| last_cycle_duration_ms | Option<i64> | |

Derives: Debug, Clone, PartialEq, Eq, Serialize, Deserialize (camelCase)。

#### SyncReplayResult (app_sync_model.rs:197)

| 字段 | 类型 |
|------|------|
| event_id | String |
| entity | SyncEntity |
| entity_id | String |
| applied | bool |
| skipped_reason | Option<String> |

#### SyncEnvelopeV1 (app_sync_model.rs:207)

| 字段 | 类型 |
|------|------|
| version | i32 |
| entity | SyncEntity |
| op | SyncOperation |
| body | String |

#### SyncCycleTrigger (app_sync_model.rs:217)

变体：Startup、Foreground、LocalMutation、Periodic、Manual。Derives: Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize (snake_case)。

#### SyncApplyContext (app_sync_model.rs:229)

变体：LocalMutation、RemoteReplay。Derives: Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize (snake_case)。

#### should_apply_lww() (app_sync_model.rs:239)

LWW (Last-Writer-Wins) 冲突解决函数。规则：(1) 时间戳大的赢；(2) 时间戳相等时，event_id 字典序大的赢。返回 true 表示远程变更应覆盖本地。

#### EntitySyncAdapter trait (app_sync_model.rs:274)

同步引擎适配器 trait。

| 方法 | 签名 |
|------|------|
| entity | fn(&self) -> SyncEntity |
| serialize_create | fn(&self, entity_id: &str) -> Result<JsonValue, String> |
| serialize_update | fn(&self, entity_id: &str) -> Result<JsonValue, String> |
| serialize_delete | fn(&self, entity_id: &str) -> Result<JsonValue, String> |
| apply_event_lww | fn(&self, entity_id, event_id, client_timestamp, payload, previous: Option<&SyncEntityMetadata>, context: SyncApplyContext) -> Result<bool, String> |
| export_for_snapshot_import | fn(&self) -> Result<Vec<JsonValue>, String> |
| import_from_snapshot_rowset | fn(&self, rows: &[JsonValue]) -> Result<(), String> |

Derives: Send + Sync。

#### APP_SYNC_TABLES (app_sync_model.rs:7)

41 个表的同步顺序列表，按 FK 依赖关系排序。涵盖平台、资产、插件存储、自定义提供者、报价、目标、目标计划、AI 线程、贡献限额、账户、导入运行、活动、导入模板、应用设置、预算组、支出事件类型、导入账户模板、分类法、分类法类别、资产分类法分配、活动分类法分配、支出活动拆分、支出分类规则、预设规则删除、支出事件、活动事件标签、预算组分配、预算目标、预算滚动设置、目标分配、AI 消息、AI 线程标签、持仓快照、快照持仓、投资组合、投资组合账户、配置目标、配置目标权重、配置目标约束。

### 4.4 Settings 模块 (settings/)

#### Settings (s

#### Settings (settings_model.rs:7)

| 字段 | 类型 | 默认值 | Notes |
|------|------|--------|-------|
| theme | String | "light" | |
| font | String | "font-mono" | |
| language | String | "en" | |
| base_currency | String | "" | |
| timezone | String | "" | |
| onboarding_completed | bool | false | |
| auto_update_check_enabled | bool | true | |
| menu_bar_visible | bool | true | |
| sync_enabled | bool | true | |
| default_return_metric | String | "twr" | |

Derives: Serialize, Deserialize, Debug, Clone (camelCase)。实现 Default。

#### SettingsUpdate (settings_model.rs:39)

所有字段 Option 包装。Derives: Serialize, Deserialize, Debug, Clone (camelCase)。

#### Sort (settings_model.rs:54)

| 字段 | 类型 |
|------|------|
| id | String |
| desc | bool |

#### AppSetting (settings_model.rs:62)

| 字段 | 类型 |
|------|------|
| setting_key | String |
| setting_value | String |

### 4.5 Limits 模块 (limits/)

#### ContributionLimit (limits_model.rs:12)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | UUID |
| group_name | String | 分组名称 |
| contribution_year | i32 | 缴款年份 |
| limit_amount | f64 | 限额 |
| account_ids | Option<String> | 关联账户 ID 列表 |
| created_at | NaiveDateTime | |
| updated_at | NaiveDateTime | |
| start_date | Option<String> | |
| end_date | Option<String> | |

Derives: Debug, Clone, Serialize, Deserialize (camelCase)。

#### NewContributionLimit (limits_model.rs:26)

创建输入：id(Option)、group_name、contribution_year、limit_amount、account_ids(Option)、start_date(Option)、end_date(Option)。Derives: Serialize, Deserialize, Debug, Clone (camelCase)。

#### AccountDeposit (limits_model.rs:37)

| 字段 | 类型 |
|------|------|
| amount | Decimal |
| currency | String |
| converted_amount | Decimal |

#### DepositsCalculation (limits_model.rs:45)

| 字段 | 类型 |
|------|------|
| total | Decimal |
| base_currency | String |
| by_account | HashMap<String, AccountDeposit> |

#### ContributionActivity (limits_model.rs:55)

| 字段 | 类型 | Notes |
|------|------|-------|
| account_id | String | |
| activity_type | String | |
| activity_instant | DateTime<Utc> | |
| amount | Option<Decimal> | |
| currency | String | |
| metadata | Option<String> | |
| source_group_id | Option<String> | |

Derives: Debug, Clone。

### 4.6 Lots 模块 (lots/)

#### LotRecord (lots/mod.rs:251)

持久化税务批次记录，一行对应一个批次（acquired sub-lot），在处置时更新。

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | UUID |
| account_id | String | |
| asset_id | String | |
| open_date | String | ISO 8601 |
| open_activity_id | Option<String> | 创建批次的活动，NULL 表示非活动行 |
| original_quantity | String | 获取时总数量（pre-split 单位），不可变 |
| remaining_quantity | String | 仍持有数量（pre-split 单位） |
| cost_per_unit | String | 每单位成本（quote currency，pre-split 单位），不可变 |
| original_cost_basis | String | 创建时成本基础（cost_per_unit x original_quantity + fee），不可变 |
| remaining_cost_basis | String | 剩余成本基础，按比例减少 |
| original_cost_basis_base | String | 转换为 base currency 的原始成本基础 |
| remaining_cost_basis_base | String | 转换为 base currency 的剩余成本基础 |
| fee_allocated | String | 分配的交易费用，不可变 |
| fee_allocated_base | String | 转换为 base currency 的费用 |
| tax_allocated | String | 分配的税款，不可变 |
| tax_allocated_base | String | 转换为 base currency 的税款 |
| currency | String | 批次货币（通常为资产报价货币） |
| base_currency | String | 用户基础货币 |
| fx_rate_to_base | String | 获取时汇率 |
| fx_rate_to_account | Option<String> | 到账户货币的汇率 |
| account_currency | Option<String> | 账户货币 |
| cost_basis_method | String | 成本基础方法（FIFO/LIFO/WAC） |
| split_ratio | String | 拆分累积比率，默认为 "1" |
| is_closed | bool | 是否已完全处置 |
| close_date | Option<String> | 关闭日期 |
| close_activity_id | Option<String> | 关闭活动 |
| created_at | String | |
| updated_at | String | |

Derives: Debug, Clone, Serialize, Deserialize (camelCase)。

#### LotClosure (lots/mod.rs:37)

已完全处置的批次记录。

| 字段 | 类型 | Notes |
|------|------|-------|
| lot_id | String | |
| close_date | String | ISO 8601 |
| close_activity_id | Option<String> | |
| open_activity_id | Option<String> | |
| account_id | String | |
| asset_id | String | |
| open_date | String | |
| original_quantity | String | |
| cost_per_unit | String | |
| original_cost_basis | String | |
| original_cost_basis_base | String | |
| remaining_cost_basis_base | String | |
| fee_allocated | String | |
| fee_allocated_base | String | |
| tax_allocated | String | |
| tax_allocated_base | String | |
| currency | String | |
| base_currency | String | |
| fx_rate_to_base | String | |
| cost_basis_method | String | |
| split_ratio | String | |

Derives: Debug, Clone。

#### LotDisposal (lots/mod.rs:90)

确定性处置切片，某次卖出消耗 FIFO 批次时产生。

| 字段 | 类型 |
|------|------|
| id | String |
| lot_id | String |
| account_id | String |
| asset_id | String |
| disposal_activity_id | String |
| disposal_date | String |
| quantity | String |
| proceeds | String |
| cost_basis | String |
| realized_pnl | String |
| proceeds_base | String |
| cost_basis_base | String |
| realized_pnl_base | String |
| currency | String |
| base_currency | String |
| fx_rate_to_base | String |
| cost_basis_method | String |
| created_at | String |

Derives: Debug, Clone, Serialize, Deserialize (camelCase)。

#### AssetLotSource (lots/mod.rs:327)

枚举：TransactionLot、SnapshotPosition。Derives: Debug, Clone, Serialize, Deserialize, PartialEq, Eq (SCREAMING_SNAKE_CASE)。

#### AssetLotView (lots/mod.rs:339)

UI 层面批次视图。包含 TransactionLot 和 SnapshotPosition 两种来源。

关键字段：id、account_id、account_name、asset_id、source(AssetLotSource)、currency、base_currency(Option)、valuation_currency、quantity(Decimal)、original_quantity(Decimal)、remaining_quantity(Decimal)、cost_basis(Decimal)、cost_basis_base(Option)、unit_cost(Decimal)、fees(Decimal)、taxes(Decimal)、taxes_base(Option)、valuation_unit_cost(Decimal)、valuation_cost_basis(Decimal)、fx_rate_to_base(Option)、split_ratio(Decimal)、contract_multiplier(Decimal)、acquisition_date(Option)、snapshot_date(Option)、is_closed(bool)、close_date(Option)、disposal_proceeds(Option)、disposal_cost_basis(Option)、disposal_cost_basis_base(Option)、realized_pnl(Option)、realized_pnl_base(Option)、valuation_disposal_cost_basis(Option)、valuation_realized_pnl(Option)。

Derives: Debug, Clone, Serialize, Deserialize (camelCase)。

#### LotRepositoryTrait (lots/mod.rs:113)

持久化接口。

| 方法 | 签名 |
|------|------|
| replace_lots_for_account | async fn(&self, account_id, &[LotRecord]) -> Result<()> |
| get_open_lots_for_account | async fn(&self, account_id) -> Result<Vec<LotRecord>> |
| get_open_lots_for_account_asset | async fn(&self, account_id, asset_id) -> Result<Vec<LotRecord>> |
| get_all_open_lots | async fn(&self) -> Result<Vec<LotRecord>> |
| get_lots_as_of_date | async fn(&self, account_ids, date) -> Result<Vec<LotRecord>> |
| get_all_lots_for_account | async fn(&self, account_id) -> Result<Vec<LotRecord>> |
| get_lots_for_asset | async fn(&self, asset_id) -> Result<Vec<LotRecord>> |
| get_asset_lot_view | async fn(&self, asset_id, include_snapshot_positions: bool) -> Result<Vec<AssetLotView>> |
| get_all_lots | async fn(&self) -> Result<Vec<LotRecord>> |
| sync_lots_for_account | async fn(&self, account_id, &[LotRecord], &[LotClosure]) -> Result<()> |
| sync_lot_disposals_for_account | async fn(&self, account_id, &[String], &[LotDisposal], replace_all: bool) -> Result<()> |
| get_lot_disposals_for_account | async fn(&self, account_id) -> Result<Vec<LotDisposal>> |
| get_lot_disposals_for_accounts_in_date_range | async fn(&self, account_ids, start_date_exclusive, end_date_inclusive) -> Result<Vec<LotDisposal>> |
| get_lot_disposals_for_accounts_in_date_range_sync | fn(&self, account_ids, start_date_exclusive, end_date_inclusive) -> Result<Vec<LotDisposal>> |
| get_open_position_quantities | async fn(&self) -> Result<HashMap<String, Decimal>> |
| count_lots | fn(&self) -> Result<i64> |

Derives: Send + Sync (async_trait)。

#### 提取函数 (lots/mod.rs)

- `extract_lot_records(snapshot)` — 将快照中的内存批次转换为 LotRecord 列表
- `extract_lot_records_
- `extract_lot_records_with_cost_basis_method(snapshot, method)` — 同上，记录成本基础方法
- `lot_record_to_snapshot_lot(position_id, record)` — 从持久化记录重建内存 Lot
- `check_lot_quantity_consistency(snapshot, lot_records)` — 检查批次数量与持仓数量一致性，返回不匹配数

### 4.7 Health 模块 (health/)

#### Severity (health/model.rs:26)

枚举：Info、Warning、Error、Critical。Default = Info。SCREAMING_SNAKE_CASE 序列化。实现 PartialOrd 排序（Info < Warning < Error < Critical）。方法：as_str()。

#### HealthCategory (health/model.rs:62)

枚举：PriceStaleness、FxIntegrity、Classification、DataConsistency、AccountConfiguration、SettingsConfiguration。SCREAMING_SNAKE_CASE 序列化。方法：as_str()、label()。

#### FixAction (health/model.rs:119)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | 动作类型标识（如 "sync_prices"） |
| label | String | 按钮文本 |
| payload | Value | JSON 执行参数 |

构造函数：sync_prices(asset_ids)、migrate_legacy_classifications()、retry_sync(asset_ids)、rebuild_account_history(account_ids)。

#### NavigateAction (health/model.rs:178)

| 字段 | 类型 | Notes |
|------|------|-------|
| route | String | 路由路径 |
| query | Option<Value> | 可选查询参数 |
| label | String | 按钮文本 |

构造函数：to_holdings(filter)、to_activities(filter)、to_accounts()、to_taxonomies()、to_market_data()、to_general_settings()、to_connect()、to_asset_manual_quote(asset_id)、to_asset_snapshots(asset_id)、to_asset_activities(asset_id)、to_activity(activity_id)。

#### ActionRef (health/model.rs:413)

枚举：Fix { action: FixAction }、Navigate { action: NavigateAction }。serde(tag = "kind") 序列化。

#### DiagnosticAction (health/model.rs:431)

| 字段 | 类型 |
|------|------|
| primary | bool |
| action | ActionRef (flatten) |

#### Evidence (health/model.rs:442)

| 字段 | 类型 | Notes |
|------|------|-------|
| label | String | 短标签 |
| value | String | 证据值 |
| route | Option<String> | 可选深度链接 |

#### DiagnosticDomain (health/model.rs:476)

枚举：Unknown、AccountSetup、Ledger、MarketData、Fx、Classification、GeneratedData、PerformanceInputs。

#### DiagnosticLevel (health/model.rs:518)

枚举：Source、Generated、Workflow。

#### HealthImpact (health/model.rs:538)

| 字段 | 类型 | Notes |
|------|------|-------|
| affected_count | Option<u32> | |
| affected_mv_pct | Option<f64> | 占组合百分比 |
| amount | Option<f64> | |
| currency | Option<String> | |
| description | Option<String> | |

#### HealthEntityRef (health/model.rs:570)

| 字段 | 类型 |
|------|------|
| kind | String |
| id | String |
| label | Option<String> |
| route | Option<String> |

#### HealthDateRange (health/model.rs:603)

| 字段 | 类型 |
|------|------|
| start | String |
| end | String |

#### HealthDiagnostic (health/model.rs:621)

结构化诊断，解释健康问题的根因，含支持证据和有序修复动作。

| 字段 | 类型 | Notes |
|------|------|-------|
| fingerprint | String | 稳定标识，基于 code/domain/level/severity/entities 的 SHA-256 哈希 |
| domain | DiagnosticDomain | |
| level | DiagnosticLevel | |
| severity | Severity | |
| code | String | 根因代码（如 "MISSING_MARKET_QUOTE"） |
| title | String | 短标题 |
| explanation | String | 长解释 |
| impact | Option<HealthImpact> | |
| entities | Vec<HealthEntityRef> | |
| date | Option<String> | |
| date_range | Option<HealthDateRange> | |
| evidence | Vec<Evidence> | |
| actions | Vec<DiagnosticAction> | |

Builder 方法：new()、domain()、level()、severity()、impact()、entity()、date()、date_range()、fingerprint()、evidence()、fix()、navigate()。computed_fingerprint() 方法基于 code/domain/level/severity/entities/date/date_range 生成稳定哈希（忽略说明文本、证据、动作）。

#### HealthIssue (health/model.rs:855)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | 稳定标识符 |
| severity | Severity | |
| category | HealthCategory | |
| title | String | 短标题（max 40 chars） |
| message | String | 长说明（max 150 chars） |
| code | Option<String> | 前端翻译代码 |
| params | HashMap<String, Value> | 翻译插值参数 |
| affected_count | u32 | |
| affected_mv_pct | Option<f64> | 0.0 ~ 1.0 |
| fix_action | Option<FixAction> | 自动修复动作 |
| navigate_action | Option<NavigateAction> | 导航动作 |
| details | Option<String> | |
| affected_items | Option<Vec<AffectedItem>> | |
| diagnostics | Option<Vec<HealthDiagnostic>> | 结构化诊断 |
| data_hash | String | 数据哈希，用于检测变更 |
| timestamp | DateTime<Utc> | |

使用 HealthIssueBuilder 构建。build() 方法：自动生成 data_hash（基于 diagnostics 指纹聚合）、从 diagnostics 提升 primary fix/navigate action、聚合 severity。

#### HealthIssueBuilder (health/model.rs:929)

Builder 方法：id()、severity()、category()、title()、message()、code()、param()、affected_count()、affected_mv_pct()、fix_action()、navigate_action()、details()、affected_items()、diagnostics()、data_hash()、build()。

#### AffectedItem (health/model.rs:198)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | 唯一标识 |
| name | String | 显示名称 |
| symbol | Option<String> | 股票代码徽章 |
| route | Option<String> | 导航路线 |

构造函数：asset_with_name(id, symbol, name)、asset(id, symbol)、asset_market_data(id, symbol)、simple(id, name)、account(id, name)、activity(id, name)。

#### HealthStatus (health/model.rs:1198)

| 字段 | 类型 | Notes |
|------|------|-------|
| overall_severity | Severity | 最高严重级别 |
| issue_counts | HashMap<Severity, u32> | 各级别计数 |
| issues | Vec<HealthIssue> | |
| checked_at | DateTime<Utc> | |
| is_stale | bool | 缓存超过 5 分钟为 stale |

方法：healthy()、from_issues(issues)、total_count()、issues_by_severity()、issues_by_category()、mark_stale()。

#### HealthConfig (health/model.rs:1291)

| 字段 | 类型 | 默认值 | Notes |
|------|------|--------|-------|
| price_stale_warning_hours | u32 | 48 | |
| price_stale_critical_hours | u32 | 72 | |
| fx_stale_warning_hours | u32 | 24 | |
| fx_stale_critical_hours | u32 | 72 | |
| mv_escalation_threshold | f64 | 0.30 | 30% |
| classification_warn_threshold | f64 | 0.05 | 5% |

#### IssueDismissal (health/model.rs:1334)

| 字段 | 类型 |
|------|------|
| issue_id | String |
| dismissed_at | DateTime<Utc> |
| data_hash | String |

#### HealthCheck trait (health/traits.rs:108)

| 方法 | 签名 |
|------|------|
| id | fn(&self) -> &'static str |
| category | fn(&self) -> HealthCategory |
| run | async fn(&self, ctx: &HealthContext) -> Result<Vec<HealthIssue>> |

#### HealthContext (health/traits.rs:24)

| 字段 | 类型 |
|------|------|
| config | HealthConfig |
| base_currency | String |
| now | DateTime<Utc> |
| total_portfolio_value | f64 |

#### HealthDismissalStore trait (health/traits.rs:141)

方法：save_dismissal()、remove_dismissal()、get_dismissals()、get_dismissal()、clear_all()。

#### HealthServiceTrait (health/traits.rs:205)

主要方法：run_checks(base_currency)、run_checks_with_data()（14 个参数，支持注入所有数据）、get_cached_status()、dismiss_issue()、restore_issue()、get_dismissed_ids()、execute_fix()、get_config()、update_config()、clear_cache()、run_full_checks()（11 个服务参数）。

#### HealthError (health/errors.rs:10)

变体：CheckFailed(check_id, message)、FixActionFailed(action_id, message)、UnknownFixAction、InvalidFixPayload(action_id, message)、InvalidConfig、IssueNotFound。Derives: Error, Debug。From<HealthError> for crate::errors::Error 实现。

#### Checks 数据模型

##### AssetHoldingInfo (price_staleness.rs:21)

| 字段 | 类型 | Notes |
|------|------|-------|
| asset_id | String | |
| symbol | String | |
| name | Option<String> | |
| exchange_mic | Option<String> | 交易所 MIC |
| market_value | f64 | 基础货币市值 |
| uses_market_pricing | bool | 是否使用市场定价 |

##### FxPairInfo (fx_integrity.rs:17)

| 字段 | 类型 | Notes |
|------|------|-------|
| pair_id | String | 如 "EUR:USD" |
| from_currency | String | |
| to_currency | String | |
| affected_mv | f64 | 基础货币受影响的市值 |
| latest_quote_time | Option<DateTime<Utc>> | |

##### UnclassifiedAssetInfo (classification.rs:20)

| 字段 | 类型 |
|------|------|
| asset_id | String |
| symbol | String |
| name | Option<String> |
| market_value | f64 |
| missing_taxonomy | String |

##### LegacyAssetInfo (classification.rs:35)

| 字段 | 类型 |
|------|------|
| asset_id | String |
| symbol | String |
| name | Option<String> |

##### LegacyMigrationInfo (classification.rs:45)

| 字段 | 类型 |
|------|------|
| assets_needing_migration | Vec<LegacyAssetInfo> |
| assets_already_migrated | i32 |

##
- `extract_lot_records_with_cost_basis_method(snapshot, method)` — 同上，记录成本基础方法
- `lot_record_to_snapshot_lot(position_id, record)` — 从持久化记录重建内存 Lot
- `check_lot_quantity_consistency(snapshot, lot_records)` — 检查批次数量与持仓数量一致性，返回不匹配数

### 4.7 Health 模块 (health/)

#### Severity (health/model.rs:26)

枚举：Info、Warning、Error、Critical。Default = Info。SCREAMING_SNAKE_CASE 序列化。实现 PartialOrd 排序（Info < Warning < Error < Critical）。方法：as_str()。

#### HealthCategory (health/model.rs:62)

枚举：PriceStaleness、FxIntegrity、Classification、DataConsistency、AccountConfiguration、SettingsConfiguration。SCREAMING_SNAKE_CASE 序列化。方法：as_str()、label()。

#### FixAction (health/model.rs:119)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | 动作类型标识（如 "sync_prices"） |
| label | String | 按钮文本 |
| payload | Value | JSON 执行参数 |

构造函数：sync_prices(asset_ids)、migrate_legacy_classifications()、retry_sync(asset_ids)、rebuild_account_history(account_ids)。

#### NavigateAction (health/model.rs:178)

| 字段 | 类型 | Notes |
|------|------|-------|
| route | String | 路由路径 |
| query | Option<Value> | 可选查询参数 |
| label | String | 按钮文本 |

构造函数：to_holdings(filter)、to_activities(filter)、to_accounts()、to_taxonomies()、to_market_data()、to_general_settings()、to_connect()、to_asset_manual_quote(asset_id)、to_asset_snapshots(asset_id)、to_asset_activities(asset_id)、to_activity(activity_id)。

#### ActionRef (health/model.rs:413)

枚举：Fix { action: FixAction }、Navigate { action: NavigateAction }。serde(tag = "kind") 序列化。

#### DiagnosticAction (health/model.rs:431)

| 字段 | 类型 |
|------|------|
| primary | bool |
| action | ActionRef (flatten) |

#### Evidence (health/model.rs:442)

| 字段 | 类型 | Notes |
|------|------|-------|
| label | String | 短标签 |
| value | String | 证据值 |
| route | Option<String> | 可选深度链接 |

#### DiagnosticDomain (health/model.rs:476)

枚举：Unknown、AccountSetup、Ledger、MarketData、Fx、Classification、GeneratedData、PerformanceInputs。

#### DiagnosticLevel (health/model.rs:518)

枚举：Source、Generated、Workflow。

#### HealthImpact (health/model.rs:538)

| 字段 | 类型 | Notes |
|------|------|-------|
| affected_count | Option<u32> | |
| affected_mv_pct | Option<f64> | 占组合百分比 |
| amount | Option<f64> | |
| currency | Option<String> | |
| description | Option<String> | |

#### HealthEntityRef (health/model.rs:570)

| 字段 | 类型 |
|------|------|
| kind | String |
| id | String |
| label | Option<String> |
| route | Option<String> |

#### HealthDateRange (health/model.rs:603)

| 字段 | 类型 |
|------|------|
| start | String |
| end | String |

#### HealthDiagnostic (health/model.rs:621)

结构化诊断，解释健康问题的根因，含支持证据和有序修复动作。

| 字段 | 类型 | Notes |
|------|------|-------|
| fingerprint | String | 稳定标识，基于 code/domain/level/severity/entities 的 SHA-256 哈希 |
| domain | DiagnosticDomain | |
| level | DiagnosticLevel | |
| severity | Severity | |
| code | String | 根因代码（如 "MISSING_MARKET_QUOTE"） |
| title | String | 短标题 |
| explanation | String | 长解释 |
| impact | Option<HealthImpact> | |
| entities | Vec<HealthEntityRef> | |
| date | Option<String> | |
| date_range | Option<HealthDateRange> | |
| evidence | Vec<Evidence> | |
| actions | Vec<DiagnosticAction> | |

Builder 方法：new()、domain()、level()、severity()、impact()、entity()、date()、date_range()、fingerprint()、evidence()、fix()、navigate()。computed_fingerprint() 方法基于 code/domain/level/severity/entities/date/date_range 生成稳定哈希（忽略说明文本、证据、动作）。

#### HealthIssue (health/model.rs:855)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | 稳定标识符 |
| severity | Severity | |
| category | HealthCategory | |
| title | String | 短标题（max 40 chars） |
| message | String | 长说明（max 150 chars） |
| code | Option<String> | 前端翻译代码 |
| params | HashMap<String, Value> | 翻译插值参数 |
| affected_count | u32 | |
| affected_mv_pct | Option<f64> | 0.0 ~ 1.0 |
| fix_action | Option<FixAction> | 自动修复动作 |
| navigate_action | Option<NavigateAction> | 导航动作 |
| details | Option<String> | |
| affected_items | Option<Vec<AffectedItem>> | |
| diagnostics | Option<Vec<HealthDiagnostic>> | 结构化诊断 |
| data_hash | String | 数据哈希，用于检测变更 |
| timestamp | DateTime<Utc> | |

使用 HealthIssueBuilder 构建。build() 方法：自动生成 data_hash（基于 diagnostics 指纹聚合）、从 diagnostics 提升 primary fix/navigate action、聚合 severity。

#### HealthIssueBuilder (health/model.rs:929)

Builder 方法：id()、severity()、category()、title()、message()、code()、param()、affected_count()、affected_mv_pct()、fix_action()、navigate_action()、details()、affected_items()、diagnostics()、data_hash()、build()。

#### AffectedItem (health/model.rs:198)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | 唯一标识 |
| name | String | 显示名称 |
| symbol | Option<String> | 股票代码徽章 |
| route | Option<String> | 导航路线 |

构造函数：asset_with_name(id, symbol, name)、asset(id, symbol)、asset_market_data(id, symbol)、simple(id, name)、account(id, name)、activity(id, name)。

#### HealthStatus (health/model.rs:1198)

| 字段 | 类型 | Notes |
|------|------|-------|
| overall_severity | Severity | 最高严重级别 |
| issue_counts | HashMap<Severity, u32> | 各级别计数 |
| issues | Vec<HealthIssue> | |
| checked_at | DateTime<Utc> | |
| is_stale | bool | 缓存超过 5 分钟为 stale |

方法：healthy()、from_issues(issues)、total_count()、issues_by_severity()、issues_by_category()、mark_stale()。

#### HealthConfig (health/model.rs:1291)

| 字段 | 类型 | 默认值 | Notes |
|------|------|--------|-------|
| price_stale_warning_hours | u32 | 48 | |
| price_stale_critical_hours | u32 | 72 | |
| fx_stale_warning_hours | u32 | 24 | |
| fx_stale_critical_hours | u32 | 72 | |
| mv_escalation_threshold | f64 | 0.30 | 30% |
| classification_warn_threshold | f64 | 0.05 | 5% |

#### IssueDismissal (health/model.rs:1334)

| 字段 | 类型 |
|------|------|
| issue_id | String |
| dismissed_at | DateTime<Utc> |
| data_hash | String |

#### HealthCheck trait (health/traits.rs:108)

| 方法 | 签名 |
|------|------|
| id | fn(&self) -> &'static str |
| category | fn(&self) -> HealthCategory |
| run | async fn(&self, ctx: &HealthContext) -> Result<Vec<HealthIssue>> |

#### HealthContext (health/traits.rs:24)

| 字段 | 类型 |
|------|------|
| config | HealthConfig |
| base_currency | String |
| now | DateTime<Utc> |
| total_portfolio_value | f64 |

#### HealthDismissalStore trait (health/traits.rs:141)

方法：save_dismissal()、remove_dismissal()、get_dismissals()、get_dismissal()、clear_all()。

#### HealthServiceTrait (health/traits.rs:205)

主要方法：run_checks(base_currency)、run_checks_with_data()（14 个参数，支持注入所有数据）、get_cached_status()、dismiss_issue()、restore_issue()、get_dismissed_ids()、execute_fix()、get_config()、update_config()、clear_cache()、run_full_checks()（11 个服务参数）。

#### HealthError (health/errors.rs:10)

变体：CheckFailed(check_id, message)、FixActionFailed(action_id, message)、UnknownFixAction、InvalidFixPayload(action_id, message)、InvalidConfig、IssueNotFound。Derives: Error, Debug。From<HealthError> for crate::errors::Error 实现。

#### Checks 数据模型

##### AssetHoldingInfo (price_staleness.rs:21)

| 字段 | 类型 | Notes |
|------|------|-------|
| asset_id | String | |
| symbol | String | |
| name | Option<String> | |
| exchange_mic | Option<String> | 交易所 MIC |
| market_value | f64 | 基础货币市值 |
| uses_market_pricing | bool | 是否使用市场定价 |

##### FxPairInfo (fx_integrity.rs:17)

| 字段 | 类型 | Notes |
|------|------|-------|
| pair_id | String | 如 "EUR:USD" |
| from_currency | String | |
| to_currency | String | |
| affected_mv | f64 | 基础货币受影响的市值 |
| latest_quote_time | Option<DateTime<Utc>> | |

##### UnclassifiedAssetInfo (classification.rs:20)

| 字段 | 类型 |
|------|------|
| asset_id | String |
| symbol | String |
| name | Option<String> |
| market_value | f64 |
| missing_taxonomy | String |

##### LegacyAssetInfo (classification.rs:35)

| 字段 | 类型 |
|------|------|
| asset_id | String |
| symbol | String |
| name | Option<String> |

##### LegacyMigrationInfo (classification.rs:45)

| 字段 | 类型 |
|------|------|
| assets_needing_migration | Vec<LegacyAssetInfo> |
| assets_already_migrated | i32 |

##### MigrationStatus (fixes/classification_migration.rs:17)

| 字段 | 类型 |
|------|------|
| needed | bool |
| assets_with_legacy_data | i32 |
| assets_already_migrated | i32 |

##### MigrationResult (fixes/classification_migration.rs:29)

| 字段 | 类型 |
|------|------|
| sectors_migrated | i32 |
| countries_migrated | i32 |
| assets_processed | i32 |
| errors | Vec<String> |

## 5. 附属 Crate (Subsidiary Crates)

### 5.1 Spending 模块 (spending/)

#### SpendingError (spending/src/error.rs:8)

枚举：EventTypeInUse { count }、InvalidEventRange、GlobalRuleHasAccount、InvalidInput { message }、NotFound { entity, id }。Derives: Error, Debug。

#### SpendingSettings (spending/src/settings/model.rs:6)

| 字段 | 类型 | Notes |
|------|------|-------|
| enabled | bool | 运行时开关 |
| account_ids | Vec<String> | 纳入支出追踪的账户列表 |

#### SpendingSettingsUpdate (spending/src/settings/model.rs:11)

所有字段 Option：enabled、account_ids。

#### CashActivityFilter (spending/src/cash_activities/model.rs:10)

| 字段 | 类型 |
|------|------|
| account_ids | Option<Vec<String>> |
| start_date | Option<String> |
| end_date | Option<String> |
| activity_types | Option<Vec<String>> |

#### CashActivityStatusFilter (spending/src/cash_activities/model.rs:25)

枚举：All（默认）、NeedsReview、Uncategorized、Categorized。

#### CashActivitySortField (spending/src/cash_activities/model.rs:36)

枚举：Date（默认）、Amount。

#### SortDirection (spending/src/cash_activities/model.rs:45)

枚举：Asc、Desc（默认）。

#### CashFlowBucket (spending/src/cash_activities/model.rs:96)

枚举：Spending、Income、Saving、Neutral。

#### TransferLinkStatus (spending/src/cash_activities/model.rs:103)

枚举：Linked、Unlinked、Invalid。

#### CashActivity (spending/src/cash_activities/model.rs:122)

| 字段 | 类型 | Notes |
|------|------|-------|
| activity | Activity (flatten) | 复用 core 的 Activity |
| cash_flow_bucket | CashFlowBucket | |
| assignments | Vec<ActivityTaxonomyAssignment> | |
| splits | Vec<ActivitySplit> | |
| event_id | Option<String> | |
| transfer_link_status | Option<TransferLinkStatus> | |

#### CashActivitySearchRequest (spending/src/cash_activities/model.rs:55)

搜索/筛选请求，含 search、account_ids、activity_types、category_ids、subcategory_ids、event_ids、status、start_date、end_date、min_amount、max_amount、sort_by、sort_dir、offset、limit（默认50）。

#### CashActivitySearchResponse (spending/src/cash_activities/model.rs:148)

| 字段 | 类型 |
|------|------|
| items | Vec<CashActivity> |
| total_count | usize |

#### ActivityTaxonomyAssignment (spending/src/activity_assignments/model.rs:6)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | |
| activity_id | String | |
| taxonomy_id | String | |
| category_id | String | |
| weight | i32 | 基点，10000 = 100% |
| source | String | "manual" / "rule" / "import" |
| created_at | NaiveDateTime | |
| updated_at | NaiveDateTime | |

#### NewActivityTaxonomyAssignment (spending/src/activity_assignments/model.rs:21)

id、activity_id、taxonomy_id、category_id、weight（默认10000）、source（默认"manual"）。

#### ActivityEvent (spending/src/activity_events/model.rs:7)

| 字段 | 类型 |
|------|------|
| activity_id | String |
| event_id | String |
| created_at | NaiveDateTime |
| updated_at | NaiveDateTime |

#### ActivitySplit (spending/src/activity_splits/model.rs:7)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | |
| activity_id | String | |
| taxonomy_id | String | |
| category_id | String | |
| amount | Decimal | |
| note | Option<String> | |
| sort_order | i32 | |
| created_at | NaiveDateTime | |
| updated_at | NaiveDateTime | |

#### BudgetGroup (spending/src/budget/model.rs:6)

| 字段 | 类型 |
|------|------|
| id, name, key | String |
| color, icon | Option<String> |
| sort_order | i32 |
| is_system | bool |
| created_at, updated_at | NaiveDateTime |

#### BudgetTarget (spending/src/budget/model.rs:112)

| 字段 | 类型 |
|------|------|
| id, period_key | String |
| target_type | BudgetTargetType (Category / GroupBuffer) |
| taxonomy_id, category_id, group_id | Option<String> |
| amount | String (Decimal) |
| created_at, updated_at | NaiveDateTime |

#### BudgetRolloverSetting (spending/src/budget/model.rs:156)

| 字段 | 类型 |
|------|------|
| id | String |
| target_type | BudgetRolloverTargetType (Category / Group) |
| taxonomy_id, category_id, group_id | Option<String> |
| enabled | bool |
| start_month, starting_balance | String |
| created_at, updated_at | NaiveDateTime |

#### BudgetSnapshot (spending/src/budget/model.rs:257)

| 字段 | 类型 |
|------|------|
| state | BudgetSnapshotState (groups, group_assignments, targets, rollover_settings) |
| computed | BudgetSnapshotComputed (currency, period_key, group_rows, ungrouped_rows, income_rows, totals) |

#### CategorizationRule (spending/src/categorization_rules/model.rs:38)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | |
| name | String | |
| pattern | String | 匹配模式文本 |
| match_type | RuleMatchType | Contains / StartsWith / Exact / Regex |
| taxonomy_id, category_id | Option<String> | |
| activity_type | Option<String> | |
| priority | i32 | |
| is_global | bool | |
| account_id | Option<String> | |
| preset_id, preset_rule_key, preset_version | Option<String> | 预设来源 |
| preset_modified | bool | 用户是否编辑过预设规则 |
| created_at, updated_at | NaiveDateTime | |

#### RuleMatchType (spending/src/categorization_rules/model.rs:6)

枚举：Contains、StartsWith、Exact、Regex。方法：as_str()、try_parse()、parse()。

#### ReportRequest (spending/src/analytics/model.rs:7)

| 字段 | 类型 |
|------|------|
| start_date | String (RFC3339) |
| end_date | String (RFC3339) |
| account_ids | Option<Vec<String>> |

#### PeriodSummary (spending/src/analytics/model.rs:16)

| 字段 | 类型 | Notes |
|------|------|-------|
| income | f64 | |
| outflow | f64 | 消费支出 |
| saved | f64 | 储蓄转移 |
| net | f64 | income - outflow - saved |
| count | usize | |

#### MonthlyReport (spending/src/analytics/model.rs:69)

| 字段 | 类型 |
|------|------|
| current, prior | PeriodSummary |
| spending_breakdown, income_breakdown, savings_breakdown | Vec<CategoryBreakdownRow> |
| by_day | Vec<DayBucket> |
| by_day_by_category | Vec<DayCategoryBucket> |

#### SpendingSummary (spending/src/analytics/model.rs:149)

| 字段 | 类型 |
|------|------|
| period | String |
| by_month | HashMap<String, f64> |
| by_category | HashMap<String, CategorySpending> |
| by_subcategory | HashMap<String, SubcategorySpending> |
| by_account | HashMap<String, f64> |
| by_month_by_category, by_month_by_subcategory | HashMap<String, HashMap<String, f64>> |
| total_spending, monthly_average | f64 |
| currency | String |
| transaction_count | usize |
| yoy_growth | Option<f64> |

#### SpendingInsight (spending/src/insight/model.rs:213)

完整洞察响应，含 period/prior 元数据、headline、groups（Vec<GroupInsight>）、uncategorized、income_breakdown、savings_breakdown、by_day、by_day_by_category、by_month。支持 foreign_currencies 和 native_outflow_by_currency。

#### Headline (spending/src/insight/model.rs:103)

| 字段 | 类型 | Notes |
|------|------|-------|
| spent, income, saved | f64 | |
| net_cashflow | f64 | income - spent - saved |
| budget | f64 | 预算总额 |
| remaining | f64 | |
| prior_spent | f64 | |
| delta_vs_prior_pct | Option<f64> | |
| pace | PaceState | 日均消费 + 预测 |
| status | HealthStatus | OnTrack / Approaching / Over / CashflowNegative |

### 5.2 Connect 模块 (connect/)

#### BrokerAccount (connect/src/broker/models.rs:54)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | Option<String> | UUID（可为 null） |
| name | Option<String> | |
| account_number | Option<String> | |
| account_type | Option<String> | TFSA, RRSP, MARGIN 等 |
| currency | Option<String> | |
| balance | Option<BrokerAccountBalance> | |
| meta | Option<Value> | |
| owner | Option<AccountOwner> | 共享账户 |
| brokerage_authorization | Option<String> | 旧版 |
| institution_name | Option<String> | 旧版 |
| created_date | Option<String> | 旧版 |
| sync_status | Option<BrokerAccountSyncStatus> | 旧版 |
| status | Option<String> | open/closed/archived |
| raw_type | Option<String> | 旧版 |
| is_paper | bool | |
| sync_enabled | bool | 默认 true |
| shared_with_household | bool | |

方法：get_currency()、get_account_type()、display_name()、to_meta_json()。

#### AccountUniversalActivity (connect/src/broker/models.rs:331)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | Option<String> | |
| symbol | Option<AccountUniversalActivitySymbol> | |
| option_symbol | Option<AccountUniversalActivityOptionSymbol> | |
| price, units, amount | Option<f64> | |
| currency | Option<AccountUniversalActivityCurrency> | |
| activity_type | Option<String> | BUY/SELL/DIVIDEND |
| subtype | Option<String> | DRIP/STAKING_REWARD |
| raw_type | Option<String> | |
| trade_date, settlement_date | Option<String> | |
| fee, fx_rate | Option<f64> | |
| external_reference_id | Option<String> | 去重 |
| source_group_id | Option<String> | 多腿交易分组 |
| mapping_metadata | Option<MappingMetadata> | |
| needs_review | bool | |

#### BrokerConnection (connect/src/broker/models.rs:151)

| 字段 | 类型 |
|------|------|
| id | String |
| brokerage | Option<BrokerConnectionBrokerage> |
| connection_type | Option<String> |
| status | Option<String> |
| disabled | bool |
| disabled_date | Option<String> |
| updated_at | Option<String> |
| name | Option<String> |

#### BrokerBrokerage (connect/src/broker/models.rs:128)

| 字段 | 类型 |
|------|------|
| id, slug, name, display_name, url | Option<String> |
| enabled | bool |

#### SyncResult (connect/src/broker/models.rs:636)

| 字段 | 类型 |
|------|------|
| success | bool |
| message | String |
| connections_synced | Option<SyncConnectionsResponse> |
| accounts_synced | Option<SyncAccountsResponse> |
| activities_synced | Option<SyncActivitiesResponse> |
| holdings_synced | Option<SyncHoldingsResponse> |
| new_accounts | Option<Vec<NewAccountInfo>> |

#### HoldingsPosition (connect/src/broker/models.rs:494)

| 字段 | 类型 |
|------|------|
| symbol | Option<HoldingsSymbol> |
| units, price, open_pnl, average_purchase_price | Option<f64> |
| currency | Option<HoldingsCurrency> |
| contract_multiplier | Option<f64> |
| cash_equivalent | Option<bool> |

#### HoldingsOptionPosition (connect/src/broker/models.rs:538)

| 字段 | 类型 |
|------|------|
| option_symbol | Option<HoldingsOptionSymbol> |
| symbol | Option<HoldingsOptionSymbolWrapper>（旧版） |
| units, price, average_purchase_price | Option<f64> |
| currency | Option<HoldingsCurrency> |

方法：resolved_option_symbol() — 优先使用顶层 option_symbol，回退到旧版嵌套。

#### SubscriptionPlan (connect/src/broker/models.rs:882)

| 字段 | 类型 |
|------|------|
| id, name | String |
| tagline, description | String |
| pricing | PlanPricing (monthly, yearly) |
| limits | PlanLimits (household_size, institution_connections, devices) |
| features | Vec<String> |
| is_available, is_coming_soon | bool |

#### PlanLimitValue (connect/src/broker/models.rs:866)

枚举：Limited(i32) / Unlimited(String)。方法：display()。

#### BrokerSyncState (connect/src/broker_ingest/models.rs:19)

| 字段 | 类型 | Notes |
|------|------|-------|
| account_id | String | |
| provider | String | |
| checkpoint_json | Option<Value> | 提供商特定检查点 |
| last_attempted_at | Option<DateTime<Utc>> | |
| last_successful_at | Option<DateTime<Utc>> | |
| last_error | Option<String> | |
| last_run_id | Option<String> | |
| sync_status | SyncStatus | Idle / Running / NeedsReview / Failed |
| created_at, updated_at | DateTime<Utc> | |

方法：new()、get_checkpoint()、set_checkpoint()、start_sync()、complete_sync()、fail_sync()。

#### ImportRun (connect/src/broker_ingest/models.rs:141)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | UUID v4 |
| account_id | String | |
| source_system | String | |
| run_type | ImportRunType | Sync / Import |
| mode | ImportRunMode | Initial / Incremental / Backfill / Repair |
| status | ImportRunStatus | Running / Applied / NeedsReview / Failed / Cancelled |
| started_at | DateTime<Utc> | |
| finished_at | Option<DateTime<Utc>> | |
| review_mode | ReviewMode | Never / Always / IfWarnings |
| applied_at | Option<DateTime<Utc>> | |
| checkpoint_in, checkpoint_out | Option<Value> | |
| summary | Option<ImportRunSummary> | |
| warnings | Option<Vec<String>> | |
| error | Option<String> | |
| created_at, updated_at | DateTime<Utc> | |

方法：new()、complete()、fail()、mark_needs_review()。

#### ImportRunSummary (connect/src/broker_ingest/models.rs:163)

| 字段 | 类型 |
|------|------|
| fetched, inserted, updated, skipped, warnings, errors, removed, assets_created | u32 |

### 5.3 Device Sync 模块 (device-sync/)

#### DevicePlatform (device-sync/src/types.rs:36)

枚举：Ios、Android、Mac、Windows、Linux、Web。方法：detect() — 编译期检测。

#### TrustState (device-sync/src/types.rs:81)

枚举：Untrusted、Trusted、Revoked。

#### EnrollDeviceResponse (device-sync/src/types.rs:128)

serde(tag = "mode") 判别联合：
- Bootstrap { device_id, e2ee_key_version }
- Pair { device_id, e2ee_key_version, require_sas, pairing_ttl_seconds, trusted_devices }
- Ready { device_id, e2ee_key_version, trust_state }

#### Device (device-sync/src/types.rs:154)

| 字段 | 类型 |
|------|------|
| id | String |
| user_id | String |
| display_name | String |
| platform | String |
| device_public_key | Option<String> |
| trust_state | TrustState |
| trusted_key_version | Option<f64> |
| os_version, app_version | Option<String> |
| last_seen_at | Option<String> |
| created_at | String |

#### InitializeKeysResult (device-sync/src/types.rs:212)

serde(tag = "mode") 判别联合：
- Bootstrap { challenge, nonce, key_version }
- PairingRequired { e2ee_key_version, require_sas, pairing_ttl_seconds, trusted_devices }
- Ready { e2ee_key_version }

#### SyncPushEventRequest (device-sync/src/types.rs:506)

| 字段 | 类型 |
|------|------|
| event_id, device_id | String |
| event_type | String |
| entity | SyncEntity |
| entity_id | String |
| client_timestamp | String |
| payload | String |
| payload_key_version | i32 |

#### SyncEvent (device-sync/src/types.rs:543)

| 字段 | 类型 |
|------|------|
| event_id, device_id, event_type | String |
| entity | String（远程字符串，通过 sync_entity_from_remote 转换） |
| entity_id, client_timestamp | String |
| payload | String |
| payload_key_version | i32 |
| seq | i64 |
| user_id, team_id, server_timestamp | String |

#### SyncPullResponse (device-sync/src/types.rs:614)

| 字段 | 类型 |
|------|------|
| from, to | i64 |
| next_cursor | i64 |
| has_more | bool |
| events | Vec<SyncEvent> |
| gc_watermark | Option<i64> |
| latest_snapshot_seq | Option<i64> |

#### SyncPushResponse (device-sync/src/types.rs:534)

| 字段 | 类型 |
|------|------|
| accepted | Vec<SyncPushResultItem> |
| duplicate | Vec<SyncPushResultItem> |
| server_cursor | i64 |

#### SnapshotLatestResponse (device-sync/src/types.rs:653)

| 字段 | 类型 |
|------|------|
| snapshot_id | String |
| schema_version | i32 |
| covers_tables | Vec<String> |
| oplog_seq | i64 |
| size_bytes | i64 |
| checksum | String |
| created_at | String |

#### SnapshotUploadHeaders (device-sync/src/types.rs:679)

| 字段 | 类型 |
|------|------|
| event_id | Option<String> |
| schema_version | i32 |
| covers_tables | Vec<String> |
| size_bytes | i64 |
| checksum | String |
| metadata_payload | String |
| payload_key_version | i32 |
| base_seq | Option<i64> |

#### ReconcileReadyStateResponse (device-sync/src/types.rs:700)

| 字段 | 类型 |
|------|------|
| action | String（NOOP / PULL_TAIL / BOOTSTRAP_SNAPSHOT / WAIT_SNAPSHOT） |
| cursor | Option<i64> |
| latest_snapshot | Option<SyncLatestSnapshotRef> |

#### DeviceSyncError (device-sync/src/error.rs:42)

枚举变体：Http(reqwest::Error)、Json(serde_json::Error)、Api { status, code, message, details }、InvalidRequest(String)、Auth(String)。方法：api()、api_structured()、invalid_request()、auth()、status_code()、error_code()、is_integrity_error()、is_stale_cursor()、retry_class()、is_snapshot_id_validation_error()。

#### ApiRetryClass (device-sync/src/error.rs:10)

枚举：Retryable、Permanent、ReauthRequired。

### 5.4 AI 模块 (ai/)

#### ChatThread (ai/src/types.rs:236)

| 字段 | 类型 | Notes |
|------|------|-------|
| id | String | UUID v4 |
| title | Option<String> | |
| is_pinned | bool | |
| tags | Vec<String> | |
| config | Option<ChatThreadConfig> | 模型/模板/工具快照 |
| created_at | DateTime<Utc> | |
| updated_at | DateTime<Utc> | |

方法：new()、with_config()、with_id()、get_tools_allowlist()。

#### ChatThreadConfig (ai/src/types.rs:128)

| 字段 | 类型 | Notes |
|------|------|-------|
| schema_version | u32 | 版本 1 |
| provider_id | String | "openai" / "anthropic" |
| model_id | String | "gpt-4o" / "claude-3-sonnet" |
| prompt_template_id | String | |
| prompt_version | String | |
| locale | Option<String> | |
| detail_level | Option<String> | |
| tools_allowlist | Option<Vec<String>> | 默认使用 DEFAULT_TOOLS_ALLOWLIST |

#### ChatMessage (ai/src/types.rs:358)

| 字段 | 类型 |
|------|------|
| id | String |
| thread_id | String |
| role | ChatMessageRole (User / Assistant / System / Tool) |
| content | ChatMessageContent |
| created_at | DateTime<Utc> |

#### ChatMessageContent (ai/src/types.rs:409)

| 字段 | 类型 |
|------|------|
| schema_version | u32 |
| parts | Vec<ChatMessagePart> |
| truncated | bool |

#### ChatMessagePart (ai/src/types.rs:459)

serde(tag = "type") 枚举：System { content }、Text { content }、Reasoning { content }、ToolCall { tool_call_id, name, arguments }、ToolResult { tool_call_id, success, data, meta, error }、Error { code, message }。

#### AiStreamEvent (ai/src/types.rs:712)

serde(tag = "type") 枚举变体：

| 变体 | 字段 | Notes |
|------|------|-------|
| System | thread_id, run_id, message_id | 流开始的元数据 |
| TextDelta | thread_id, run_id, message_id, delta | 文本增量 |
| ReasoningDelta | thread_id, run_id, message_id, delta | 推理增量 |
| ToolCall | thread_id, run_id, message_id, tool_call | 工具调用请求 |
| ToolResult | thread_id, run_id, message_id, result | 工具执行结果 |
| Error | thread_id, run_id, message_id, code, message | |
| Done | thread_id, run_id, message_id, message, usage | 终端事件 |
| ThreadTitleUpdated | thread_id, run_id, title | 自动生成标题 |

#### ToolResult (ai/src/types.rs:585)

| 字段 | 类型 |
|------|------|
| data | Value |
| meta | HashMap<String, Value> |

方法：ok()、empty()、with_meta()、with_truncation()、with_duration_ms()、with_account_scope()、with_count()、to_result_data()、to_llm_string()。

#### SendMessageRequest (ai/src/types.rs:923)

| 字段 | 类型 | Notes |
|------|------|-------|
| thread_id | Option<String> | |
| content | String | |
| config | Option<ChatModelConfig> | |
| provider_id | Option<String> | 已废弃 |
| model_id | Option<String> | 已废弃 |
| allowed_tools | Option<Vec<String>> | |
| parent_message_id | Option<String> | 编辑操作 |
| attachments | Option<Vec<MessageAttachment>> | |

#### AiError (ai/src/error.rs:8)

枚举变体：InvalidInput、MissingApiKey(String)、Provider(String)、ToolNotFound(String)、ToolNotAllowed(String)、ToolExecutionFailed(String)、ThreadNotFound(String)、InvalidCursor(String)、Core(CoreError)、Internal(String)。方法：code() 返回字符串标识。

#### ChatRepositoryTrait (ai/src/types.rs:550)

方法：create_thread、get_thread、list_threads、list_threads_paginated、update_thread、delete_thread、create_message、get_message、get_messages_by_thread、update_message、add_tag、remove_tag、get_tags。

### 5.5 Agent Tools 模块 (agent-tools/)

#### AgentTool trait (agent-tools/src/tool.rs:56)

| 方法 | 签名 | Notes |
|------|------|-------|
| name | fn(&self) -> &'static str | 稳定标识 |
| description | fn(&self) -> &'static str | 模型可见描述 |
| input_schema | fn(&self) -> Value | JSON Schema |
| required_scopes | fn(&self) -> &'static [AgentScope] | |
| access_level | fn(&self) -> AgentToolAccess | Read / Draft / Write / Suggest |
| sanitize_args_for_audit | fn(&self, args) -> Value | 默认 identity |
| call | async fn(&self, env, args) -> Result | 执行 |

#### AgentToolAccess (agent-tools/src/tool.rs:11)

枚举：Read、Draft、Write、Suggest。

#### AgentToolError (agent-tools/src/tool.rs:37)

枚举变体：InvalidArgs(serde_json::Error)、InvalidInput(String)、ExecutionFailed(String)、NotFound(String)、ScopeDenied { tool, missing }。

#### AgentScope (agent-tools/src/scope.rs:13)

枚举：AccountsRead、HoldingsRead、PerformanceRead、ActivitiesRead、FinancialPlanningRead、HealthRead、ClassificationRead、ActivitiesDraft、ActivitiesWrite、ClassificationSuggest、ClassificationWrite。

方法：ALL（完整列表）、READ_SCOPES（只读子集）、as_str()、parse()。

#### AgentScopeSet (agent-tools/src/scope.rs:104)

| 方法 | 签名 | Notes |
|------|------|-------|
| new | fn() -> Self | |
| read_only | fn() -> Self | 所有读权限 |
| read_activity_draft | fn() -> Self | 读 + 草稿 |
| read_activity_write | fn() -> Self | 读 + 草稿 + 写入 |
| read_activity_write_classification_suggest | fn() -> Self | 完整权限 |
| from_strs | fn(iter) -> Self | 静默跳过未知 |
| insert | fn(&mut self, scope) | |
| contains | fn(&self, scope) -> bool | |
| grants_all | fn(&self, required) -> bool | |
| dependency_error | fn(&self) -> Option<String> | 验证依赖 |

#### AgentEnvironment trait (agent-tools/src/env.rs:32)

提供 18 个服务访问方法：
- base_currency() -> String
- account_service() -> Arc<dyn AccountServiceTrait>
- activity_service() -> Arc<dyn ActivityServiceTrait>
- holdings_service() -> Arc<dyn HoldingsServiceTrait>
- valuation_service() -> Arc<dyn ValuationServiceTrait>
- goal_service() -> Arc<dyn GoalServiceTrait>
- settings_service() -> Arc<dyn SettingsServiceTrait>
- quote_service() -> Arc<dyn QuoteServiceTrait>
- asset_service() -> Arc<dyn AssetServiceTrait>
- allocation_service() -> Arc<dyn AllocationServiceTrait>
- performance_service() -> Arc<dyn PerformanceServiceTrait>
- income_service() -> Arc<dyn IncomeServiceTrait>
- health_service() -> Arc<dyn HealthServiceTrait>
- taxonomy_service() -> Arc<dyn TaxonomyServiceTrait>
- portfolio_service() -> Arc<dyn PortfolioServiceTrait>
- net_worth_service() -> Arc<dyn NetWorthServiceTrait>
- contribution_limit_service() -> Arc<dyn ContributionLimitServiceTrait>
- cash_activity_service() -> Arc<dyn CashActivityServiceTrait>
- categorization_rules_service() -> Arc<dyn CategorizationRulesServiceTrait>

### 5.6 Market Data 模块 (market-data/)

#### InstrumentId (market-data/src/models/instrument.rs:40)

枚举：
- Equity { ticker: Arc<str>, mic: Option<Mic> }
- Crypto { base: Arc<str>, quote: Currency }
- Fx { base: Currency, quote: Currency }
- Metal { code: Arc<str>, quote: Currency }
- Option { occ_symbol: Arc<str> }
- Bond { isin: Arc<str> }

方法：kind() -> AssetKind、instrument_kind() -> InstrumentKind。

#### InstrumentKind (market-data/src/models/instrument.rs:10)

枚举：Equity、Crypto、Fx、Metal、Option、Bond。用于提供商能力路由。

#### AssetKind (market-data/src/models/instrument.rs:20)

枚举（用于市场数据分类，独立于 portfolio 的 AssetKind）：Security、Crypto、Cash、FxRate、Option、Commodity、PrivateEquity、Property、Vehicle、Liability、Other。Default = Security。

#### Quote (market-data/src/models/quote.rs:57)

| 字段 | 类型 | Notes |
|------|------|-------|
| timestamp | DateTime<Utc> | |
| open | Option<Decimal> | |
| high | Option<Decimal> | |
| low | Option<Decimal> | |
| close | Decimal | 必填 |
| volume | Option<Decimal> | |
| currency | String | |
| source | String | YAHOO / ALPHA_VANTAGE / MANUAL |

方法：new()、ohlcv()。

#### QuoteContext (market-data/src/models/quote.rs:32)

| 字段 | 类型 | Notes |
|------|------|-------|
| instrument | InstrumentId | |
| identifiers | QuoteIdentifiers | ISIN |
| overrides | Option<ProviderOverrides> | |
| currency_hint | Option<Currency> | |
| preferred_provider | Option<ProviderId> | |
| bond_metadata | Option<BondQuoteMetadata> | 收益率曲线定价 |
| custom_provider_code | Option<String> | |

#### BondQuoteMetadata (market-data/src/models/quote.rs:19)

| 字段 | 类型 |
|------|------|
| coupon_rate | Decimal |
| maturity_date | NaiveDate |
| face_value | Decimal |
| coupon_frequency | String |

#### AssetProfile (market-data/src/models/profile.rs:5)

| 字段 | 类型 | Notes |
|------|------|-------|
| source | Option<String> | 提供商 |
| name | Option<String> | |
| quote_type | Option<String> | EQUITY / ETF / CRYPTOCURRENCY |
| sector | Option<String> | |
| sectors | Option<String> | JSON 数组 |
| asset_allocation | Option<String> | JSON 数组 |
| industry | Option<String> | |
| website | Option<String> | |
| description | Option<String> | |
| country | Option<String> | ISO 3166-1 alpha-2 |
| employees | Option<u64> | |
| logo_url | Option<String> | |
| market_cap | Option<f64> | |
| pe_ratio | Option<f64> | |
| dividend_yield | Option<f64> | |
| week_52_high, week_52_low | Option<f64> | |
| isin | Option<String> | |

#### Type Aliases (market-data/src/models/types.rs)

| 类型 | 真实类型 | Notes |
|------|----------|-------|
| ProviderId | Cow<'static, str> | 静态常量 |
| Mic | Cow<'static, str> | ISO 10383 |
| Currency | Cow<'static, str> | ISO 4217 |
| ProviderSymbol | Arc<str> | 运行时符号 |

#### SplitEvent (market-data/src/models/mod.rs:40)

| 字段 | 类型 |
|------|------|
| date | NaiveDate |
| ratio | Decimal |

## 6. 存储层映射 (crates/storage-sqlite/)

### 6.1 架构概览

`storage-sqlite` 是唯一包含 Diesel ORM 依赖的 crate，实现了所有定义在 `wealthfolio-core` 和 `wealthfolio-connect` 中的 repository trait。所有其他 crate 通过 trait 接口与数据库交互，保持数据库无关性。

核心基础设施：
- **DbPool**: `r2d2::Pool<ConnectionManager<SqliteConnection>>` 连接池，最大 8 连接，最小 1 空闲连接
- **WriteHandle**: 单写入者 actor 模式，通过 MPSC 通道 + `oneshot` 回复实现序列化写入
- **DbTransactionExecutor**: 事务执行 trait，用于连接池上的事务包装
- **StorageError**: 6 变体（ConnectionFailed, PoolError, QueryFailed, MigrationFailed, SerializationError, CoreError），映射到 `wealthfolio_core::Error`
- **Migrations**: `embed_migrations!()` 嵌入 Diesel 迁移，通过 `run_migrations()` 自动应用
- **Schema**: `schema.rs` 自动生成，包含 40+ 个表定义和 `joinable!`/`allow_tables_to_appear_in_same_query!` 声明

### 6.2 数据库表清单 (schema.rs)

| 表名 | 主键 | 说明 |
|------|------|------|
| accounts | id | 账户 |
| activities | id | 活动/交易 |
| assets | id | 资产 |
| lots | id | 税务批次 |
| lot_disposals | id | 批次处置 |
| quotes | id | 价格引用 |
| daily_account_valuation | id | 每日估值 |
| holdings_snapshots | id | 持仓快照 |
| snapshot_positions | id (Integer) | 快照持仓明细 |
| goals | id | 目标 |
| goal_plans | goal_id | 目标计划 |
| goals_allocation | id | 目标资金分配 |
| health_issue_dismissals | issue_id | 健康检查关闭 |
| contribution_limits | id | 缴款限额 |
| app_settings | setting_key | 键值配置 |
| taxonomies | id | 分类体系 |
| taxonomy_categories | (id, taxonomy_id) | 分类类别 |
| asset_taxonomy_assignments | id | 资产分类分配 |
| portfolios | id | 投资组合 |
| portfolio_accounts | id | 组合-账户关联 |
| allocation_targets | id | 资产配置目标 |
| allocation_target_weights | id | 配置权重 |
| allocation_target_constraints | id | 配置约束 |
| import_runs | id | 导入运行 |
| import_templates | id | 导入模板 |
| import_account_templates | id | 账户导入模板 |
| brokers_sync_state | (account_id, provider) | 券商同步状态 |
| platforms | id | 券商平台 |
| market_data_providers | id | 行情数据提供商 |
| market_data_custom_providers | id | 自定义提供商 |
| quote_sync_state | asset_id | 引用同步状态 |
| sync_outbox | event_id | 设备同步出站 |
| sync_applied_events | event_id | 已应用事件 |
| sync_cursor | id | 同步游标 |
| sync_device_config | device_id | 设备配置 |
| sync_engine_state | id | 同步引擎状态 |
| sync_entity_metadata | (entity, entity_id) | 实体元数据 |
| sync_table_state | table_name | 表同步状态 |
| ai_threads | id | AI 聊天线程 |
| ai_messages | id | AI 聊天消息 |
| ai_thread_tags | id | 线程标签 |
| spending_categorization_rules | id | 支出分类规则 |
| spending_activity_events | activity_id | 活动事件关联 |
| spending_activity_splits | id | 活动拆分 |
| spending_event_types | id | 事件类型 |
| spending_events | id | 事件 |
| spending_preset_rule_deletions | (preset_id, preset_rule_key) | 预设规则删除 |
| budget_groups | id | 预算组 |
| budget_group_assignments | id | 预算组分配 |
| budget_targets | id | 预算目标 |
| budget_rollover_settings | id | 预算滚动设置 |
| activity_taxonomy_assignments | id | 活动分类分配 |
| addon_storage | (addon_id, key) | 插件键值存储 |
| personal_access_tokens | id | 个人访问令牌 |
| mcp_audit_log | id | MCP 审计日志 |

### 6.3 Repository 实现清单

所有 repository 遵循相同模式：接收 `Arc<Pool>` + `WriteHandle`，读取方法使用 `get_connection(&self.pool)?` 获取连接，写入方法通过 `self.writer.exec()` 或 `self.writer.exec_tx()` 提交到单写入者 actor。

| Repository | 文件 | 实现 Trait | 关键方法 |
|-----------|------|-----------|---------|
| AccountRepository | accounts/repository.rs | AccountRepositoryTrait | create, update, get_by_id, list, get_accounting_settings_by_account_ids, delete |
| ActivityRepository | activities/repository.rs | ActivityRepositoryTrait | create, update, get_by_id, search, bulk_upsert, add_income, get_income_data, get_import_templates 等 |
| AssetRepository | assets/repository.rs | AssetRepositoryTrait | create, get_by_id, list, list_by_asset_ids, update_profile, merge_assets |
| AlternativeAssetRepository | assets/alternative_repository.rs | AlternativeAssetRepositoryTrait | update_alternative_metadata, delete_alternative_asset |
| FxRepository | fx/repository.rs | FxRepositoryTrait | get_all_currency_quotes, get_fx_rate, get_fx_rate_at, get_fx_rate_at_or_latest, get_currency_history |
| GoalRepository | goals/repository.rs | GoalRepositoryTrait | create, get_by_id, list, update, save_plan, get_plan, delete, update_summary |
| HealthDismissalRepository | health/repository.rs | HealthDismissalStore | save_dismissal, get_dismissal, get_dismissals, remove_dismissal |
| ContributionLimitRepository | limits/repository.rs | ContributionLimitRepositoryTrait | create, get_by_id, list, get_by_group, update, delete |
| LotRepository | lots.rs | LotRepositoryTrait | get_open_lots, get_closed_lots, save_lot, save_lots, close_lot, get_disposals, save_disposal, get_lot_views, rebuild_lots, get_lot_for_accounting 等 |
| SettingsRepository | settings/repository.rs | SettingsRepositoryTrait | get_settings, update_settings, get_setting |
| MarketDataRepository | market_data/repository.rs | QuoteStore, ProviderSettingsStore | get_quotes, get_latest_quotes, save_quotes, get_providers, upsert_provider, save_provider_settings |
| QuoteSyncStateRepository | market_data/quote_sync_state_repository.rs | QuoteSyncStateRepositoryTrait | get_sync_state, upsert_sync_state, get_stale_sync_states |
| SnapshotRepository | portfolio/snapshot/repository.rs | SnapshotRepositoryTrait | save_snapshots, get_snapshot, list_snapshots, delete_snapshot |
| ValuationRepository | portfolio/valuation/repository.rs | ValuationRepositoryTrait | save_valuations, get_valuations, get_valuation, delete_valuations, get_latest_valuation |
| AllocationTargetRepository | portfolio/allocation_targets/repository.rs | AllocationTargetRepositoryTrait | create, get_by_id, list, update, delete, save_weights 等 |
| PortfolioRepository | portfolios/repository.rs | PortfolioRepositoryTrait | create, get_by_id, list, update, delete, add_account, remove_account |
| TaxonomyRepository | taxonomies/repository.rs | TaxonomyRepositoryTrait | create_taxonomy, get_taxonomy, list, update_taxonomy, delete_taxonomy, create_category, update_category, delete_category, assign_asset, get_assignments |
| PlatformRepository | sync/platform/repository.rs | PlatformRepositoryTrait | get_by_id, list, upsert_platforms |
| BrokerSyncStateRepository | sync/state/repository.rs | BrokerSyncStateRepositoryTrait | get_by_account_id, upsert_attempt, upsert_success, upsert_failure, upsert_needs_review, get_all |
| ImportRunRepository | sync/import_run/repository.rs | ImportRunRepositoryTrait | create, update, get_by_id, get_recent_for_account, get_all, get_by_run_type |
| SyncRepository | sync/app_sync/repository.rs | SyncRepositoryTrait | push_event, pull_events, get_cursor, advance_cursor, get_snapshot_ref, reconcile_ready_state, upload_snapshot 等 |
| AddonStorageRepository | addons/storage.rs | AddonStorageRepositoryTrait | get, set, delete, list_keys, list_for_addon |
| AiChatRepository | ai_chat/repository.rs | ChatRepositoryTrait | create_thread, get_thread, list_threads, update_thread, delete_thread, create_message, get_message, get_messages_by_thread, update_message, add_tag, remove_tag, get_tags |
| CustomProviderRepository | custom_provider/repository.rs | CustomProviderRepository | create, get_by_id, list, update, delete, get_sources |
| AuditLogRepository | agent/audit_log.rs | — | log, list, get_by_session |
| PersonalAccessTokenRepository | agent/pat.rs | — | create, get_by_id, list, revoke, update_last_used |

### 6.4 写入架构

所有写入操作通过 `WriteHandle` 进入单写入者 actor（`db/write_actor.rs`），保证：
1. **序列化写入**：所有写入通过 MPSC 通道排队，避免 SQLite 写入冲突
2. **事务支持**：`exec_tx` 方法允许在事务内执行多个操作
3. **出站集成**：写入完成后自动触发 `flush_projected_outbox` 将变更写入 `sync_outbox` 表
4. **类型安全**：通过 `oneshot` 通道返回类型擦除的结果，调用方通过泛型恢复类型

### 6.5 同步架构

设备同步层（`sync/app_sync/`）包含：
- **SyncOutboxModel / SyncOutboxEventDB**: 出站事件存储
- **OutboxProjector**: 将业务变更投影为同步事件
- **SyncAppliedEventDB**: 去重已应用事件
- **SyncCursorDB**: 乐观锁游标
- **SyncEngineStateDB**: 引擎状态（重试、失败计数）
- **SyncEntityMetadataDB**: 每个实体的最后事件元数据
- **SyncDeviceConfigDB**: 设备信任状态和密钥版本
- **SyncTableStateDB**: 表级同步开关

`sync_entity_from_remote()` 函数将 38 个远程实体字符串映射到 `SyncEntity` 变体，支持双向转换。

### 6.6 数据库模型与领域模型转换

每个领域模块包含 `model.rs` 定义 Diesel 数据库模型（`*DB` 后缀），通过 `From`/`TryFrom` 实现与领域模型的转换。关键约定：
- `Decimal` 值存储为 `String`，避免 SQLite 精度损失
- 枚举存储为 `String`，通过 `serde_json` 或自定义序列化
- 嵌套 JSON 存储为 `Option<String>`（如 `accounts.meta`）
- `created_at`/`updated_at` 存储为 `String`（RFC3339 格式）或 `NaiveDateTime`
- `Bool` 字段映射为 SQLite `Integer`（0/1）

## 7. 综合评分

### 7.1 模块评分表

| 模块 | 可复用性 | 迁移难度 | 代码风险 | 说明 |
|------|---------|---------|---------|------|
| **accounts** | A | low | low | 独立领域模块，Account 模型清晰，Repository trait 接口稳定。`meta` JSON 字段承载 accounting 设置是唯一耦合点，但设计上向后兼容。 |
| **activities** | A | high | medium | 核心交易模型，14 种活动类型封闭枚举。ActivityRepositoryTrait 大而全（50+ 方法），迁移时需全量实现。`DailyActivityDB` 等派生表增加了维护成本。 |
| **assets** | A | low | low | Asset 模型是独立的值对象，Enum 映射（kind, instrument_type, quote_mode）设计清晰。AlternativeAssetRepository 实现了简化模型。 |
| **portfolio** | B | medium | high | 包含 snapshot/valuation/allocation 三个子系统，相互依赖复杂。Snapshot 的确定性 UUID 生成和 HoldingsSnapshot 状态机（6 种源类型）是高风险区域。Valuation 计算在存储层之外。 |
| **goals** | B | medium | low | Goal 模型依赖 account 和 portfolio 层。`GoalFundingRule` 的 share_percent 验证（100% 上限）涉及跨账户聚合查询。Plan 的 JSON settings 是黑盒。 |
| **quotes** | A | low | medium | QuoteStore 和 ProviderSettingsStore trait 设计清晰。优先级排序（MANUAL > PROVIDER > BROKER）是业务关键逻辑。`quote_sync_state` 跟踪精度高。 |
| **fx** | A | low | low | ExchangeRate 模型简单，FX 资产作为特殊 Asset 类型处理。时间点查询实现良好。 |
| **settings** | A | low | low | 简单的键值存储，KISS 设计。 |
| **health** | A | low | low | HealthDismissalStore trait 极小（4 方法），IssueDismissal 模型简单。SHA-256 指纹用于去重。 |
| **limits** | A | low | low | ContributionLimit 独立模型，Repository trait 小而稳定。 |
| **lots** | B | high | high | 30+ 字段的 LotRecord，String 存储 Decimal。`rebuild_lots` 是核心业务逻辑，涉及完整交易历史回放。Cost Basis 方法（FIFO/LIFO/WAC）和 Pooling Scope 的大量组合。LotDisposal 的 `original_*` 字段是历史不可变记录。 |
| **sync** | A | medium | medium | 38 种 SyncEntity 变体，LWW 冲突解决策略。Outbox 模式设计良好。SyncRepositoryTrait 是大型接口。`sync_entity_from_remote()` 的字符串映射是脆弱点。 |
| **spending** | B | medium | medium | 可选运行时特性，独立于核心领域。10 个子模块，功能完整但规模大。Budget snapshot 计算复杂度高。与 Activity 的数据同步通过 `activity_sync` 模块实现。 |
| **connect** | B | high | high | 外部 API 耦合，大量类型转换（connect ↔ core）。BrokerAccount 有 30+ 字段且含旧版兼容字段。BrokerSyncState 状态机（Idle/Running/NeedsReview/Failed）和 ImportRun 状态机（5 状态）是核心复杂度。 |
| **device-sync** | A | high | medium | E2EE 加密层，Bootstrap/Pair/Ready 三态注册协议。Push/Pull 同步 + Snapshot 上传/下载。网络层错误处理自带重试分类。耦合于特定 API 端点。 |
| **ai** | A | medium | low | 聊天模型，ChatRepositoryTrait（11 方法）独立于存储实现。ChatMessage 内容使用 schema 版本化（当前版本 1）。AiStreamEvent 7 变体，流式事件驱动架构。 |
| **agent-tools** | A | low | low | 独立权限模型（11 个 AgentScope），AgentTool trait 对象安全。无状态设计，可在任何运行时复用。 |
| **market-data** | A | low | low | 6 种 InstrumentId 变体，provider-agnostic 设计。Quote 和 AssetProfile 是纯值对象。 |
| **storage-sqlite** | B | high | high | 唯一 Diesel 耦合层，40+ 个表，20+ 个 Repository 实现。WriteHandle 单写入者 actor 模式是架构核心。Migrations 管理 50+ 个迁移版本。`StorageError` 正确映射到领域错误。 |

### 7.2 可复用性分级

- **A（高可复用）**: 模块独立于运行时，trait 接口清晰，无外部耦合。accounts, assets, quotes, fx, settings, health, limits, ai, agent-tools, market-data 均可直接复用于新前端或新存储后端。
- **B（中可复用）**: 模块间有依赖或包含外部耦合。portfolio, goals, spending, lots, connect, device-sync, storage-sqlite 需要宿主环境适配。

### 7.3 迁移难度

- **low**: 纯数据模型，trait 接口小。可在一周内完成迁移。
- **medium**: 中等复杂度，涉及跨模块查询或状态机。需要 1-3 周。
- **high**: 大型接口，复杂业务逻辑，或外部依赖。需要 1-3 个月。

### 7.4 代码风险

- **low**: 代码路径简单，测试覆盖充分，逻辑直观。
- **medium**: 有潜在边界情况，部分逻辑复杂，但风险可控。
- **high**: 核心业务逻辑（Lot 重建、Valuation 计算、Broker 同步），错误可能导致数据不一致。存储层是全局单点故障。

### 7.5 总结

Wealthfolio 的 Rust 领域模型层呈现清晰的六边形架构：

1. **核心域（crates/core）**: 25 个领域模块，定义 entity、repository trait、service trait。无外部依赖，仅依赖标准库和常见 crate（serde, chrono, rust_decimal, uuid）。业务逻辑完全在 trait 之后抽象。

2. **附属域（crates/spending, connect, device-sync, ai, agent-tools, market-data）**: 可选或外围功能，各自独立于核心域。spending 通过 feature flag 隔离，connect 通过 trait 解耦，agent-tools 无状态且运行时无关。

3. **存储层（crates/storage-sqlite）**: 唯一 Diesel 耦合层，实现所有 repository trait。单写入者 actor 模式保证 SQLite 写入安全，Diesel 迁移管理 50+ 版本。

4. **设计亮点**:
   - UUID v7 非顺序 ID，避免 SQLite 自增主键的冲突问题
   - `Decimal` 的 String 存储，避免 SQLite 浮点精度损失
   - 确定性 Snapshot UUID（SHA-256 哈希），防止重复
   - Lot 的 `original_*` 字段，历史不可变，支持审计回溯
   - LWW 冲突解决 + Outbox 模式，设备同步可靠
   - 健康检查的 SHA-256 指纹去重，避免重复告警
   - 权限系统的 11 个 AgentScope，支持最小权限原则

5. **架构约束**:
   - 所有数据本地存储（SQLite），无云依赖
   - 密钥通过 OS keyring 管理，不落盘
   - 日志中不记录密钥或财务数据
   - 支持桌面（Tauri）和 Web（Axum）双运行时
