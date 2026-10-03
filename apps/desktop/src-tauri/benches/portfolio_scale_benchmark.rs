//! Large-Portfolio Scale and Performance Benchmarks (M11-03)
//!
//! Measures performance across representative large-portfolio workloads:
//! 1. Statement CSV Parsing & Preparation (1,000 rows)
//! 2. Holdings Aggregation with Multi-Currency & FX (5 accounts, 50 assets, 500+ lots)
//! 3. Daily Portfolio Valuation Calculation across multiple accounts
//! 4. Performance Summary (XIRR polynomial & TWR compounding across 250+ periods)
//! 5. Taxonomy Allocation & Constraint Verification (50 positions)
//!
//! Run with:
//!   cargo bench -p alpha-forge --bench portfolio_scale_benchmark

use std::sync::Arc;
use std::time::Duration;

use chrono::NaiveDate;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rust_decimal::Decimal;
use sqlx::SqlitePool;

use alpha_forge_lib::database::repositories::account_repository::AccountRepository;
use alpha_forge_lib::database::repositories::activity_repository::{
    ActivityRepository, ImportRunRepository,
};
use alpha_forge_lib::database::repositories::allocation_target_repository::AllocationTargetRepository;
use alpha_forge_lib::database::repositories::asset_repository::{AssetRepository, QuoteRepository};
use alpha_forge_lib::database::repositories::lot_repository::{
    LotDisposalRepository, LotRepository,
};
use alpha_forge_lib::database::repositories::taxonomy_repository::TaxonomyRepository;
use alpha_forge_lib::database::repositories::test_support::setup_test_db;
use alpha_forge_lib::database::repositories::valuation_repository::ValuationRepository;

use alpha_forge_lib::services::activity_import_service::ActivityImportService;
use alpha_forge_lib::services::activity_service::ActivityService;
use alpha_forge_lib::services::allocation_service::AllocationService;
use alpha_forge_lib::services::holdings_service::HoldingsService;
use alpha_forge_lib::services::performance_service::PerformanceService;
use alpha_forge_lib::services::valuation_service::ValuationService;

use domain::financial::{
    AccountType, ActivityStatus, ActivityType, AllocationTargetConstraintInput,
    AllocationTargetWeightInput, AssetKind, AssetTaxonomyAssignmentInput, BasisStatus,
    ConstraintAction, ConstraintEffect, ConstraintSubjectType, CostBasisMethod, CreateAccountInput,
    CreateActivityInput, CreateAllocationTargetInput, CreateAssetInput, CreateLotInput,
    CreateTaxonomyCategoryInput, CreateTaxonomyInput, ExternalFlowSource, InstrumentType,
    QuoteMode, ScopeType, TrackingMode, UpsertQuoteInput, UpsertValuationInput, ValuationStatus,
};

fn dec(value: &str) -> Decimal {
    Decimal::from_str_exact(value).expect("valid decimal")
}

struct BenchmarkContext {
    #[allow(dead_code)]
    pub pool: SqlitePool,
    pub account_ids: Vec<String>,
    #[allow(dead_code)]
    pub asset_ids: Vec<String>,
    pub holdings_service: Arc<HoldingsService>,
    pub valuation_service: ValuationService,
    pub performance_service: PerformanceService,
    pub allocation_service: AllocationService,
    pub activity_import_service: ActivityImportService,
    pub generic_csv_1000: String,
    pub ibkr_csv_1000: String,
}

