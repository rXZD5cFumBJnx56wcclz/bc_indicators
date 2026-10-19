mod prelude;
use prelude::*;

use bc_indicators::trend_ma::TrendMa;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    OPEN.iter()
        .copied()
        .map(|v| vec![v])
        .collect::<Vec<Vec<f64>>>()
});

fn trend_ma_bf_1(c: &mut Criterion) {
    let ind = TrendMa::default();
    ind.init_bf(&IN_);
    c.bench_function("trend_ma_bf_1", |b| b.iter(|| ind.ind(&[OPEN_LAST])));
}

criterion_group!(benches, trend_ma_bf_1,);
criterion_main!(benches);
