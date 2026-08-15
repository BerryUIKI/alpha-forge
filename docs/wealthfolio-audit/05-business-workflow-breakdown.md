# Wealthfolio — 核心业务流程分解 (Business Workflow Breakdown)

> 生成日期：2026-08-12
> 仓库：Wealthfolio
> 范围：14 个核心业务流程的逐步分解，含条件分支、循环逻辑、错误分支、重试策略、缓存判断、权限校验，以及每个处理步骤对应的源文件与目标函数。
> 每个流程末尾附：可复用性评级、迁移难度、代码风险评估。

---

## 评级图例

| 维度 | 级别 | 含义 |
|------|------|------|
| 可复用性 | A / B / C | A=几乎可直接复用；B=需少量修改；C=与宿主强耦合不可拆分 |
| 迁移难度 | 低 / 中 / 高 | 拆分或迁移到独立模块的工作量 |
| 代码风险 | 低 / 中 / 高 | 逻辑复杂度、并发安全、边界处理风险 |

---

## 1. 账户管理 (Account Management)

### 1.1 流程概述

账户 CRUD 与盘点，核心文件：
- `crates/core/src/accounts/accounts_service.rs` (272 行)
- `apps/frontend/src/commands/account.ts`（命令包装）
- `crates/storage-sqlite/src/repositories/account_repository.rs`（持久化）

### 1.2 逐步分解

| 步骤 | 处理 | 源文件 / 目标函数 | 分支 / 循环 / 错误 |
|------|------|-------------------|--------------------|
| 1 | 校验账户参数（名称、币种、类型） | `accounts_service.rs::create_account` | **错误分支**：名称/币种为空 → 返回 `ValidationError` |
| 2 | 判断默认账户 | `accounts_service.rs::create_account` | **条件分支**：`is_default=true` → 先清除其他账户的默认标记（`clear_default_account`） |
| 3 | 持久化账户 | `account_repository.rs::insert` | **错误分支**：唯一约束冲突 → 返回 `DatabaseError::Constraint` |
| 4 | 追踪模式判定 | `accounts_service.rs`（`TrackingMode`） | **条件分支**：`Holdings` / `Transactions` / `NotSet` 三态，决定后续快照还是交易计算 |
| 5 | 列出账户 | `accounts_service.rs::list_accounts` | **循环逻辑**：遍历全部账户，按 `is_archived` 过滤；健康检查依赖此列表判定未配置账户 |
| 6 | 更新账户 | `accounts_service.rs::update_account` | **错误分支**：账户不存在 → `NotFound`；改名/换币种需级联刷新快照币种 |
| 7 | 删除账户 | `accounts_service.rs::delete_account` | **错误分支**：存在关联资产/交易 → 拒绝删除或标记归档 |

### 1.3 细粒度逻辑

- **TrackingMode → 计算路径选择**：`TrackingMode::Holdings` 走快照式持仓（成本基准来自快照 Position）；`TrackingMode::Transactions` 走交易式（成本基准来自 lot/disposal）；`NotSet` 视为未配置（健康检查 `AccountConfigurationCheck` 标记）。
  - 源：`crates/core/src/health/service.rs` 中 `AccountConfigurationCheck` 依赖该枚举。
- **币种一致性**：账户币种异动时，`update_account` 需同步刷新 `account_currency` 相关快照，否则 FX 换算失真。

### 1.4 评估

| 维度 | 评估 |
|------|------|
| 可复用性 | **B** — 账户模型与 CRUD 通用，但 `TrackingMode` 与快照/交易计算耦合 |
| 迁移难度 | 低 |
| 代码风险 | 低 |

---

## 2. 活动记录 (Activity Recording)

### 2.1 流程概述

新增/导入/编辑交易活动，触发编译与传递对解析。核心文件：
- `crates/core/src/activities/activities_service.rs` (6413 行)
- `crates/core/src/activities/compiler.rs` (862 行)
- `crates/core/src/activities/transfer_pairs.rs` (377 行)

### 2.2 逐步分解

| 步骤 | 处理 | 源文件 / 目标函数 | 分支 / 循环 / 错误 |
|------|------|-------------------|--------------------|
| 1 | 幂等键计算 | `activities_service.rs`（`compute_idempotency_key`） | **条件分支**：由 account_id + activity_type + date + asset_id + quantity + unit_price + amount + fee + tax + currency 拼接；重复键 → 拒绝/去重 |
| 2 | 活动类型归一化 | `activities_service.rs`（`effective_type`） | **条件分支**：`activity_type_override` 存在则用之，否则用 `activity_type` |
| 3 | 活动编译拆分 | `compiler.rs`（`compile_activity`） | **条件分支**：`DRIP`/`STAKING_REWARD`/`DIVIDEND_IN_KIND` 子类型展开为「收入腿 + 买入腿」两条活动 |
| 4 | 传递对解析 | `transfer_pairs.rs`（`resolve_transfer_pairs`） | **循环逻辑**：按 `source_group_id` 分组 `TRANSFER_IN`/`TRANSFER_OUT`；**分支**：现金腿 / 证券腿 / FX 转换腿 |
| 5 | 状态判定 | `activities_service.rs`（`ActivityStatus`） | **条件分支**：`Draft` / `Pending` / `Posted`；未 posted 不参与快照计算 |
| 6 | 持久化活动 | `activities_service.rs::create_activity` | **错误分支**：幂等键冲突 / 币种缺失 → 返回 `ValidationError`；币种缺失被健康检查 `missing_currency_activities_from_data` 标记 |
| 7 | 触发派生计算 | `activities_service.rs`（通知快照/估值刷新） | **条件分支**：改动日期落在已计算区间 → 增量重算（`IncrementalFromLast`/`SinceDate`） |

### 2.3 细粒度逻辑