async fn setup_large_portfolio_fixtures() -> BenchmarkContext {
    let pool = setup_test_db().await;

    let account_repo = Arc::new(AccountRepository::new(pool.clone()));
    let asset_repo = Arc::new(AssetRepository::new(pool.clone()));
    let quote_repo = Arc::new(QuoteRepository::new(pool.clone()));
    let activity_repo = Arc::new(ActivityRepository::new(pool.clone()));
    let import_run_repo = Arc::new(ImportRunRepository::new(pool.clone()));
    let lot_repo = Arc::new(LotRepository::new(pool.clone()));
    let disposal_repo = Arc::new(LotDisposalRepository::new(pool.clone()));
    let valuation_repo = Arc::new(ValuationRepository::new(pool.clone()));
    let taxonomy_repo = Arc::new(TaxonomyRepository::new(pool.clone()));
    let target_repo = Arc::new(AllocationTargetRepository::new(pool.clone()));

    // 1. Create 5 Accounts across different currencies and types
    let account_names = [
        ("US Prime Brokerage", AccountType::Securities, "USD"),
        ("European Growth Portfolio", AccountType::Securities, "EUR"),
        ("High-Yield Cash Vault", AccountType::Cash, "USD"),
        ("Institutional Crypto Fund", AccountType::Securities, "USD"),
        ("Retirement Rollover IRA", AccountType::Securities, "USD"),
    ];

    let mut account_ids = Vec::new();
    for (name, acc_type, curr) in account_names {
        let acc = account_repo
            .create(CreateAccountInput {
                workspace_id: None,
                name: name.to_string(),
                account_type: acc_type,
                group_name: Some("Benchmark Group".to_string()),
                currency: curr.to_string(),
                is_default: account_ids.is_empty(),
                platform_id: None,
                account_number: Some(format!("ACT-{:04}", account_ids.len() + 1)),
                tracking_mode: TrackingMode::Transactions,
            })
            .await
            .expect("create account");
        account_ids.push(acc.id);
    }

    // 2. Create 50 Assets (Equities, ETFs, Crypto, FX pairs)
    let equity_symbols = [
        "AAPL", "MSFT", "GOOGL", "AMZN", "NVDA", "META", "TSLA", "BRKB", "JPM", "JNJ", "V", "PG",
        "XOM", "UNH", "HD", "MA", "LLY", "ABBV", "MRK", "PEP", "KO", "COST", "AVGO", "TMO", "MCD",
        "CSCO", "ACN", "WMT", "BAC", "CRM", "ABT", "LIN", "DIS", "NKE", "AMD",
    ];
    let etf_symbols = ["SPY", "QQQ", "VTI", "BND", "GLD"];
    let crypto_symbols = ["BTC", "ETH", "SOL", "AVAX", "LINK"];
    let fx_pairs = ["EUR/USD", "GBP/USD", "JPY/USD", "CAD/USD", "CHF/USD"];

    let mut asset_ids = Vec::new();

    // Insert Equities
    for sym in equity_symbols {
        let asset = asset_repo
            .create(CreateAssetInput {
                kind: AssetKind::Investment,
                name: Some(format!("{} Inc.", sym)),
                display_code: Some(sym.to_string()),
                notes: None,
                is_active: true,
                quote_mode: QuoteMode::Market,
                quote_ccy: "USD".to_string(),
                instrument_type: Some(InstrumentType::Equity),
                instrument_symbol: Some(sym.to_string()),
                instrument_exchange_mic: Some("XNAS".to_string()),
                provider_config: None,
            })
            .await
            .expect("create equity asset");
        asset_ids.push(asset.id);
    }

    // Insert ETFs
    for sym in etf_symbols {
        let asset = asset_repo
            .create(CreateAssetInput {
                kind: AssetKind::Investment,
                name: Some(format!("{} Trust", sym)),
                display_code: Some(sym.to_string()),
                notes: None,
                is_active: true,
                quote_mode: QuoteMode::Market,
                quote_ccy: "USD".to_string(),
                instrument_type: Some(InstrumentType::Equity),
                instrument_symbol: Some(sym.to_string()),
                instrument_exchange_mic: Some("ARCX".to_string()),
                provider_config: None,
            })
            .await
            .expect("create etf asset");
        asset_ids.push(asset.id);
    }

    // Insert Crypto
    for sym in crypto_symbols {
        let asset = asset_repo
            .create(CreateAssetInput {
                kind: AssetKind::Investment,
                name: Some(format!("{} Token", sym)),
                display_code: Some(sym.to_string()),
                notes: None,
                is_active: true,
                quote_mode: QuoteMode::Market,
                quote_ccy: "USD".to_string(),
                instrument_type: Some(InstrumentType::Crypto),
                instrument_symbol: Some(sym.to_string()),
                instrument_exchange_mic: None,
                provider_config: None,
            })
            .await
            .expect("create crypto asset");
        asset_ids.push(asset.id);
    }

    // Insert FX pairs
    for pair in fx_pairs {
        let asset = asset_repo
            .create(CreateAssetInput {
                kind: AssetKind::Fx,
                name: Some(format!("{} Exchange Rate", pair)),
                display_code: Some(pair.to_string()),
                notes: None,
                is_active: true,
                quote_mode: QuoteMode::Market,
                quote_ccy: "USD".to_string(),
                instrument_type: Some(InstrumentType::Fx),
                instrument_symbol: Some(pair.to_string()),
                instrument_exchange_mic: None,
                provider_config: None,
            })
            .await
            .expect("create fx asset");
        asset_ids.push(asset.id);
    }

    // 3. Seed Quotes for all 50 assets across 250 trading days (2025)
    let start_date = NaiveDate::from_ymd_opt(2025, 1, 2).unwrap();
    for (i, asset_id) in asset_ids.iter().enumerate() {
        let base_price = Decimal::from(50 + (i * 7) % 300);
        for day in 0..250 {
            let quote_date = start_date + chrono::Duration::days(day as i64);
            let variation = Decimal::from((day % 15) as i64) / Decimal::from(10);
            let close = base_price + variation;

            quote_repo
                .upsert(UpsertQuoteInput {
                    asset_id: asset_id.clone(),
                    day: quote_date,
                    source: "benchmark_feed".to_string(),
                    open: Some(close - Decimal::ONE),
                    high: Some(close + Decimal::TWO),
                    low: Some(close - Decimal::TWO),
                    close,
                    adjclose: Some(close),
                    volume: Some(Decimal::from(1_000_000)),
                    currency: "USD".to_string(),
                    notes: None,
                })
                .await
                .expect("upsert quote");
        }
    }

    // Explicit FX Quote for EUR/USD to support European account conversion
    if let Some(eur_usd_id) = asset_ids.get(45) {
        for day in 0..250 {
            let quote_date = start_date + chrono::Duration::days(day as i64);
            quote_repo
                .upsert(UpsertQuoteInput {
                    asset_id: eur_usd_id.clone(),
                    day: quote_date,
                    source: "fx_feed".to_string(),
                    open: Some(dec("1.0840")),
                    high: Some(dec("1.0890")),
                    low: Some(dec("1.0810")),
                    close: dec("1.0850"),
                    adjclose: Some(dec("1.0850")),
                    volume: None,
                    currency: "USD".to_string(),
                    notes: None,
                })
                .await
                .expect("upsert fx quote");
        }
    }

    // 4. Seed 500 Lots across Account 0 and Account 1
    for i in 0..500 {
        let acc_id = if i % 2 == 0 {
            &account_ids[0]
        } else {
            &account_ids[1]
        };
        let asset_id = &asset_ids[i % 40];
        let lot_date = start_date + chrono::Duration::days((i % 200) as i64);

        lot_repo
            .create(CreateLotInput {
                account_id: acc_id.clone(),
                asset_id: asset_id.clone(),
                open_date: lot_date,
                open_activity_id: None,
                original_quantity: dec("25.0"),
                cost_per_unit: dec("50.0"),
                original_cost_basis: dec("1250.0"),
                fee_allocated: Decimal::ZERO,
                currency: "USD".to_string(),
                base_currency: "USD".to_string(),
                fx_rate_to_base: dec("1.0"),
                fx_rate_to_account: None,
                account_currency: None,
                cost_basis_method: CostBasisMethod::Fifo,
            })
            .await
            .expect("create lot");
    }

    // 5. Seed 1,000 Activities across Accounts
    for i in 0..1000 {
        let acc_id = &account_ids[i % account_ids.len()];
        let asset_id = &asset_ids[i % 40];
        let act_date = start_date + chrono::Duration::days((i % 240) as i64);

        let (act_type, qty, unit_price, amt) = match i % 5 {
            0 => (
                ActivityType::Deposit,
                Decimal::ZERO,
                Decimal::ZERO,
                dec("5000.0"),
            ),
            1 => (ActivityType::Buy, dec("10.0"), dec("150.0"), dec("1500.0")),
            2 => (
                ActivityType::Dividend,
                Decimal::ZERO,
                Decimal::ZERO,
                dec("45.0"),
            ),
            3 => (ActivityType::Sell, dec("5.0"), dec("160.0"), dec("800.0")),
            _ => (ActivityType::Fee, Decimal::ZERO, Decimal::ZERO, dec("1.50")),
        };

        activity_repo
            .create(CreateActivityInput {
                account_id: acc_id.clone(),
                asset_id: if act_type == ActivityType::Deposit || act_type == ActivityType::Fee {
                    None
                } else {
                    Some(asset_id.clone())
                },
                activity_type: act_type,
                activity_type_override: None,
                source_type: Some("TRADE".to_string()),
                subtype: None,
                status: ActivityStatus::Posted,
                activity_date: act_date,
                settlement_date: Some(act_date),
                quantity: Some(qty),
                unit_price: Some(unit_price),
                amount: Some(amt),
                fee: Some(dec("1.0")),
                tax: None,
                currency: "USD".to_string(),
                fx_rate: None,
                notes: Some(format!("Benchmark activity #{}", i)),
                metadata: None,
                source_system: Some("benchmark".to_string()),
                source_record_id: None,
                source_group_id: None,
                idempotency_key: Some(format!("bench-act-{}", i)),
                import_run_id: None,
            })
            .await
            .expect("create activity");
    }

    // 6. Seed 250 Valuations per account (1,250 valuations total)
    for acc_id in &account_ids {
        for day in 0..250 {
            let val_date = start_date + chrono::Duration::days(day as i64);
            let base_val = Decimal::from(100_000 + day * 150);
            let cash_val = Decimal::from(10_000 + (day % 30) * 100);

            valuation_repo
                .upsert(UpsertValuationInput {
                    account_id: acc_id.clone(),
                    valuation_date: val_date,
                    account_currency: "USD".to_string(),
                    base_currency: "USD".to_string(),
                    fx_rate_to_base: dec("1.0"),
                    cash_balance: cash_val,
                    investment_market_value: base_val - cash_val,
                    total_value: base_val,
                    cost_basis: base_val - dec("5000"),
                    net_contribution: base_val,
                    cash_balance_base: cash_val,
                    investment_market_value_base: base_val - cash_val,
                    total_value_base: base_val,
                    cost_basis_base: base_val - dec("5000"),
                    net_contribution_base: base_val,
                    external_inflow_base: if day % 30 == 0 {
                        dec("5000.0")
                    } else {
                        Decimal::ZERO
                    },
                    external_outflow_base: Decimal::ZERO,
                    performance_eligible_value_base: base_val,
                    external_flow_source: ExternalFlowSource::ActivityDerived,
                    value_status: ValuationStatus::Complete,
                    basis_status: BasisStatus::Complete,
                })
                .await
                .expect("upsert valuation");
        }
    }

    // 7. Seed Taxonomy & Allocation Target (50 assets mapped across 4 categories)
    let taxonomy = taxonomy_repo
        .create(CreateTaxonomyInput {
            name: "Asset Allocation Model".to_string(),
            color: "#3b82f6".to_string(),
            description: Some("Benchmark asset class taxonomy".to_string()),
            is_system: false,
            is_single_select: true,
            sort_order: 0,
        })
        .await
        .expect("create taxonomy");

    let categories = [
        ("US Equities", "#3b82f6"),
        ("Fixed Income", "#10b981"),
        ("Cryptocurrency", "#f59e0b"),
        ("International & FX", "#8b5cf6"),
    ];
    let mut cat_ids = Vec::new();
    for (cat_name, color) in categories {
        let cat = taxonomy_repo
            .create_category(CreateTaxonomyCategoryInput {
                taxonomy_id: taxonomy.id.clone(),
                parent_id: None,
                name: cat_name.to_string(),
                key: cat_name.to_lowercase().replace(' ', "_"),
                color: color.to_string(),
                description: None,
                sort_order: 0,
            })
            .await
            .expect("create category");
        cat_ids.push(cat.id);
    }

    // Assign assets to categories
    for (i, asset_id) in asset_ids.iter().enumerate() {
        let cat_id = &cat_ids[i % cat_ids.len()];
        taxonomy_repo
            .assign_asset(AssetTaxonomyAssignmentInput {
                asset_id: asset_id.clone(),
                taxonomy_id: taxonomy.id.clone(),
                category_id: cat_id.clone(),
                weight: 10000,
                source: "benchmark".to_string(),
            })
            .await
            .expect("assign asset");
    }

    let target = target_repo
        .create(CreateAllocationTargetInput {
            name: "Target Balanced 60/40".to_string(),
            scope_type: ScopeType::All,
            scope_id: None,
            taxonomy_id: taxonomy.id.clone(),
            trigger_type: "manual".to_string(),
            drift_band_bps: 5000,
            rebalance_goal: "nearest_band".to_string(),
            min_trade_amount: dec("0"),
            whole_shares_only: false,
            allow_sells: true,
            max_turnover_bps: None,
        })
        .await
        .expect("create target");

    let target_weights = [
        (cat_ids[0].clone(), 6000),
        (cat_ids[1].clone(), 2000),
        (cat_ids[2].clone(), 1000),
        (cat_ids[3].clone(), 1000),
    ];
    for (cat_id, bps) in target_weights {
        target_repo
            .add_weight(AllocationTargetWeightInput {
                target_id: target.id.clone(),
                taxonomy_id: taxonomy.id.clone(),
                category_id: cat_id,
                target_bps: bps,
                is_locked: false,
                is_required: false,
            })
            .await
            .expect("add weight");
    }

    target_repo
        .add_constraint(AllocationTargetConstraintInput {
            target_id: target.id.clone(),
            subject_type: ConstraintSubjectType::Category,
            subject_id: cat_ids[0].clone(),
            action: ConstraintAction::Buy,
            effect: ConstraintEffect::Block,
            reason: Some("Category allocation upper bound".to_string()),
            metadata_json: None,
        })
        .await
        .expect("add constraint");

    // 8. Generate 1,000-row Generic CSV and IBKR CSV
    let mut generic_csv =
        String::from("date,type,symbol,quantity,price,amount,fee,currency,notes\n");
    for i in 0..1000 {
        let sym = equity_symbols[i % equity_symbols.len()];
        let date_str = "2025-06-15";
        if i % 3 == 0 {
            generic_csv.push_str(&format!(
                "{},BUY,{},10,150.25,1503.50,1.00,USD,Import row #{}\n",
                date_str, sym, i
            ));
        } else if i % 3 == 1 {
            generic_csv.push_str(&format!(
                "{},DIVIDEND,{},0,0.00,45.00,0.00,USD,Dividend row #{}\n",
                date_str, sym, i
            ));
        } else {
            generic_csv.push_str(&format!(
                "{},SELL,{},5,160.50,801.50,1.00,USD,Sell row #{}\n",
                date_str, sym, i
            ));
        }
    }

    let mut ibkr_csv = String::from(
        "\"Trades\",\"Header\",\"DataDiscriminator\",\"Asset Category\",\"Currency\",\"Symbol\",\"Date/Time\",\"Quantity\",\"T. Price\",\"Proceeds\",\"Comm/Fee\",\"Code\"\n"
    );
    for i in 0..1000 {
        let sym = equity_symbols[i % equity_symbols.len()];
        let date_str = "2025-06-15, 10:30:00";
        let qty = if i % 2 == 0 { 20 } else { -10 };
        ibkr_csv.push_str(&format!(
            "\"Trades\",\"Data\",\"Order\",\"Stocks\",\"USD\",\"{}\",\"{}\",\"{}\",\"155.00\",\"1550.00\",\"-1.00\",\"O\"\n",
            sym, date_str, qty
        ));
    }

    // Build services
    let holdings_service = Arc::new(HoldingsService::new(
        account_repo.clone(),
        asset_repo.clone(),
        quote_repo.clone(),
        lot_repo.clone(),
        disposal_repo.clone(),
    ));

    let valuation_service = ValuationService::new(
        valuation_repo.clone(),
        account_repo.clone(),
        holdings_service.clone(),
    );

    let performance_service = PerformanceService::new(valuation_repo.clone(), account_repo.clone());

    let allocation_service = AllocationService::new(
        taxonomy_repo.clone(),
        target_repo.clone(),
        account_repo.clone(),
        holdings_service.clone(),
    );

    let activity_service = Arc::new(ActivityService::new(
        pool.clone(),
        activity_repo.clone(),
        account_repo.clone(),
        asset_repo.clone(),
        lot_repo.clone(),
        disposal_repo.clone(),
    ));

    let activity_import_service = ActivityImportService::new(
        account_repo.clone(),
        asset_repo.clone(),
        activity_repo.clone(),
        import_run_repo.clone(),
        lot_repo.clone(),
        activity_service.clone(),
    );

    BenchmarkContext {
        pool,
        account_ids,
        asset_ids,
        holdings_service,
        valuation_service,
        performance_service,
        allocation_service,
        activity_import_service,
        generic_csv_1000: generic_csv,
        ibkr_csv_1000: ibkr_csv,
    }
}

