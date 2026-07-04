use std::sync::LazyLock;

use bc_utils_lg::statics::prices::{OPEN, OPEN_LAST};
use criterion::{Criterion, criterion_group, criterion_main};

use bc_indicators::{
    ready_imports::{Indicator, IndicatorExt},
    trend_ma::TREND_MA,
};

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    OPEN.iter()
        .copied()
        .map(|v| vec![v])
        .collect::<Vec<Vec<f64>>>()
});

fn trend_ma_bf_1(c: &mut Criterion) {
    let ind = TREND_MA::new();
    let bf = ind.bf(&IN_);
    c.bench_function("trend_ma_bf_1", |b| {
        b.iter(|| ind.ind_with_bf(&[OPEN_LAST], &bf, 0))
    });
}

fn trend_ma_f_1(c: &mut Criterion) {
    let ind = TREND_MA::new();
    c.bench_function("trend_ma_f_1", |b| b.iter(|| ind.ind_f(&IN_)));
}

fn trend_ma_coll_1(c: &mut Criterion) {
    let ind = TREND_MA::new();
    c.bench_function("trend_ma_coll_1", |b| {
        b.iter(|| ind.ind_coll::<Vec<f64>>(&IN_))
    });
}

criterion_group!(benches, trend_ma_bf_1, trend_ma_f_1, trend_ma_coll_1);
criterion_main!(benches);