- **活动编译的多腿展开**：`compiler.rs` 中 `DRIP` 等复合类型需拆为收入动作 + 再投资买入动作，保证现金流与持仓同步。
  - 源：`compiler.rs::compile_activity`。
- **传递对的净额与 FX**：`transfer_pairs.rs` 处理跨账户资金/证券转移，含汇率转换腿；健康检查 `TransferIntegrityCheck` 依赖 `invalid_transfer_groups_from_activities` 判定未配对/未分组/外部标记冲突。
  - 源：`crates/core/src/health/service.rs` 中 `invalid_transfer_groups_from_activities`。
- **幂等性**：防止 CSV 重复导入或 broker 重复同步产生重复活动。

### 2.4 评估

| 维度 | 评估 |
|------|------|
| 可复用性 | **B** — 活动编译与传递对逻辑通用，但与账户/快照/估值耦合 |
| 迁移难度 | 中 |
| 代码风险 | 中（复合类型展开 + 传递对多腿易出错） |

---

## 3. 组合快照与持仓 (Portfolio Snapshot & Holdings)

### 3.1 流程概述

按账户生成每日持仓快照并聚合成持仓视图。核心文件：
- `crates/core/src/portfolio/snapshot/snapshot_service.rs` (1849 行)
- `crates/core/src/portfolio/snapshot/holdings_calculator/mod.rs` (535 行)
- `crates/core/src/portfolio/holdings_service.rs`（聚合 API）

### 3.2 逐步分解

| 步骤 | 处理 | 源文件 / 目标函数 | 分支 / 循环 / 错误 |
|------|------|-------------------|--------------------|
| 1 | 确定重算模式 | `snapshot_service.rs`（`SnapshotRecalcMode`） | **条件分支**：`Full` / `IncrementalFromLast` / `SinceDate`；增量模式从最后一个快照续算 |
| 2 | 追加校验 | `snapshot_service.rs` | **错误分支**：增量模式下日期早于最后快照 → 拒绝（append-only 约束） |
| 3 | 加载区间活动 | `snapshot_service.rs`（加载已 posted 活动） | **循环逻辑**：按账户/日期区间遍历活动 |
| 4 | 编译活动 | `compiler.rs`（复用 §2） | **条件分支**：复合类型展开为多腿 |
| 5 | 计算持仓 | `holdings_calculator/mod.rs`（`calculate_holdings`） | **循环逻辑**：逐活动应用买卖/分红/转移；按 `asset_id` 聚合 Position |
| 6 | 成本基准归集 | `holdings_calculator/mod.rs` | **条件分支**：`Transactions` 模式从 lots/disposals 衍生成 `total_cost_basis`/`average_cost`；`Holdings` 模式直接取快照 Position |
| 7 | 写入快照 | `snapshot_service.rs`（`save_snapshot`） | **错误分支**：并发写冲突 → 通过存储层 serializer 串行化 |
| 8 | 持仓视图聚合 | `holdings_service.rs`（`get_holdings`） | **循环逻辑**：跨账户合并，按 `asset_id` 去重汇总市值 |

### 3.3 细粒度逻辑

- **增量续算正确性**：增量模式必须保证从「准确基线」续算，否则 error 累积；`SinceDate` 用于带日期窗口的局部重算。
  - 源：`snapshot_service.rs` 中模式枚举与 `ProjectionRun`。
- **ProjectionRun 状态**：单次重算运行携带「转移 lot 缓存、lot 结束、处置」等临时状态，保证一次性原子完成。
  - 源：`snapshot_service.rs::ProjectionRun`。
- **位置成本基准**：`Position` 含 `total_cost_basis`、`average_cost`、`basis_status`（Complete/Unknown/PartialUnknown），健康检查 `gather_incomplete_basis_trade_activities` 与 `basis_source_issues_from_snapshots` 依赖此判定。
  - 源：`crates/core/src/health/service.rs`。

### 3.4 评估

| 维度 | 评估 |
|------|------|
| 可复用性 | **B** — 持仓计算内核可复用，但与活动编译/快照存储/估值耦合 |
| 迁移难度 | 中 |
| 代码风险 | 中（增量续算 + lot 状态机易出边界错误） |

---

## 4. 绩效计算 (Performance Calculation)

### 4.1 流程概述

基于估值历史与外部现金流计算金额加权/时间加权回报。核心文件：
- `crates/core/src/portfolio/performance/performance_service.rs`（428.2 KB，超大文件）
- `crates/core/src/portfolio/performance/`（子模块：period、contribution、metric、cumulative 等）

### 4.2 逐步分解

| 步骤 | 处理 | 源文件 / 目标函数 | 分支 / 循环 / 错误 |
|------|------|-------------------|--------------------|
| 1 | 加载估值历史 | `performance_service.rs`（读取 `DailyAccountValuation`） | **循环逻辑**：按账户/日期遍历估值行 |
| 2 | 解析外部现金流 | `performance_service.rs` | **条件分支**：`ExternalFlowSource` 三态 — 来自活动 / 净贡献差值 / 持仓推断（`NoFlow`/`FromActivities`/`Inferred`/`UnknownBoundaryTransfer`） |
| 3 | 计算净贡献 | `performance_service.rs`（`net_contribution`） | **条件分支**：`remove_outflows` 开关决定是否剔除外部流出 |
| 4 | 计算指标 | `performance_service.rs`（TWR / MWR / IRR） | **循环逻辑**：逐日/逐期迭代回报；`compound` 累计 |
| 5 | 汇总到组合/账户 | `performance_service.rs` | **循环逻辑**：跨账户聚合，按 base currency 换算 |
| 6 | 缓存判断 | `performance_service.rs` | **条件分支**：`cached_status` 或已算区间命中 → 直接返回，避免重复计算 |

### 4.3 细粒度逻辑

