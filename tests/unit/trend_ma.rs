use std::sync::LazyLock;

use crate::unit::test_funcs::*;
use bc_indicators::trend_ma::*;

const RES: f64 = 1.0;
static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| (1..11).map(|v| vec![v as f64]).collect());

#[test]
fn trend_ma_bf_res_1() {
    let settings = TREND_MA::new();
    test_bf_res_1(settings, &IN_, RES);
}

#[test]
fn trend_ma_f_res_1() {
    let settings = TREND_MA::new();
    test_f_res_1(settings, &IN_, RES);
}

#[test]
fn trend_ma_coll_res_1() {
    let settings = TREND_MA::new();
    test_coll_res_1(settings, &IN_, RES, 10);
}

#[test]
fn trend_ma_coll_res_2() {
    let settings = TREND_MA::new();
    test_coll_res_2(settings, &IN_, 10);
}
