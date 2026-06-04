use std::sync::LazyLock;

use bc_utils_lg::statics::prices::OPEN_LAST;
use criterion::{Criterion, criterion_group, criterion_main};

use bc_indicators::indicators::{
    osc_mult::OSC_MULT,
    ready_imports::{Indicator, IndicatorExt},
};

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![85.0]; 5]);

fn osc_mult_bf_1(c: &mut Criterion) {
    let ind = OSC_MULT::new(15.0, 15.0, 100.0);
    let bf = ind.bf(&IN_);
    c.bench_function("osc_mult_bf_1", |b| {
        b.iter(|| ind.ind_with_bf(&[OPEN_LAST], &bf, 0))
    });
}

fn osc_mult_f_1(c: &mut Criterion) {
    let ind = OSC_MULT::new(15.0, 15.0, 100.0);
    c.bench_function("osc_mult_f_1", |b| b.iter(|| ind.ind_f(&IN_)));
}

fn osc_mult_coll_1(c: &mut Criterion) {
    let ind = OSC_MULT::new(15.0, 15.0, 100.0);
    c.bench_function("osc_mult_coll_1", |b| {
        b.iter(|| ind.ind_coll::<Vec<f64>>(&IN_))
    });
}

criterion_group!(benches, osc_mult_bf_1, osc_mult_f_1, osc_mult_coll_1);
criterion_main!(benches);
