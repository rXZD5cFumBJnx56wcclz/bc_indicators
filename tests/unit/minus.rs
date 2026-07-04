use std::sync::LazyLock;

use bc_utils_lg::statics::prices::{CLOSE, OPEN};

use crate::unit::test_funcs::*;
use bc_indicators::minus::*;

static RES: f64 = OPEN[OPEN.len() - 1] - CLOSE[OPEN.len() - 1];
static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i], CLOSE[i]])
        .collect::<Vec<Vec<f64>>>()
});

#[test]
fn minus_bf_res_1() {
    let settings = MINUS::new();
    test_bf_res_1(settings, &IN_, RES);
}

#[test]
fn minus_f_res_1() {
    let settings = MINUS::new();
    test_f_res_1(settings, &IN_, RES);
}

#[test]
fn minus_coll_res_1() {
    let settings = MINUS::new();
    test_coll_res_1(settings, &IN_, RES, 21);
}

#[test]
fn minus_coll_res_2() {
    let settings = MINUS::new();
    test_coll_res_2(settings, &IN_, 30);
}
