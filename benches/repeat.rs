mod prelude;
use prelude::*;

use bc_indicators::repeat::Repeat;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i], CLOSE[i]])
        .collect::<Vec<Vec<f64>>>()
});

fn repeat_bf_1(c: &mut Criterion) {
    let ind = Repeat::new(1.0);
    ind.init_bf(&IN_);
    c.bench_function("repeat_bf_1", |b| {
        b.iter(|| ind.ind(&[OPEN_LAST, CLOSE_LAST]))
    });
}

criterion_group!(benches, repeat_bf_1,);
criterion_main!(benches);
