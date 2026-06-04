use std::sync::LazyLock;

use bc_utils_lg::statics::prices::{OPEN, OPEN_LAST};
use criterion::{Criterion, criterion_group, criterion_main};

use bc_indicators::indicators::{
    ema::EMA,
    ready_imports::{Indicator, IndicatorExt},
};

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    OPEN.iter()
        .copied()
        .map(|v| vec![v])
        .collect::<Vec<Vec<f64>>>()
});

fn ema_bf_1(c: &mut Criterion) {
    let ind = EMA::new(4);
    let bf = ind.bf(&IN_);
    c.bench_function("ema_bf_1", |b| {
        b.iter(|| ind.ind_with_bf(&[OPEN_LAST], &bf, 0))
    });
}

fn ema_f_1(c: &mut Criterion) {
    let ind = EMA::new(4);
    c.bench_function("ema_f_1", |b| b.iter(|| ind.ind_f(&IN_)));
}

fn ema_coll_1(c: &mut Criterion) {
    let ind = EMA::new(4);
    c.bench_function("ema_coll_1", |b| b.iter(|| ind.ind_coll::<Vec<f64>>(&IN_)));
}

criterion_group!(benches, ema_bf_1, ema_f_1, ema_coll_1);
criterion_main!(benches);
