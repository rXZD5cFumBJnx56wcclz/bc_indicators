use std::sync::LazyLock;

use bc_utils_lg::statics::prices::{CLOSE, CLOSE_LAST, OPEN, OPEN_LAST};
use criterion::{Criterion, criterion_group, criterion_main};

use bc_indicators::indicators::{
    minus::MINUS,
    ready_imports::{Indicator, IndicatorExt},
};

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i], CLOSE[i]])
        .collect::<Vec<Vec<f64>>>()
});

fn minus_bf_1(c: &mut Criterion) {
    let ind = MINUS::new();
    let bf = ind.bf(&IN_);
    c.bench_function("minus_bf_1", |b| {
        b.iter(|| ind.ind_with_bf(&[OPEN_LAST, CLOSE_LAST], &bf, 0))
    });
}

fn minus_f_1(c: &mut Criterion) {
    let ind = MINUS::new();
    c.bench_function("minus_f_1", |b| b.iter(|| ind.ind_f(&IN_)));
}

fn minus_coll_1(c: &mut Criterion) {
    let ind = MINUS::new();
    c.bench_function("minus_coll_1", |b| {
        b.iter(|| ind.ind_coll::<Vec<f64>>(&IN_))
    });
}

criterion_group!(benches, minus_bf_1, minus_f_1, minus_coll_1);
criterion_main!(benches);
