mod prelude;
use prelude::*;

use bc_indicators::osc_mult::OscMult;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![85.0]; 5]);

fn osc_mult_bf_1(c: &mut Criterion) {
    let ind = OscMult::new(15.0, 15.0, 100.0);
    ind.init_bf(&IN_);
    c.bench_function("osc_mult_bf_1", |b| b.iter(|| ind.ind(&[OPEN_LAST])));
}

criterion_group!(benches, osc_mult_bf_1,);
criterion_main!(benches);
