use std::sync::LazyLock;

use crate::unit::indicators::test_funcs::*;
use bc_indicators::indicators::profit_factor::*;

static RES: f64 = 3.0;
static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![1.0, 2.0, -1.0]; 3]);

#[test]
fn profit_factor_bf_res_1() {
    let settings = PROFIT_FACTOR::default();
    test_bf_res_1(settings, &IN_, RES);
}

#[test]
fn profit_factor_f_res_1() {
    let settings = PROFIT_FACTOR::default();
    test_f_res_1(settings, &IN_, RES);
}

#[test]
fn profit_factor_coll_res_1() {
    let settings = PROFIT_FACTOR::default();
    test_coll_res_1(settings, &IN_, RES, 3);
}

#[test]
fn profit_factor_coll_res_2() {
    let settings = PROFIT_FACTOR::default();
    test_coll_res_2(settings, &IN_, 3);
}