- **现金流来源优先级**：优先使用活动级显式外部流标记（`metadata.flow.is_external`）；否则用净贡献存量差值推断；否则看持仓跳变。`UnknownBoundaryTransfer` 会被健康检查标记。
  - 源：`crates/core/src/health/service.rs` 中 `UnknownPerformanceFlowSource` 判定。
- **币种换算**：所有指标统一折算到 base currency，`fx_rate_to_base` 参与计算。
- **超大文件风险**：`performance_service.rs` 428KB 单文件，维护与迁移难度高。

### 4.4 评估

| 维度 | 评估 |
|------|------|
| 可复用性 | **C** — 与估值/现金流/活动模型深度耦合，且单文件巨大 |
| 迁移难度 | 高 |
| 代码风险 | 高（财务精度、现金流推断、超大文件） |

---

## 5. 估值 (Valuation)

### 5.1 流程概述

为每个账户/日期生成 `DailyAccountValuation`（市值、成本基准、绩效净贡献、外部流）。核心文件：
- `crates/core/src/portfolio/valuation/valuation_service.rs` (5976 行)

### 5.2 逐步分解

| 步骤 | 处理 | 源文件 / 目标函数 | 分支 / 循环 / 错误 |
|------|------|-------------------|--------------------|
| 1 | 确定重算模式 | `valuation_service.rs`（`ValuationRecalcMode`） | **条件分支**：`Full` / `IncrementalFromLast` / `SinceDate` |
| 2 | 加载报价 | `valuation_service.rs`（从 quote 服务取价） | **条件分支**：手动报价（`QuoteMode::Manual`）与市场报价（`QuoteMode::Market`）分支 |
| 3 | 计算市值 | `valuation_service.rs`（`market_value`） | **循环逻辑**：逐持仓 × 单价；债券/期权用 `gross_trade_amount`/`contract_multiplier` 特殊处理 |
| 4 | 计算成本基准 | `valuation_service.rs`（`cost_basis`/`book_basis`） | **条件分支**：`basis_status` = Complete / Unknown / PartialUnknown |
| 5 | 计算净贡献与外部流 | `valuation_service.rs` | **条件分支**：外部流来源（含边界转移推断） |
| 6 | FX 换算 | `valuation_service.rs`（`fx_rate_to_base`） | **条件分支**：账户币种 ≠ base → 使用汇率；缺率 → 标记 `PartialUnpriced` |
| 7 | 写入估值行 | `valuation_service.rs`（`save_valuation`） | **循环逻辑**：逐日生成；`calculated_at` 记录时间戳 |
| 8 | 缓存判断 | `valuation_service.rs` | **条件分支**：已计算区间命中 → 跳过 |

### 5.3 细粒度逻辑

- **估值质量健康检查**：`valuation_quality_issues_from_histories` 检测缺失生成日期（`MissingGeneratedValuation`）、未定价天数（`PartialUnpriced`）、未知外部流；`value_asset_issues_from_snapshots` 按资产归类报价缺口。
  - 源：`crates/core/src/health/service.rs`。
- **成本基准缺失**：`IncompleteBasisActivity`（交易式缺价买入）与 `IncompleteBasisSnapshot`（持仓式快照缺基准）分别深链到活动/快照编辑器。
  - 源：`crates/core/src/health/service.rs`。

### 5.4 评估

| 维度 | 评估 |
|------|------|
| 可复用性 | **C** — 与快照/报价/活动/绩效深度耦合，单文件 5976 行 |
| 迁移难度 | 高 |
| 代码风险 | 高（财务精度、多种外部流来源、超大文件） |

---

## 6. 市场数据同步 (Market Data Sync)

### 6.1 流程概述

按排程拉取报价与汇率，更新 quote 历史。核心文件：
- `crates/core/src/quotes/sync.rs` (2814 行)
- `crates/core/src/quotes/service.rs` (4427 行)
- `crates/market-data/`（provider 层）

### 6.2 逐步分解

| 步骤 | 处理 | 源文件 / 目标函数 | 分支 / 循环 / 错误 |
|------|------|-------------------|--------------------|
| 1 | 同步分类 | `sync.rs`（`SyncCategory`） | **条件分支**：`Active` / `RecentlyClosed` / `Closed` / `NeedsBackfill` / `New` |
| 2 | 资产级锁 | `sync.rs`（`SYNC_LOCKS` HashSet + RAII guard） | **条件分支**：每资产上全局锁，避免并发重复同步；已锁 → 跳过 |
| 3 | 报价拉取 | `sync.rs` → `market-data` provider | **循环逻辑**：逐资产遍历提交 provider；**错误分支**：provider 失败 → 记录 `QuoteSyncErrorInfo` |
| 4 | 重试策略 | `market-data`（circuit breaker / rate limit） | **条件分支**：超限 → 退避暂停；连续失败 → 熔断降级到备用 provider |
| 5 | Provider 解析链 | `market-data`（`ResolverChain`/`ProviderRegistry`） | **循环逻辑**：主 provider 失败后依次尝试备用 provider |
| 6 | 手动/市场报价分开 | `service.rs`（`QuoteMode`） | **条件分支**：`Manual` 存用户录入价；`Market` 存拉取价 |
| 7 | 汇率同步 | `service.rs`（FX 引擎） | **条件分支**：`register_currency_pair` + `ensure_fx_pairs` 保证汇率对存在 |
| 8 | 缓存判断 | `service.rs`（`cached_status`） | **条件分支**：报价新鲜度在阈值内 → 跳过拉取 |

### 6.3 细粒度逻辑

- **价格新鲜度健康检查**：`PriceStalenessCheck` 用 `latest_quote_times` 对比 `price_stale_warning_hours` / `price_stale_critical_hours`；现金与 FX 基础设施资产被 `is_price_staleness_candidate` 排除。
  - 源：`crates/core/src/health/service.rs`。
