use std::sync::LazyLock;

use bc_utils_lg::statics::prices::OPEN;

use crate::unit::indicators::test_funcs::*;
use bc_indicators::indicators::sma::*;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    OPEN.iter()
        .copied()
        .map(|v| vec![v])
        .collect::<Vec<Vec<f64>>>()
});
static RES: LazyLock<f64> =
    LazyLock::new(|| OPEN[OPEN.len() - 10..].into_iter().sum::<f64>() / 10.0);

#[test]
fn sma_bf_res_1() {
    let settings = SMA::new(10);
    test_bf_res_1(settings, &IN_, *RES);
}

#[test]
fn sma_f_res_1() {
    let settings = SMA::new(10);
    test_f_res_1(settings, &IN_, *RES);
}

#[test]
fn sma_coll_res_1() {
    let settings = SMA::new(10);
    test_coll_res_1(settings, &IN_, *RES, 21);
}

#[test]
fn sma_coll_res_2() {
    let settings = SMA::new(10);
    test_coll_res_2(settings, &IN_, 30);
}
