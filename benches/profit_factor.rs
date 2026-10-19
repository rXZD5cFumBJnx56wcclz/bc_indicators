mod prelude;
use prelude::*;

use bc_indicators::profit_factor::ProfitFactor;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![1.0, 2.0, -1.0]; 3]);

fn profit_factor_bf_1(c: &mut Criterion) {
    let ind = ProfitFactor;
    ind.init_bf(&IN_);
    c.bench_function("profit_factor_bf_1", |b| b.iter(|| ind.ind(&[OPEN_LAST])));
}

criterion_group!(benches, profit_factor_bf_1,);
criterion_main!(benches);