- **同步错误下钻**：每次拉取失败写入 `QuoteSyncErrorInfo`，供 `QuoteSyncCheck` 呈现。

### 6.4 评估

| 维度 | 评估 |
|------|------|
| 可复用性 | **A** — `market-data` crate 完全自包含，`ResolverChain`/`ProviderRegistry` 通用 |
| 迁移难度 | 低（market-data）；中（sync 编排耦合 core） |
| 代码风险 | 中（并发锁 + provider 熔断 + 限流） |

---

## 7. 券商同步 (Broker Sync / Wealthfolio Connect)

### 7.1 流程概述

连接券商账号，分两阶段（活动 + 持仓）拉取数据并写库。核心文件：
- `crates/connect/src/broker/orchestrator.rs` (1014 行)
- `crates/connect/src/broker/`（token 生命周期、导入运行跟踪）

### 7.2 逐步分解

| 步骤 | 处理 | 源文件 / 目标函数 | 分支 / 循环 / 错误 |
|------|------|-------------------|--------------------|
| 1 | 连接鉴权 | `orchestrator.rs`（token 生命周期） | **错误分支**：token 过期/失效 → 触发重新授权（OAuth 流程） |
| 2 | 两阶段编排 | `orchestrator.rs`（`sync` 主流程） | **条件分支**：阶段一拉活动 → 阶段二拉持仓；按需分页 |
| 3 | 分页拉取 | `orchestrator.rs` | **循环逻辑**：`page_token`/offset 循环直到耗尽；每页记录进度 |
| 4 | 活动写库 | `orchestrator.rs` → `activities_service.rs` | **错误分支**：幂等键冲突 → 跳过（去重）；活动缺少币种 → 标记待修复 |
| 5 | 持仓写库 | `orchestrator.rs` → `snapshot_service.rs` | **条件分支**：`Holdings` 模式账号直接落快照 |
| 6 | 进度上报 | `orchestrator.rs` | **循环逻辑**：每页/每阶段回调进度事件给前端 |
| 7 | 导入运行跟踪 | `orchestrator.rs`（`ImportRun`） | **错误分支**：中途失败 → 记录失败状态，可重试续跑 |

### 7.3 细粒度逻辑

- **幂等去重**：写库前计算幂等键，重复活动被跳过，保证重复同步不产生脏数据。
- **失败续跑**：`ImportRun` 记录已处理游标，重试时从断点继续，避免全量重拉。
- **权限边界**：Connect 为只读集成，`BrokerSyncServiceTrait` 与 `PlatformRepositoryTrait` 是集成点（见 `check/08-reusable-module-assessment.md` §4）。

### 7.4 评估

| 维度 | 评估 |
|------|------|
| 可复用性 | **B** — 两阶段同步编排通用；券商模型可提取为独立 crate |
| 迁移难度 | 中 |
| 代码风险 | 中（分页状态机 + 幂等 + 失败续跑） |

---

## 8. 设备同步 (Device Sync)

### 8.1 流程概述

多设备配对 + 后台同步循环 + E2EE 加密。核心文件：
- `crates/device-sync/src/engine/runtime.rs` (207 行)
- `crates/device-sync/src/crypto/`（X25519 + ChaCha20-Poly1305 + HKDF）
- `crates/device-sync/src/`（配对协议、同步状态机）

### 8.2 逐步分解

| 步骤 | 处理 | 源文件 / 目标函数 | 分支 / 循环 / 错误 |
|------|------|-------------------|--------------------|
| 1 | 配对握手 | `runtime.rs`（`pair`） | **错误分支**：密钥交换失败 / 配对码无效 → 拒绝 |
| 2 | 密码学握手 | `crypto/`（X25519 ECDH + HKDF 派生会话密钥） | **条件分支**：双方公钥交换 → 派生共享密钥 |
| 3 | 后台同步循环 | `runtime.rs`（`sync_loop`） | **循环逻辑**：定时/事件触发，拉取变更集 |
| 4 | 变更集加密 | `crypto/`（ChaCha20-Poly1305） | **错误分支**：加密失败 → 跳过该批次 |
| 5 | 变更集解密 | `crypto/`（AEAD 校验） | **错误分支**：MAC 校验失败 → 丢弃并告警（防篡改） |
| 6 | 冲突解决 | `runtime.rs`（同步状态机） | **条件分支**：同一实体冲突 → 按时间戳/版本合并 |
| 7 | 推送批次 | `runtime.rs` | **错误分支**：服务端拒绝（如 addon 存储键超出字符集）→ 整批失败并重试 |

### 8.3 细粒度逻辑

- **E2EE 密钥层次**：X25519 做 ECDH，HKDF 派生，ChaCha20-Poly1305 做 AEAD；密钥经 `SecretStore` 存 OS keyring，绝不落盘。
  - 源：`crates/device-sync/src/crypto/`。
- **addon 存储同步约束**：`AddonService::validate_storage_key` 限制 `_ . : -` 字符集，保证同步服务端可接受（见 §14）。
- **依赖解耦**：`device-sync` 仅依赖 `core` 的 `SecretStore` trait，见 `check/08-reusable-module-assessment.md` §5。

### 8.4 评估

| 维度 | 评估 |
|------|------|
| 可复用性 | **A** — 密码学与协议层完全自包含，仅依赖 3 方法的 `SecretStore` trait |
| 迁移难度 | 低 |
| 代码风险 | 中（密码学正确性 + 冲突合并） |

---

## 9. 目标规划 (Goal Planning)

### 9.1 流程概述

目标 CRUD、资金分配规则、目标摘要刷新。核心文件：
- `crates/core/src/goals/goals_service.rs` (1772 行)

