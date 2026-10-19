mod prelude;
use prelude::*;

use bc_indicators::rsi::Rsi;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    OPEN.iter()
        .copied()
        .map(|v| vec![v])
        .collect::<Vec<Vec<f64>>>()
});

fn rsi_bf_1(c: &mut Criterion) {
    let ind = Rsi::new(4);
    ind.init_bf(&IN_);
    c.bench_function("rsi_bf_1", |b| b.iter(|| ind.ind(&[OPEN_LAST])));
}

criterion_group!(benches, rsi_bf_1,);
criterion_main!(benches);
