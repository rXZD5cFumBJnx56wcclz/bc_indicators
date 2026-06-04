use std::sync::LazyLock;

use bc_utils_lg::statics::prices::{OPEN, OPEN_LAST};
use criterion::{Criterion, criterion_group, criterion_main};

use bc_indicators::indicators::{
    mm_scaler::MM_SCALER,
    ready_imports::{Indicator, IndicatorExt},
};

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    OPEN.iter()
        .copied()
        .map(|v| vec![v])
        .collect::<Vec<Vec<f64>>>()
});

fn mm_scaler_bf_1(c: &mut Criterion) {
    let ind = MM_SCALER::new(20);
    let bf = ind.bf(&IN_);
    c.bench_function("mm_scaler_bf_1", |b| {
        b.iter(|| ind.ind_with_bf(&[OPEN_LAST], &bf, 0))
    });
}

fn mm_scaler_f_1(c: &mut Criterion) {
    let ind = MM_SCALER::new(20);
    c.bench_function("mm_scaler_f_1", |b| b.iter(|| ind.ind_f(&IN_)));
}

fn mm_scaler_coll_1(c: &mut Criterion) {
    let ind = MM_SCALER::new(20);
    c.bench_function("mm_scaler_coll_1", |b| {
        b.iter(|| ind.ind_coll::<Vec<f64>>(&IN_))
    });
}

criterion_group!(benches, mm_scaler_bf_1, mm_scaler_f_1, mm_scaler_coll_1);
criterion_main!(benches);
