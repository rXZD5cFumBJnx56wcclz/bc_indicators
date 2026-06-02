use bc_utils_lg::statics::settings::WINDOW;

use crate::unit::indicators::test_funcs::*;
use bc_indicators::indicators::rma::*;

#[test]
fn rma_bf_res_1() {
    let settings = RMA::new(WINDOW);
    test_bf_res_1(settings, 2.2548879972457887);
}

#[test]
fn rma_f_res_1() {
    let settings = RMA::new(WINDOW);
    test_f_res_1(settings, 2.2548879972457887);
}

#[test]
fn rma_coll_res_1() {
    let settings = RMA::new(WINDOW);
    test_coll_res_1(settings, 2.2548879972457887, 21);
}

#[test]
fn rma_coll_res_2() {
    let settings = RMA::new(WINDOW);
    test_coll_res_2(settings, 30);
}
