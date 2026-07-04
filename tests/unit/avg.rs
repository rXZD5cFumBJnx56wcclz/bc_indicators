use std::sync::LazyLock;

use bc_utils_lg::statics::prices::{CLOSE, HIGH, LOW, OPEN};

use crate::unit::test_funcs::*;
use bc_indicators::avg::*;

static RES: f64 =
    (OPEN[OPEN.len() - 1] + CLOSE[OPEN.len() - 1] + HIGH[OPEN.len() - 1] + LOW[OPEN.len() - 1])
        / 4.0;
static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    (0..OPEN.len())
        .map(|i| vec![OPEN[i], CLOSE[i], HIGH[i], LOW[i]])
        .collect::<Vec<Vec<f64>>>()
});

#[test]
fn avg_bf_res_1() {
    let settings = AVG::new();
    test_bf_res_1(settings, &IN_, RES);
}

#[test]
fn avg_f_res_1() {
    let settings = AVG::new();
    test_f_res_1(settings, &IN_, RES);
}

#[test]
fn avg_coll_res_1() {
    let settings = AVG::new();
    test_coll_res_1(settings, &IN_, RES, 21);
}

#[test]
fn avg_coll_res_2() {
    let settings = AVG::new();
    test_coll_res_2(settings, &IN_, 30);
}
