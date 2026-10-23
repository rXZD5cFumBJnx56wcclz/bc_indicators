mod prelude;
use bc_indicators::percent::Percent;
use prelude::*;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i], CLOSE[i]])
        .collect::<Vec<Vec<f64>>>()
});

fn percent_bf_1(c: &mut Criterion) {
    let ind = Percent::default();
    ind.init_bf(&IN_);
    c.bench_function("percent_bf_1", |b| {
        b.iter(|| ind.ind(&[OPEN_LAST, CLOSE_LAST]))
    });
}

criterion_group!(benches, percent_bf_1,);
criterion_main!(benches);
