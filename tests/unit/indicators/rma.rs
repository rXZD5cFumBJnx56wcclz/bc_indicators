use std::sync::LazyLock;

use bc_utils_lg::statics::prices::OPEN;
use bc_utils_lg::statics::settings::WINDOW;

use crate::unit::indicators::test_funcs::*;
use bc_indicators::indicators::rma::*;

static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    OPEN.iter()
        .copied()
        .map(|v| vec![v])
        .collect::<Vec<Vec<f64>>>()
});
const RES: f64 = 2.2548879972457887;

#[test]
fn rma_bf_res_1() {
    let settings = RMA::new(WINDOW);
    test_bf_res_1(settings, &IN_, RES);
}

#[test]
fn rma_f_res_1() {
    let settings = RMA::new(WINDOW);
    test_f_res_1(settings, &IN_, RES);
}

#[test]
fn rma_coll_res_1() {
    let settings = RMA::new(WINDOW);
    test_coll_res_1(settings, &IN_, RES, 21);
}

#[test]
fn rma_coll_res_2() {
    let settings = RMA::new(WINDOW);
    test_coll_res_2(settings, &IN_, 30);
}
