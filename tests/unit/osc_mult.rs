use std::sync::LazyLock;

use crate::unit::test_funcs::*;
use bc_indicators::osc_mult::*;

static RES: f64 = 0.5;
static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![85.0]; 5]);

#[test]
fn osc_mult_bf_res_1() {
    let settings = OSC_MULT::new(30.0, 70.0, 100.0);
    test_bf_res_1(settings, &IN_, RES);
}

#[test]
fn osc_mult_f_res_1() {
    let settings = OSC_MULT::new(30.0, 70.0, 100.0);
    test_f_res_1(settings, &IN_, RES);
}

#[test]
fn osc_mult_coll_res_1() {
    let settings = OSC_MULT::new(30.0, 70.0, 100.0);
    test_coll_res_1(settings, &IN_, RES, 2);
}

#[test]
fn osc_mult_coll_res_2() {
    let settings = OSC_MULT::new(30.0, 70.0, 100.0);
    test_coll_res_2(settings, &IN_, 2);
}