fn bench_holdings_aggregation(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let ctx = rt.block_on(setup_large_portfolio_fixtures());
    let as_of_date = NaiveDate::from_ymd_opt(2025, 9, 1).unwrap();
    let single_acc_id = ctx.account_ids[0].clone();

    let mut group = c.benchmark_group("holdings_aggregation");
    group.measurement_time(Duration::from_secs(3));
    group.sample_size(15);

    group.bench_function(
        BenchmarkId::new("single_account_250_lots", &single_acc_id),
        |b| {
            b.to_async(&rt).iter(|| async {
                ctx.holdings_service
                    .get_holdings(&single_acc_id, as_of_date)
                    .await
                    .expect("get_holdings");
            });
        },
    );

    group.bench_function(BenchmarkId::new("all_accounts_multi_currency", 5), |b| {
        b.to_async(&rt).iter(|| async {
            ctx.holdings_service
                .get_all_holdings(as_of_date)
                .await
                .expect("get_all_holdings");
        });
    });

    group.finish();
}

fn bench_valuation_calculation(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let ctx = rt.block_on(setup_large_portfolio_fixtures());
    let val_date = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();
    let single_acc_id = ctx.account_ids[0].clone();

    let mut group = c.benchmark_group("valuation_calculation");
    group.measurement_time(Duration::from_secs(3));
    group.sample_size(15);

    group.bench_function(
        BenchmarkId::new("single_account_daily", &single_acc_id),
        |b| {
            b.to_async(&rt).iter(|| async {
                ctx.valuation_service
                    .calculate_day(&single_acc_id, val_date)
                    .await
                    .expect("calculate_day");
            });
        },
    );

    group.bench_function(BenchmarkId::new("all_accounts_daily", 5), |b| {
        b.to_async(&rt).iter(|| async {
            ctx.valuation_service
                .calculate_all(val_date)
                .await
                .expect("calculate_all");
        });
    });

    group.finish();
}

