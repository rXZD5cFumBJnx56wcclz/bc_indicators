mod prelude;
use prelude::*;

use bc_indicators::avg::AVG;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i], CLOSE[i], HIGH[i], LOW[i]])
        .collect::<Vec<Vec<f64>>>()
});

fn avg_bf_1(c: &mut Criterion) {
    let ind = AVG::new();
    let bf = ind.bf(&IN_);
    c.bench_function("avg_bf_1", |b| {
        b.iter(|| ind.ind_with_bf(&[OPEN_LAST], &bf, 0))
    });
}

fn avg_f_1(c: &mut Criterion) {
    let ind = AVG::new();
    c.bench_function("avg_f_1", |b| b.iter(|| ind.ind_f(&IN_)));
}

fn avg_coll_1(c: &mut Criterion) {
    let ind = AVG::new();
    c.bench_function("avg_coll_1", |b| b.iter(|| ind.ind_coll::<Vec<f64>>(&IN_)));
}

criterion_group!(benches, avg_bf_1, avg_f_1, avg_coll_1);
criterion_main!(benches);
