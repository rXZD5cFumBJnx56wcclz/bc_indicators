use std::sync::LazyLock;

use bc_utils_lg::statics::prices::{CLOSE, OPEN};

use crate::unit::indicators::test_funcs::*;
use bc_indicators::indicators::repeat::*;

static RES: f64 = 1.0;
static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i], CLOSE[i]])
        .collect::<Vec<Vec<f64>>>()
});
static SETTINGS_: LazyLock<REPEAT> = LazyLock::new(|| REPEAT::new(1.0));

#[test]
fn repeat_bf_res_1() {
    test_bf_res_1((*SETTINGS_).clone(), &IN_, RES);
}

#[test]
fn repeat_f_res_1() {
    test_f_res_1((*SETTINGS_).clone(), &IN_, RES);
}

#[test]
fn repeat_coll_res_1() {
    test_coll_res_1((*SETTINGS_).clone(), &IN_, RES, 21);
}

#[test]
fn repeat_coll_res_2() {
    test_coll_res_2((*SETTINGS_).clone(), &IN_, 30);
}
