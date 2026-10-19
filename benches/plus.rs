mod prelude;
use prelude::*;

use bc_indicators::plus::Plus;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i], CLOSE[i]])
        .collect::<Vec<Vec<f64>>>()
});

fn plus_bf_1(c: &mut Criterion) {
    let ind = Plus;
    ind.init_bf(&IN_);
    c.bench_function("plus_bf_1", |b| {
        b.iter(|| ind.ind(&[OPEN_LAST, CLOSE_LAST]))
    });
}

criterion_group!(benches, plus_bf_1,);
criterion_main!(benches);
