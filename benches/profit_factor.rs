use std::sync::LazyLock;

use bc_utils_lg::statics::prices::OPEN_LAST;
use criterion::{Criterion, criterion_group, criterion_main};

use bc_indicators::{
    profit_factor::PROFIT_FACTOR,
    ready_imports::{Indicator, IndicatorExt},
};

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![1.0, 2.0, -1.0]; 3]);

fn profit_factor_bf_1(c: &mut Criterion) {
    let ind = PROFIT_FACTOR::new();
    let bf = ind.bf(&IN_);
    c.bench_function("profit_factor_bf_1", |b| {
        b.iter(|| ind.ind_with_bf(&[OPEN_LAST], &bf, 0))
    });
}

fn profit_factor_f_1(c: &mut Criterion) {
    let ind = PROFIT_FACTOR::new();
    c.bench_function("profit_factor_f_1", |b| b.iter(|| ind.ind_f(&IN_)));
}

fn profit_factor_coll_1(c: &mut Criterion) {
    let ind = PROFIT_FACTOR::new();
    c.bench_function("profit_factor_coll_1", |b| {
        b.iter(|| ind.ind_coll::<Vec<f64>>(&IN_))
    });
}

criterion_group!(
    benches,
    profit_factor_bf_1,
    profit_factor_f_1,
    profit_factor_coll_1
);
criterion_main!(benches);