### 9.2 逐步分解

| 步骤 | 处理 | 源文件 / 目标函数 | 分支 / 循环 / 错误 |
|------|------|-------------------|--------------------|
| 1 | 创建目标 | `goals_service.rs::create_goal` | **错误分支**：同时存在多个活跃退休目标 → 拒绝（单一活跃退休目标约束） |
| 2 | 目标资金规则 | `goals_service.rs::save_goal_funding` | **错误分支**：`share_percent` 不在 0-100 → `ValidationError`；重复 accountId → 拒绝 |
| 3 | 税务桶校验 | `goals_service.rs::save_goal_funding` | **条件分支**：taxable / tax_deferred / tax_free 三桶；DC 关联账户不得出现在规则中 |
| 4 | 资金容量校验 | `goals_service.rs::validate_goal_funding_capacity` | **错误分支**：总分配超出可投资资金 → 拒绝（防止过度配置） |
| 5 | 计划保存 | `goals_service.rs::save_goal_plan` | **循环逻辑**：逐年/逐段计划生成 |
| 6 | 摘要刷新 | `goals_service.rs::refresh_goal_summary` | **条件分支**：`compute_summary_current_value` 算现值；`compute_retirement_overview_with_mode` 算退休概览 |
| 7 | 健康状态 | `goals_service.rs::refresh_goal_summary` | **条件分支**：`on_track` / `at_risk` / `off_track` / `not_applicable` |
| 8 | 种子规则 | `goals_service.rs::build_retirement_seed_rules` | **循环逻辑**：为新退休目标生成初始资金规则 |

### 9.3 细粒度逻辑

- **退休验证**：有限区间、非负金额、年龄顺序、重复 DC 关联账户校验。
- **资金分配语义**：`share_percent` 按比例分配，税务桶决定资金去向类型。

### 9.4 评估

| 维度 | 评估 |
|------|------|
| 可复用性 | **B** — 目标/资金规则模型通用，但与退休专项逻辑耦合 |
| 迁移难度 | 中 |
| 代码风险 | 中（资金容量 + 税务桶校验多分支） |

---

## 10. 退休规划 (Retirement Planning)

### 10.1 流程概述

退休概览、储蓄计算、情景模拟。核心文件：
- `crates/core/src/goals/goals_service.rs`（`compute_retirement_overview` / `compute_save_up_overview`）

### 10.2 逐步分解

| 步骤 | 处理 | 源文件 / 目标函数 | 分支 / 循环 / 错误 |
|------|------|-------------------|--------------------|
| 1 | 输入校验 | `goals_service.rs::compute_retirement_overview` | **错误分支**：年龄/金额非法 → `ValidationError` |
| 2 | 退休概览计算 | `goals_service.rs::compute_retirement_overview_with_mode` | **循环逻辑**：逐年龄/逐年模拟资金增长 |
| 3 | 储蓄目标计算 | `goals_service.rs::compute_save_up_overview` | **条件分支**：按税务桶分别计算储蓄需求 |
| 4 | 外部现金流 | `goals_service.rs` | **条件分支**：退休收入 > 支出 → 盈余；反之为缺口 |
| 5 | 摘要集成 | `refresh_goal_summary`（复用 §9） | **条件分支**：健康状态判定 |

### 10.3 细粒度逻辑

- 退休计算与目标摘要共享 `compute_retirement_overview_with_mode`，避免重复实现。

### 10.4 评估

| 维度 | 评估 |
|------|------|
| 可复用性 | **B** — 模拟逻辑通用，但税务桶/账户模型为核心专属 |
| 迁移难度 | 中 |
| 代码风险 | 中（财务模拟精度 + 边界年龄） |

---

## 11. 支出模块 (Spending Module)

### 11.1 流程概述

现金账户活动分类、预算管理、分析与报表。核心文件：
- `crates/spending/src/activity_classification.rs` (346 行)
- `crates/spending/src/budget/service.rs` (1748 行)
- `crates/spending/src/analytics/service.rs` (2110 行)

### 11.2 逐步分解

#### 11.2.1 活动分类

| 步骤 | 处理 | 源文件 / 目标函数 | 分支 / 循环 / 错误 |
|------|------|-------------------|--------------------|
| 1 | 分类判定 | `activity_classification.rs::classify_activity` | **条件分支**：`Income` / `Expense` / `ExpenseRefund` / `Saving` / `InternalTransfer` / `Ignored` |
| 2 | 边界检测 | `activity_classification.rs::classify_activity_for_aggregation` | **条件分支**：跨账户（CASH/TRANSFER_OUT）→ `Saving`；同组内 → `InternalTransfer` |
| 3 | 信用卡逻辑 | `activity_classification.rs::classify_activity` | **条件分支**：CREDIT_CARD WITHDRAWAL/FEE/INTEREST → `Expense`；CREDIT → `ExpenseRefund` |
| 4 | 现金账户逻辑 | `activity_classification.rs` | **条件分支**：DEPOSIT/TRANSFER_IN/INTEREST → `Income`；WITHDRAWAL/TRANSFER_OUT/FEE/TAX → `Expense`；CREDIT+BONUS → `Income`；CREDIT+REFUND/REBATE/REIMBURSEMENT → `ExpenseRefund` |

#### 11.2.2 预算

