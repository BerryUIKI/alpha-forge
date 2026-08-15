# Portfolio Module — Domain Models Reference

> **Status:** Phase 1 complete. All 17 domain structs and 17 enums defined.
> **Source file:** `crates/domain/src/financial.rs` (1170 lines)
> **All documentation is in English.**

---

## 1. Enums

All enums derive `Display` (writes canonical DB value) and `parse(&str) -> Option<Self>`
(reads DB value back). Serialized as `snake_case` via `#[serde(rename_all = "snake_case")]`.

### 1.1 Account Enums

| Enum | Variants | DB CHECK |
|------|----------|----------|
| `AccountType` | `securities`, `cash`, `credit_card`, `cryptocurrency` | `CHECK(account_type IN ('SECURITIES','CASH','CREDIT_CARD','CRYPTOCURRENCY'))` |
| `TrackingMode` | `not_set`, `transactions`, `holdings` | `CHECK(tracking_mode IN ('NOT_SET','TRANSACTIONS','HOLDINGS'))` |

### 1.2 Asset Enums

| Enum | Variants | DB CHECK |
|------|----------|----------|
| `AssetKind` | `investment`, `property`, `vehicle`, `collectible`, `precious_metal`, `private_equity`, `liability`, `other`, `fx` | CHECK on `assets.kind` |
| `QuoteMode` | `market`, `manual` | CHECK on `assets.quote_mode` |
| `InstrumentType` | `equity`, `crypto`, `fx`, `option`, `metal` | CHECK on `assets.instrument_type` |

### 1.3 Activity Enums

| Enum | Variants |
|------|----------|
| `ActivityType` | `buy`, `sell`, `split`, `dividend`, `interest`, `deposit`, `withdrawal`, `transfer_in`, `transfer_out`, `fee`, `tax`, `credit`, `adjustment`, `cash_journal` (14 variants) |
| `ActivityStatus` | `posted`, `pending`, `canceled` |
| `ExternalFlowSource` | `deposit`, `withdrawal`, `transfer`, `credit_interest`, `debit_interest`, `dividend`, `capital_gains`, `refund`, `fee`, `tax`, `commission`, `rebate`, `correction`, `other` (14 variants) |

### 1.4 Lot & Cost Basis Enums

| Enum | Variants |
|------|----------|
| `BasisStatus` | `filled`, `settled`, `partial` |
| `CostBasisMethod` | `fifo`, `lifo`, `average_cost`, `specific_id` |

### 1.5 Valuation & Snapshot Enums

| Enum | Variants |
|------|----------|
| `ValuationStatus` | `initial`, `estimated`, `calculated`, `reviewed`, `final` |
| `HoldingSnapshotSource` | `manual`, `system`, `import` |

### 1.6 Allocation Enums

| Enum | Variants |
|------|----------|
| `ScopeType` | `account`, `portfolio`, `workspace` |
| `ConstraintSubjectType` | `asset`, `category`, `taxonomy`, `account` |
| `ConstraintAction` | `min`, `max`, `exact`, `exclude` |
| `ConstraintEffect` | `hard`, `soft`, `warning` |

---

## 2. Structs

### 2.1 Platform

