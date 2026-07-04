use std::sync::LazyLock;

use bc_utils_lg::statics::prices::{CLOSE, CLOSE_LAST, OPEN, OPEN_LAST};
use criterion::{Criterion, criterion_group, criterion_main};

use bc_indicators::{
    percent::PERCENT,
    ready_imports::{Indicator, IndicatorExt},
};

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i], CLOSE[i]])
        .collect::<Vec<Vec<f64>>>()
});

fn percent_bf_1(c: &mut Criterion) {
    let ind = PERCENT::new();
    let bf = ind.bf(&IN_);
    c.bench_function("percent_bf_1", |b| {
        b.iter(|| ind.ind_with_bf(&[OPEN_LAST, CLOSE_LAST], &bf, 0))
    });
}

fn percent_f_1(c: &mut Criterion) {
    let ind = PERCENT::new();
    c.bench_function("percent_f_1", |b| b.iter(|| ind.ind_f(&IN_)));
}

fn percent_coll_1(c: &mut Criterion) {
    let ind = PERCENT::new();
    c.bench_function("percent_coll_1", |b| {
        b.iter(|| ind.ind_coll::<Vec<f64>>(&IN_))
    });
}

criterion_group!(benches, percent_bf_1, percent_f_1, percent_coll_1);
criterion_main!(benches);
