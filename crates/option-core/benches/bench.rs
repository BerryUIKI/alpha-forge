//! Option core benchmarks.
//!
//! Run with:
//!   cargo bench -p option-core
//!
//! Results depend on hardware; record the environment (see
//! `docs/option/ROADMAP.md` Phase 2 verification notes) alongside
//! reported numbers.

use criterion::{criterion_group, criterion_main, Criterion};
use domain::option::OptionType;

use option_core::{
    black_scholes_price, calculate_greeks, calculate_implied_volatility,
    calculate_payoff_at_expiry, StrategyLeg,
};

fn bench_black_scholes(c: &mut Criterion) {
    c.bench_function("black_scholes_price_atm_call", |b| {
        b.iter(|| black_scholes_price(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap())
    });
}

fn bench_greeks(c: &mut Criterion) {
    c.bench_function("calculate_greeks_atm_call", |b| {
        b.iter(|| calculate_greeks(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.2, 0.0).unwrap())
    });
}

fn bench_implied_volatility(c: &mut Criterion) {
    let market_price =
        black_scholes_price(OptionType::Call, 100.0, 100.0, 1.0, 0.05, 0.25, 0.0).unwrap();

    c.bench_function("calculate_implied_volatility_from_25pct", move |b| {
        b.iter(|| {
            calculate_implied_volatility(
                OptionType::Call,
                100.0,
                100.0,
                1.0,
                0.05,
                0.0,
                market_price,
                100,
                0.0001,
            )
            .unwrap()
        })
    });
}

fn bench_strategy_payoff(c: &mut Criterion) {
    let legs = vec![
        StrategyLeg {
            option_type: OptionType::Call,
            strike: 100.0,
            expiration: 1.0,
            quantity: 1,
            premium: 5.0,
        },
        StrategyLeg {
            option_type: OptionType::Call,
            strike: 110.0,
            expiration: 1.0,
            quantity: -1,
            premium: 2.0,
        },
    ];

    c.bench_function("strategy_payoff_two_legs", |b| {
        b.iter(|| calculate_payoff_at_expiry(&legs, 105.0).unwrap())
    });
}

criterion_group!(
    benches,
    bench_black_scholes,
    bench_greeks,
    bench_implied_volatility,
    bench_strategy_payoff
);
criterion_main!(benches);
