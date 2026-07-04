use std::sync::LazyLock;

use bc_utils_lg::statics::prices::{CLOSE, CLOSE_LAST, OPEN, OPEN_LAST};
use criterion::{Criterion, criterion_group, criterion_main};

use bc_indicators::{
    div::DIV,
    ready_imports::{Indicator, IndicatorExt},
};

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i], CLOSE[i]])
        .collect::<Vec<Vec<f64>>>()
});

fn div_bf_1(c: &mut Criterion) {
    let ind = DIV::new();
    let bf = ind.bf(&IN_);
    c.bench_function("div_bf_1", |b| {
        b.iter(|| ind.ind_with_bf(&[OPEN_LAST, CLOSE_LAST], &bf, 0))
    });
}

fn div_f_1(c: &mut Criterion) {
    let ind = DIV::new();
    c.bench_function("div_f_1", |b| b.iter(|| ind.ind_f(&IN_)));
}

fn div_coll_1(c: &mut Criterion) {
    let ind = DIV::new();
    c.bench_function("div_coll_1", |b| b.iter(|| ind.ind_coll::<Vec<f64>>(&IN_)));
}

criterion_group!(benches, div_bf_1, div_f_1, div_coll_1);
criterion_main!(benches);
