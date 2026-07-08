mod prelude;
use prelude::*;

use bc_indicators::repeat::REPEAT;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i], CLOSE[i]])
        .collect::<Vec<Vec<f64>>>()
});

fn repeat_bf_1(c: &mut Criterion) {
    let ind = REPEAT::new(1.0);
    let bf = ind.bf(&IN_);
    c.bench_function("repeat_bf_1", |b| {
        b.iter(|| ind.ind_with_bf(&[OPEN_LAST, CLOSE_LAST], &bf, 0))
    });
}

fn repeat_f_1(c: &mut Criterion) {
    let ind = REPEAT::new(1.0);
    c.bench_function("repeat_f_1", |b| b.iter(|| ind.ind_f(&IN_)));
}

fn repeat_coll_1(c: &mut Criterion) {
    let ind = REPEAT::new(1.0);
    c.bench_function("repeat_coll_1", |b| {
        b.iter(|| ind.ind_coll::<Vec<f64>>(&IN_))
    });
}

criterion_group!(benches, repeat_bf_1, repeat_f_1, repeat_coll_1);
criterion_main!(benches);