| 步骤 | 处理 | 源文件 / 目标函数 | 分支 / 循环 / 错误 |
|------|------|-------------------|--------------------|
| 1 | 构建预算快照 | `budget/service.rs::get`（`BudgetSnapshot`） | **循环逻辑**：按类别行/组行/收入行/汇总行遍历 |
| 2 | 默认组 | `budget/service.rs`（`DEFAULT_GROUPS`） | **条件分支**：6 组固定 UUID（Needs/Wants/Savings/Giving/Personal/Other） |
| 3 | 逐月实际值 | `budget/service.rs::actuals_by_month` | **循环逻辑**：加载区间活动 → 分类 → FX 换算（月末汇率）→ 按用户本地月分桶 |
| 4 | 滚动结转 | `budget/service.rs::compute_rollover_for_month` | **循环逻辑**：从 `start_month` 起逐月 `month_keys_between` 迭代，结转余额 |
| 5 | 目标覆盖优先级 | `budget/service.rs::TargetIndex::effective_category_decimal` | **条件分支**：`month_category` 覆盖 > `default_category` |
| 6 | 父类别向上追溯 | `budget/service.rs::top_category_id` | **循环逻辑**：`MAX_DEPTH=32` 循环保护，沿 parent 链追溯 |
| 7 | FX 换算容错 | `budget/service.rs::fx_to_target` | **条件分支**：同币种短路；`FxService` 错误 → 返回 None（排除 native amount） |
| 8 | 期间校验 | `budget/service.rs::validate_period_key` / `validate_month_key` | **错误分支**：YYYY-MM 格式不匹配 → 拒绝 |

#### 11.2.3 分析报表

| 步骤 | 处理 | 源文件 / 目标函数 | 分支 / 循环 / 错误 |
|------|------|-------------------|--------------------|
| 1 | 月报生成 | `analytics/service.rs::monthly_report` | **循环逻辑**：current + prior 期间，by_day 分桶，category 分解 |
| 2 | 汇总报告 | `analytics/service.rs::spending_summary` | **条件分支**：TOTAL / YTD / LAST_YEAR / TWO_YEARS_AGO 四个时段 |
| 3 | 事件支出汇总 | `analytics/service.rs::event_spending_summaries` | **循环逻辑**：按事件标签分组，`event_overlaps_window` 过滤日期窗口 |
| 4 | 汇总构建 | `analytics/service.rs::build_summary` | **循环逻辑**：by_month / by_account / by_category / by_subcategory 四维聚合；`UNCATEGORIZED_CATEGORY_ID = "__uncategorized__"` |
| 5 | 分类与分组 | `analytics/service.rs::summarize` | **循环逻辑**：逐活动分类 → FX 换算 → `count` 统计贡献收入/流出的活动数 |
| 6 | FX 日期约定 | `analytics/service.rs` | **条件分支**：`fx_as_of_current = end.date_naive()`，`fx_as_of_prior = prior_end.date_naive()` |

### 11.3 细粒度逻辑

- **退款桶清零**：`build_summary` 中退款超出当月开销时，整个桶清零（`total_spending=0`，`by_month`/`by_category` 清空），避免负值。
  - 源：`analytics/service.rs` 第 1884-1895 行测试。
- **跨时区分桶**：`build_summary` 支持用户时区，活动日期转换到本地时区后分月。
  - 源：`analytics/service.rs` 第 2100-2108 行测试。
- **预算组缓冲与滚动链**：`compute_rollover_for_month` 支持正/负余额结转到下月。

### 11.4 评估

| 维度 | 评估 |
|------|------|
| 可复用性 | **B** — 分类逻辑通用，但预算/分析依赖 `core` 活动模型与 FX 服务 |
| 迁移难度 | 中 |
| 代码风险 | 中（分类多分支 + 预算滚动逻辑 + FX 容错） |

---

## 12. 配置目标 (Allocation Targets)

### 12.1 流程概述

投资组合再平衡优化，计算目标配置与建议交易。核心文件：
- `crates/core/src/portfolio/allocation_targets/optimizer.rs` (1884 行)

### 12.2 逐步分解

| 步骤 | 处理 | 源文件 / 目标函数 | 分支 / 循环 / 错误 |
|------|------|-------------------|--------------------|
| 1 | 计算规划总额 | `optimizer.rs::plan`（`planning_total`） | **条件分支**：现金是否在 total_value 外 → 加上 `available_cash` |
| 2 | 卖出阶段 | `optimizer.rs::run_sell_phase` | **循环逻辑**：贪心循环，按漂移改善/美元得分排序，`max_turnover_bps` 限制周转率 |
| 3 | 整股批量 | `optimizer.rs::run_sell_phase` | **条件分支**：仅一个改善候选 → 整股批量（`whole_share`） |
| 4 | 买入阶段 | `optimizer.rs::run_buy_greedy` | **循环逻辑**：贪心买入，`cap_fractional_shares_to_next_bend` 控制碎股 |
| 5 | 按比例加仓 | `optimizer.rs::run_proportional_topup` | **循环逻辑**：按 `target_bps` 权重分配剩余现金，最大 sleeve 优先 |
| 6 | 目标区间 | `optimizer.rs::desired_bps_for_goal` | **条件分支**：`ExactTarget` → `target_bps`；`NearestBand` → `max(0, target_bps - band_bps)` |
| 7 | 最小交易量过滤 | `optimizer.rs::plan` | **条件分支**：金额小于 `min_trade_amount` → 过滤（不生成交易） |
| 8 | 买卖排序 | `optimizer.rs::plan` | **条件分支**：卖出交易前置，买入交易后置 |
| 9 | 警告生成 | `optimizer.rs::plan` | **条件分支**：`TurnoverCapReached` / `NoBuyCandidate` 写入 `RebalanceWarningKind` |

### 12.3 细粒度逻辑

- **DriftPriorityOptimizer 贪心策略**：先卖后买，两阶段买方（`CashFlowOnly`/`SellToRebalance`/`Hybrid` 两遍贪心），确保流动性优先。
- **周转率上限**：`max_turnover_bps` 限制单次调仓交易量，防止过度交易。
- **整股优化**：`whole_share` 批处理减少碎股产生。

### 12.4 评估

