use std::sync::LazyLock;

use bc_utils_lg::statics::prices::{CLOSE, CLOSE_LAST, OPEN, OPEN_LAST};
use criterion::{Criterion, criterion_group, criterion_main};

use bc_indicators::indicators::{
    plus::PLUS,
    ready_imports::{Indicator, IndicatorExt},
};

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i], CLOSE[i]])
        .collect::<Vec<Vec<f64>>>()
});

fn plus_bf_1(c: &mut Criterion) {
    let ind = PLUS::new();
    let bf = ind.bf(&IN_);
    c.bench_function("plus_bf_1", |b| {
        b.iter(|| ind.ind_with_bf(&[OPEN_LAST, CLOSE_LAST], &bf, 0))
    });
}

fn plus_f_1(c: &mut Criterion) {
    let ind = PLUS::new();
    c.bench_function("plus_f_1", |b| b.iter(|| ind.ind_f(&IN_)));
}

fn plus_coll_1(c: &mut Criterion) {
    let ind = PLUS::new();
    c.bench_function("plus_coll_1", |b| b.iter(|| ind.ind_coll::<Vec<f64>>(&IN_)));
}

criterion_group!(benches, plus_bf_1, plus_f_1, plus_coll_1);
criterion_main!(benches);
