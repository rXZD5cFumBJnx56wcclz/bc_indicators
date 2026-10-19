mod prelude;
use prelude::*;

use bc_indicators::avg::Avg;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i], CLOSE[i], HIGH[i], LOW[i]])
        .collect::<Vec<Vec<f64>>>()
});

fn avg_bf_1(c: &mut Criterion) {
    let ind = Avg;
    ind.init_bf(&IN_);
    c.bench_function("avg_bf_1", |b| b.iter(|| ind.ind(&[OPEN_LAST])));
}

criterion_group!(benches, avg_bf_1,);
criterion_main!(benches);
