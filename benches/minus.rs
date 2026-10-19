mod prelude;
use prelude::*;

use bc_indicators::minus::Minus;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i], CLOSE[i]])
        .collect::<Vec<Vec<f64>>>()
});

fn minus_bf_1(c: &mut Criterion) {
    let ind = Minus;
    ind.init_bf(&IN_);
    c.bench_function("minus_bf_1", |b| {
        b.iter(|| ind.ind(&[OPEN_LAST, CLOSE_LAST]))
    });
}

criterion_group!(benches, minus_bf_1,);
criterion_main!(benches);