| 维度 | 评估 |
|------|------|
| 可复用性 | **B** — 优化器算法通用，但与 `RebalanceInput`/`RebalancePlan` 核心模型耦合 |
| 迁移难度 | 中 |
| 代码风险 | 中（贪心算法 + 碎股处理 + 周转率边界） |

---

## 13. 健康检查 (Health Checks)

### 13.1 流程概述

对组合进行 7 项静态度量检查，生成可操作的诊断建议。核心文件：
- `crates/core/src/health/service.rs` (3249 行)
- `crates/core/src/health/checks/`（各检查实例）

### 13.2 逐步分解

| 步骤 | 处理 | 源文件 / 目标函数 | 分支 / 循环 / 错误 |
|------|------|-------------------|--------------------|
| 1 | 缓存判断 | `service.rs::run_checks_with_data`（`cached_status`） | **条件分支**：`RwLock<Option<CachedStatus>>`，5 分钟失效标记 |
| 2 | 价格新鲜度检查 | `service.rs` → `PriceStalenessCheck` | **循环逻辑**：逐持仓对比 `latest_quote_times` 与 `price_stale_warning_hours`/`price_stale_critical_hours`；现金与 FX 资产排除 |
| 3 | 报价同步错误检查 | `service.rs` → `QuoteSyncCheck` | **循环逻辑**：遍历 `quote_sync_errors`，按严重程度分类 |
| 4 | FX 完整性检查 | `service.rs` → `FxIntegrityCheck` | **循环逻辑**：遍历 `fx_pairs`，检查汇率新鲜度 |
| 5 | 分类检查 | `service.rs` → `ClassificationCheck` | **循环逻辑**：遍历 `unclassified_assets`，标记未分类资产 |
| 6 | 数据一致性检查 | `service.rs` → `DataConsistencyCheck` | **循环逻辑**：含多项子检查 — 无效快照日期、无效活动日期、缺失 lot 处置、不完整成本基准、缺失币种、估值质量、转移完整性 |
| 7 | 账户配置检查 | `service.rs` → `AccountConfigurationCheck` | **循环逻辑**：遍历 `unconfigured_accounts`（`TrackingMode::NotSet`） |
| 8 | 转移完整性检查 | `service.rs` → `TransferIntegrityCheck` | **循环逻辑**：`invalid_transfer_groups_from_activities` 检测未配对/外部标记冲突 |
| 9 | 忽略已关闭问题 | `service.rs::filter_dismissed_issues` | **条件分支**：`data_hash` 匹配 → 跳过该 issue |
| 10 | 全量检查 | `service.rs::run_full_checks` | **循环逻辑**：汇集所有数据源（`account_service`、`holdings_service`、`quote_service` 等 10 个服务） |
| 11 | 自动修复 | `service.rs::execute_fix` | **条件分支**：`sync_prices` / `retry_sync` 等（部分待实现，`warn!("not yet implemented")`） |
| 12 | 配置验证 | `service.rs::update_config` | **错误分支**：`price_stale_warning_hours >= price_stale_critical_hours` → 拒绝 |

### 13.3 细粒度逻辑

- **成本基准缺失双路径**：交易式通过 `gather_incomplete_basis_trade_activities` 深链到活动；持仓式通过 `basis_source_issues_from_snapshots` 深链到快照编辑器。
  - 源：`service.rs`。
- **缺失处置检测**：`gather_missing_lot_disposal_sells` 对比活动 SELL 与 `disposals` 映射，发现无对应 lot 处置的卖出。
- **估值质量**：`valuation_quality_issues_from_histories` 聚合缺失生成日、缺失报价日、未知外部流。
- **严重度分级**：`Severity` 枚举（Info / Warning / Error / Critical），每项 issue 带 `diagnostics` 与 `fix_action`。

### 13.4 评估

| 维度 | 评估 |
|------|------|
| 可复用性 | **C** — 与核心模型/服务/存储深度耦合，7 项检查均依赖核心领域 |
| 迁移难度 | 高 |
| 代码风险 | 中（多数据源聚合 + 缓存一致性） |

---

## 14. 插件系统 (Addon System)

### 14.1 流程概述

插件安装/启用/更新/卸载生命周期，权限管理，网络请求审计，存储。核心文件：
- `crates/core/src/addons/service.rs` (3305 行)

### 14.2 逐步分解

