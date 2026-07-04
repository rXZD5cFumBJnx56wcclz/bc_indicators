use std::sync::LazyLock;

use crate::unit::test_funcs::*;
use bc_indicators::mm_scaler::*;

static RES: f64 = 0.6;
static IN_: LazyLock<Vec<Vec<f64>>> =
    LazyLock::new(|| vec![vec![30.0], vec![0.0], vec![100.0], vec![60.0]]);

#[test]
fn mm_scaler_bf_res_1() {
    let settings = MM_SCALER::new(3);
    test_bf_res_1(settings, &IN_, RES);
}

#[test]
fn mm_scaler_f_res_1() {
    let settings = MM_SCALER::new(3);
    test_f_res_1(settings, &IN_, RES);
}

#[test]
fn mm_scaler_coll_res_1() {
    let settings = MM_SCALER::new(3);
    test_coll_res_1(settings, &IN_, RES, 4);
}

#[test]
fn mm_scaler_coll_res_2() {
    let settings = MM_SCALER::new(3);
    test_coll_res_2(settings, &IN_, 4);
}
