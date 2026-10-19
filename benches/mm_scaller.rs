mod prelude;
use prelude::*;

use bc_indicators::mm_scaller::MmScaller;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    OPEN.iter()
        .copied()
        .map(|v| vec![v])
        .collect::<Vec<Vec<f64>>>()
});

fn mm_scaller_bf_1(c: &mut Criterion) {
    let ind = MmScaller::new(20);
    ind.init_bf(&IN_);
    c.bench_function("mm_scaller_bf_1", |b| b.iter(|| ind.ind(&[OPEN_LAST])));
}

criterion_group!(benches, mm_scaller_bf_1,);
criterion_main!(benches);
