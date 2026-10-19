mod prelude;
use prelude::*;

use bc_indicators::ema::Ema;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    OPEN.iter()
        .copied()
        .map(|v| vec![v])
        .collect::<Vec<Vec<f64>>>()
});

fn ema_bf_1(c: &mut Criterion) {
    let ind = Ema::new(4);
    ind.init_bf(&IN_);
    c.bench_function("ema_bf_1", |b| b.iter(|| ind.ind(&[OPEN_LAST])));
}

criterion_group!(benches, ema_bf_1,);
criterion_main!(benches);