fn bench_performance_summary(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let ctx = rt.block_on(setup_large_portfolio_fixtures());
    let start_date = NaiveDate::from_ymd_opt(2025, 1, 2).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2025, 9, 1).unwrap();
    let single_acc_id = ctx.account_ids[0].clone();

    let mut group = c.benchmark_group("performance_computation");
    group.measurement_time(Duration::from_secs(3));
    group.sample_size(15);

    group.bench_function(
        BenchmarkId::new("xirr_and_twr_250_periods", &single_acc_id),
        |b| {
            b.to_async(&rt).iter(|| async {
                ctx.performance_service
                    .compute_summary(&single_acc_id, start_date, end_date)
                    .await
                    .expect("compute_summary");
            });
        },
    );

    group.finish();
}

fn bench_allocation_and_constraints(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let ctx = rt.block_on(setup_large_portfolio_fixtures());
    let as_of_date = NaiveDate::from_ymd_opt(2025, 9, 1).unwrap();

    let mut group = c.benchmark_group("allocation_analysis");
    group.measurement_time(Duration::from_secs(3));
    group.sample_size(15);

    group.bench_function("all_scope_50_assets", |b| {
        b.to_async(&rt).iter(|| async {
            ctx.allocation_service
                .get_allocation(ScopeType::All, None, as_of_date)
                .await
                .expect("get_allocation");
        });
    });

    group.bench_function("constraint_checks_all_scope", |b| {
        b.to_async(&rt).iter(|| async {
            ctx.allocation_service
                .check_constraints(ScopeType::All, None, as_of_date)
                .await
                .expect("check_constraints");
        });
    });

    group.finish();
}

fn bench_csv_import_pipeline(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let ctx = rt.block_on(setup_large_portfolio_fixtures());
    let target_account_id = ctx.account_ids[0].clone();

    let mut group = c.benchmark_group("csv_statement_import");
    group.measurement_time(Duration::from_secs(3));
    group.sample_size(10);

    let csv_generic = ctx.generic_csv_1000.clone();
    let acc_id = target_account_id.clone();
    group.bench_function("generic_csv_1000_rows", |b| {
        b.to_async(&rt).iter(|| async {
            ctx.activity_import_service
                .import_csv(&acc_id, "GENERIC_CSV", &csv_generic)
                .await
                .expect("import generic csv");
        });
    });

    let csv_ibkr = ctx.ibkr_csv_1000.clone();
    let acc_id2 = target_account_id.clone();
    group.bench_function("ibkr_csv_1000_rows", |b| {
        b.to_async(&rt).iter(|| async {
            ctx.activity_import_service
                .import_csv(&acc_id2, "IBKR_CSV", &csv_ibkr)
                .await
                .expect("import ibkr csv");
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_holdings_aggregation,
    bench_valuation_calculation,
    bench_performance_summary,
    bench_allocation_and_constraints,
    bench_csv_import_pipeline
);
criterion_main!(benches);
