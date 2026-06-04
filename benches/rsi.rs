use std::sync::LazyLock;

use bc_utils_lg::statics::prices::{OPEN, OPEN_LAST};
use criterion::{Criterion, criterion_group, criterion_main};

use bc_indicators::indicators::{
    ready_imports::{Indicator, IndicatorExt},
    rsi::RSI,
};

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    OPEN.iter()
        .copied()
        .map(|v| vec![v])
        .collect::<Vec<Vec<f64>>>()
});

fn rsi_bf_1(c: &mut Criterion) {
    let ind = RSI::new(4);
    let bf = ind.bf(&IN_);
    c.bench_function("rsi_bf_1", |b| {
        b.iter(|| ind.ind_with_bf(&[OPEN_LAST], &bf, 0))
    });
}

fn rsi_f_1(c: &mut Criterion) {
    let ind = RSI::new(4);
    c.bench_function("rsi_f_1", |b| b.iter(|| ind.ind_f(&IN_)));
}

fn rsi_coll_1(c: &mut Criterion) {
    let ind = RSI::new(4);
    c.bench_function("rsi_coll_1", |b| b.iter(|| ind.ind_coll::<Vec<f64>>(&IN_)));
}

criterion_group!(benches, rsi_bf_1, rsi_f_1, rsi_coll_1);
criterion_main!(benches);