```rust
pub struct Platform {
    pub id: String,           // UUID v4
    pub name: Option<String>,
    pub url: String,
    pub external_id: Option<String>,
    pub kind: String,         // "BROKERAGE", "BANK", "EXCHANGE", etc.
    pub website_url: Option<String>,
    pub logo_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Table:** `platforms`
**Repository:** `PlatformRepository` (in `account_repository.rs`)
**Purpose:** A brokerage, bank, or exchange that hosts one or more accounts.

### 2.2 FinancialAccount

```rust
pub struct FinancialAccount {
    pub id: String,
    pub workspace_id: Option<String>,
    pub name: String,
    pub account_type: AccountType,
    pub group_name: Option<String>,
    pub currency: Option<String>,
    pub is_default: bool,
    pub is_active: bool,
    pub platform_id: Option<String>,
    pub account_number: Option<String>,
    pub meta: Option<serde_json::Value>,
    pub provider: Option<String>,
    pub provider_account_id: Option<String>,
    pub tracking_mode: TrackingMode,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Table:** `financial_accounts`
**Repository:** `AccountRepository` (in `account_repository.rs`)
**Purpose:** A brokerage, cash, credit-card, or cryptocurrency account tracked for
portfolio valuation. Replaces the placeholder `portfolio_accounts` table.

### 2.3 Asset

```rust
pub struct Asset {
    pub id: String,
    pub kind: AssetKind,
    pub display_code: String,
    pub quote_mode: QuoteMode,
    pub currency: Option<String>,
    pub instrument_type: Option<InstrumentType>,
    pub instrument_symbol: Option<String>,
    pub instrument_exchange: Option<String>,
    pub instrument_key: String,     // Generated column: kind+display_code
    pub alt_symbols: Option<serde_json::Value>,
    pub logo_url: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Table:** `assets`
**Repository:** `AssetRepository` (in `asset_repository.rs`)
**Purpose:** A tradeable or trackable instrument — equity, crypto, FX, metal, cash, or other.
Replaces the placeholder `positions.symbol` model.

### 2.4 Quote

```rust
pub struct Quote {
    pub id: String,
    pub asset_id: String,
    pub date: NaiveDate,
    pub source: Option<String>,
    pub open: Option<Decimal>,
    pub high: Option<Decimal>,
    pub low: Option<Decimal>,
    pub close: Option<Decimal>,
    pub adjclose: Option<Decimal>,
    pub volume: Option<i64>,
    pub currency: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Table:** `quotes`
**Repository:** `AssetRepository` (embedded QuoteRepository methods)
**Purpose:** A price point for an asset on a day from a source.

### 2.5 Activity

```rust
pub struct Activity {
    pub id: String,
    pub account_id: String,
    pub asset_id: Option<String>,
    pub import_run_id: Option<String>,
    pub activity_type: ActivityType,
    pub status: ActivityStatus,
    pub activity_date: NaiveDate,
    pub settlement_date: Option<NaiveDate>,
    pub quantity: Option<Decimal>,
    pub unit_price: Option<Decimal>,
    pub amount: Option<Decimal>,
    pub fee: Option<Decimal>,
    pub tax: Option<Decimal>,
    pub currency: Option<String>,
    pub fx_rate: Option<Decimal>,
    pub description: Option<String>,
    pub idempotency_key: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Table:** `activities`
**Repository:** `ActivityRepository` (in `activity_repository.rs`)
**Purpose:** A single transaction or cash movement — buy, sell, deposit, dividend, etc.
Replaces the placeholder `transactions` model.

### 2.6 ImportRun

```rust
pub struct ImportRun {
    pub id: String,
    pub account_id: String,
    pub status: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub checkpoint: Option<String>,
    pub warnings: Option<serde_json::Value>,
    pub errors: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Table:** `import_runs`
**Repository:** `ActivityRepository` (embedded ImportRunRepository methods)
**Purpose:** A batch import of activities from CSV or broker sync.

### 2.7 Lot

```rust
pub struct Lot {
    pub id: String,
    pub account_id: String,
    pub asset_id: String,
    pub open_date: NaiveDate,
    pub quantity: Decimal,
    pub cost_basis: Decimal,
    pub cost_basis_per_unit: Option<Decimal>,
    pub cost_basis_method: CostBasisMethod,
    pub status: BasisStatus,
    pub realized_gain: Option<Decimal>,
    pub realized_gain_per_unit: Option<Decimal>,
    pub disposal_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Table:** `lots`
**Repository:** `LotRepository` (in `lot_repository.rs`)
**Purpose:** A tax lot — a block of shares/units acquired at a specific price on a
specific date, used for FIFO cost-basis tracking.

### 2.8 LotDisposal

```rust
pub struct LotDisposal {
    pub id: String,
    pub lot_id: String,
    pub account_id: String,
    pub asset_id: String,
    pub disposal_date: NaiveDate,
    pub quantity: Decimal,
    pub proceeds: Decimal,
    pub cost_basis: Decimal,
    pub realized_gain: Decimal,
    pub created_at: DateTime<Utc>,
}
```

**Table:** `lot_disposals`
**Repository:** `LotRepository` (embedded LotDisposalRepository methods)
**Purpose:** Records a partial or full disposal of a tax lot.

### 2.9 HoldingSnapshot

```rust
pub struct HoldingSnapshot {
    pub id: String,
    pub account_id: String,
    pub snapshot_date: NaiveDate,
    pub source: HoldingSnapshotSource,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}
```

**Table:** `holding_snapshots`
**Repository:** `SnapshotRepository` (in `snapshot_repository.rs`)
**Purpose:** A point-in-time record of holdings for an account.

### 2.10 SnapshotPosition

```rust
pub struct SnapshotPosition {
    pub id: String,
    pub snapshot_id: String,
    pub asset_id: String,
    pub quantity: Decimal,
    pub cost_basis: Option<Decimal>,
    pub market_value: Option<Decimal>,
    pub created_at: DateTime<Utc>,
}
```

**Table:** `snapshot_positions`
**Repository:** `SnapshotRepository` (embedded)
**Purpose:** A position within a holding snapshot.

### 2.11 DailyAccountValuation

```rust
pub struct DailyAccountValuation {
    pub id: String,
    pub account_id: String,
    pub valuation_date: NaiveDate,
    pub total_value: Decimal,
    pub cash_balance: Option<Decimal>,
    pub cost_basis: Option<Decimal>,
    pub gain_loss: Option<Decimal>,
    pub gain_loss_percent: Option<Decimal>,
    pub status: ValuationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Table:** `daily_account_valuation`
**Repository:** `ValuationRepository` (in `valuation_repository.rs`)
**Purpose:** A daily valuation row for an account. UPSERT via `ON CONFLICT(account_id, valuation_date)`.

### 2.12 Taxonomy

```rust
pub struct Taxonomy {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Table:** `taxonomies`
**Repository:** `TaxonomyRepository` (in `taxonomy_repository.rs`)
**Purpose:** A classification system — e.g. "Sector", "Geography", "Asset Class".

### 2.13 TaxonomyCategory

```rust
pub struct TaxonomyCategory {
    pub id: String,
    pub taxonomy_id: String,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub sort_order: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Table:** `taxonomy_categories`
**Repository:** `TaxonomyRepository` (embedded)
**Purpose:** A category within a taxonomy — e.g. "Technology" under "Sector".

### 2.14 AssetTaxonomyAssignment

```rust
pub struct AssetTaxonomyAssignment {
    pub id: String,
    pub asset_id: String,
    pub category_id: String,
    pub weight: Option<Decimal>,
    pub created_at: DateTime<Utc>,
}
```

**Table:** `asset_taxonomy_assignments`
**Repository:** `TaxonomyRepository` (embedded)
**Purpose:** Links an asset to a taxonomy category, optionally with a weight (for
multi-category assignments).

### 2.15 AllocationTarget

```rust
pub struct AllocationTarget {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub scope: ScopeType,
    pub scope_id: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Table:** `allocation_targets`
**Repository:** `AllocationTargetRepository` (in `allocation_target_repository.rs`)
**Purpose:** A named allocation target with weights and constraints.

### 2.16 AllocationTargetWeight

```rust
pub struct AllocationTargetWeight {
    pub id: String,
    pub target_id: String,
    pub subject_type: ConstraintSubjectType,
    pub subject_id: String,
    pub target_percent: Decimal,
    pub created_at: DateTime<Utc>,
}
```

**Table:** `allocation_target_weights`
**Repository:** `AllocationTargetRepository` (embedded)
**Purpose:** A target weight for a specific subject (asset, category, etc.) within an
allocation target.

### 2.17 AllocationTargetConstraint

```rust
pub struct AllocationTargetConstraint {
    pub id: String,
    pub target_id: String,
    pub subject_type: ConstraintSubjectType,
    pub subject_id: Option<String>,
    pub action: ConstraintAction,
    pub effect: ConstraintEffect,
    pub value: Option<Decimal>,
    pub created_at: DateTime<Utc>,
}
```

**Table:** `allocation_target_constraints`
**Repository:** `AllocationTargetRepository` (embedded)
**Purpose:** A constraint on an allocation target (e.g. "max 10% in any single stock").

---

## 3. Create Input Structs

Each domain struct has a corresponding `Create*Input` struct in the same file.
These are used by repositories and will be used by Tauri commands.

| Input Struct | Key Fields |
|-------------|------------|
| `CreatePlatformInput` | `name: Option<String>`, `url: String`, `kind: String` |
| `CreateAccountInput` | `workspace_id: Option<String>`, `name: String`, `account_type: AccountType`, `currency: Option<String>`, `tracking_mode: TrackingMode` |
| `CreateAssetInput` | `kind: AssetKind`, `display_code: String`, `quote_mode: QuoteMode`, `instrument_type: Option<InstrumentType>` |
| `UpsertQuoteInput` | `asset_id: String`, `date: NaiveDate`, `close: Option<Decimal>`, etc. |
| `CreateImportRunInput` | `account_id: String`, `status: String` |
| `CreateActivityInput` | `account_id: String`, `activity_type: ActivityType`, `activity_date: NaiveDate`, `quantity: Option<Decimal>`, `unit_price: Option<Decimal>`, `amount: Option<Decimal>`, `idempotency_key: Option<String>` |
| `CreateLotInput` | `account_id: String`, `asset_id: String`, `open_date: NaiveDate`, `quantity: Decimal`, `cost_basis: Decimal` |
| `CreateLotDisposalInput` | `lot_id: String`, `account_id: String`, `asset_id: String`, `disposal_date: NaiveDate`, `quantity: Decimal`, `proceeds: Decimal`, `cost_basis: Decimal`, `realized_gain: Decimal` |
| `CreateSnapshotInput` | `account_id: String`, `snapshot_date: NaiveDate`, `source: HoldingSnapshotSource`, `positions: Vec<SnapshotPositionInput>` |
| `UpsertValuationInput` | `account_id: String`, `valuation_date: NaiveDate`, `total_value: Decimal`, `status: ValuationStatus` |
| `CreateTaxonomyInput` | `name: String`, `description: Option<String>` |
| `CreateTaxonomyCategoryInput` | `taxonomy_id: String`, `name: String`, `color: Option<String>`, `sort_order: Option<i64>` |
| `AssetTaxonomyAssignmentInput` | `asset_id: String`, `category_id: String`, `weight: Option<Decimal>` |
| `CreateAllocationTargetInput` | `name: String`, `scope: ScopeType`, `scope_id: Option<String>` |
| `AllocationTargetWeightInput` | `target_id: String`, `subject_type: ConstraintSubjectType`, `subject_id: String`, `target_percent: Decimal` |
| `AllocationTargetConstraintInput` | `target_id: String`, `subject_type: ConstraintSubjectType`, `action: ConstraintAction`, `effect: ConstraintEffect`, `value: Option<Decimal>` |