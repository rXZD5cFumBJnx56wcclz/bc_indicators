mod prelude;
use prelude::*;

use bc_indicators::wrap::Wrap;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i]])
        .collect::<Vec<Vec<f64>>>()
});

fn wrap_bf_1(c: &mut Criterion) {
    let ind = Wrap;
    ind.init_bf(&IN_);
    c.bench_function("wrap_bf_1", |b| {
        b.iter(|| ind.ind(&[OPEN_LAST, CLOSE_LAST]))
    });
}

criterion_group!(benches, wrap_bf_1,);
criterion_main!(benches);