| 步骤 | 处理 | 源文件 / 目标函数 | 分支 / 循环 / 错误 |
|------|------|-------------------|--------------------|
| 1 | 插件 ID 校验 | `service.rs::validate_addon_id` | **错误分支**：长度 > 64 / 非法字符 / 纯点号 / "staging" 保留 → 拒绝 |
| 2 | 下载 | `service.rs::download_addon_from_store` | **条件分支**：JSON 响应（含 downloadUrl）→ 二次下载；或直接 ZIP 二进制 |
| 3 | 解压校验 | `service.rs::extract_addon_zip_archive` | **错误分支**：超过 `MAX_ADDON_ARCHIVE_ENTRIES=256` / `MAX_ADDON_ARCHIVE_FILE_SIZE=5MB` / `MAX_ADDON_ARCHIVE_TOTAL_SIZE=25MB` / `MAX_ADDON_ARCHIVE_COMPRESSED_SIZE=50MB` / 路径遍历保护 → 拒绝 |
| 4 | 清单解析 | `service.rs::parse_manifest_json_metadata` | **错误分支**：缺少 id/name/version/main → 拒绝；permissions 或 network 可选 |
| 5 | 版本校验 | `service.rs::enforce_min_wealthfolio_version` | **错误分支**：`min_wealthfolio_version` 不满足 → 拒绝安装；版本字符串不可解析 → 失败关闭 |
| 6 | 权限检测 | `service.rs::detect_addon_permissions` | **循环逻辑**：扫描文件匹配 `api.category.function` / `ctx.api.category.function` 模式 |
| 7 | 网络审批 | `service.rs::apply_network_approvals` | **条件分支**：`allowed_hosts` 与 `approved_hosts` 交集；`approved_hosts` 去重排序 |
| 8 | 原子替换 | `service.rs::replace_addon_directory` | **错误分支**：temp 写入 → rename → 失败回滚 backup；防写中断 |
| 9 | 启用校验 | `service.rs::toggle_addon` | **错误分支**：启用时若版本不满足 → 禁止启用 |
| 10 | 更新权限检查 | `service.rs::ensure_update_does_not_add_permissions` | **错误分支**：新版本新增权限 → 需重新审批，拒绝自动更新 |
| 11 | 网络请求 | `service.rs::addon_network_request` | **权限校验**：`manifest_allows_function("secrets", "use")` 判定 auth 权限；`approved_hosts` 白名单 |
| 12 | 网络审计 | `service.rs::write_network_audit_entry` | **循环逻辑**：每次请求写入 JSONL 审计日志（时间戳、方法、host、状态码、字节数） |
| 13 | 存储 KV | `service.rs::get/set/delete_addon_storage_item` | **错误分支**：key 空/超长/字符集非法 → 拒绝；value 超 `MAX_ADDON_STORAGE_SYNC_PAYLOAD_LEN=250_000` → 拒绝 |
| 14 | 资产索引 | `service.rs::refresh_addon_asset_index` | **条件分支**：`CachedAddonAssets` 缓存；资产文件路径防遍历（`canonicalize` 检查） |
| 15 | 启动时加载 | `service.rs::get_enabled_addons_on_startup` | **循环逻辑**：遍历已安装列表，仅加载启用的 |
| 16 | 更新检查 | `service.rs::check_all_addon_updates` | **循环逻辑**：逐插件调 API；失败 → 记录 `error: Some(err)` 不阻塞其他插件 |

### 14.3 细粒度逻辑

- **路径遍历防护**：`validated_addon_archive_path` 拒绝反斜杠、Windows 驱动前缀、绝对路径、`..` 父目录穿越。
  - 源：`service.rs`。
- **原子替换**：`replace_addon_directory` 用 temp 目录 + backup 目录 + rename 实现原子替换；崩溃后通过 `recover_incomplete_replacements` 恢复。
- **存储同步约束**：`validate_storage_key` 限制字符集匹配同步服务端要求（见 §8）。
- **SHA-256 校验**：`verify_addon_package_sha256` 可选验证下载包完整性。

### 14.4 评估

| 维度 | 评估 |
|------|------|
| 可复用性 | **B** — 沙箱/权限/生命周期模式通用，但插件 SDK 与核心模型耦合 |
| 迁移难度 | 中 |
| 代码风险 | 中（文件系统安全 + 权限模型 + 网络审计） |

---

## 综合评估总表

| 编号 | 流程 | 可复用性 | 迁移难度 | 代码风险 | 关键文件 | 行数 |
|------|------|----------|----------|----------|----------|------|
| 1 | 账户管理 | B | 低 | 低 | `accounts_service.rs` | 272 |
| 2 | 活动记录 | B | 中 | 中 | `activities_service.rs`, `compiler.rs`, `transfer_pairs.rs` | 7652 |
| 3 | 快照与持仓 | B | 中 | 中 | `snapshot_service.rs`, `holdings_calculator/mod.rs` | 2384 |
| 4 | 绩效计算 | C | 高 | 高 | `performance_service.rs` | 428KB |
| 5 | 估值 | C | 高 | 高 | `valuation_service.rs` | 5976 |
| 6 | 市场数据同步 | A | 低 | 中 | `sync.rs`, `service.rs`, `market-data/` | 7241 |
| 7 | 券商同步 | B | 中 | 中 | `orchestrator.rs` | 1014 |
| 8 | 设备同步 | A | 低 | 中 | `runtime.rs`, `crypto/` | 207+ |
| 9 | 目标规划 | B | 中 | 中 | `goals_service.rs` | 1772 |
| 10 | 退休规划 | B | 中 | 中 | `goals_service.rs` | 同 9 |
| 11 | 支出模块 | B | 中 | 中 | `activity_classification.rs`, `budget/service.rs`, `analytics/service.rs` | 4204 |
| 12 | 配置目标 | B | 中 | 中 | `optimizer.rs` | 1884 |
| 13 | 健康检查 | C | 高 | 中 | `health/service.rs`, `health/checks/` | 3249 |
| 14 | 插件系统 | B | 中 | 中 | `addons/service.rs` | 3305 |

### 关键发现

1. **高代码风险区**：绩效计算（`performance_service.rs` 428KB）和估值（`valuation_service.rs` 5976 行）为超大单文件，内部逻辑高度耦合，任何改动的回归风险极高。
2. **最佳可复用模块**：市场数据（`market-data` crate）和设备同步（`device-sync` crate）几乎可直接提取为独立 crate，仅需少量 trait 适配。
3. **最复杂分支逻辑**：活动编译（复合类型多腿展开 + 传递对解析）、健康检查（7 项检查 + 子检查 + 数据哈希去重）、插件系统（原子替换 + 崩溃恢复 + 权限审批）。
4. **缓存策略覆盖**：报价新鲜度（5 分钟 stale 标记）、健康检查缓存（`cached_status` 5 分钟）、估值增量重算（`IncrementalFromLast`）均实现缓存判断。
5. **重试与容错**：市场数据 provider 熔断/限流 + 备用 provider 链；券商同步可续跑；插件更新失败不阻塞其他插件；预算 FX 换算失败优雅降级。
6. **权限校验**：插件系统实现声明式 API 权限、网络 host 白名单、auth 权限、存储 key 字符集校验；每次网络请求写入 JSONL 审计日志。