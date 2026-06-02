use bc_utils_lg::statics::settings::WINDOW;

use crate::unit::indicators::test_funcs::*;
use bc_indicators::indicators::rsi::*;

#[test]
fn rsi_bf_res_1() {
    let settings = RSI::new(WINDOW);
    test_bf_res_1(settings, 40.410730678054115 / 100.0);
}

#[test]
fn rsi_f_res_1() {
    let settings = RSI::new(WINDOW);
    test_f_res_1(settings, 40.410730678054115 / 100.0);
}

#[test]
fn rsi_coll_res_1() {
    let settings = RSI::new(WINDOW);
    test_coll_res_1(settings, 40.410730678054115 / 100.0, 22);
}

#[test]
fn rsi_coll_res_2() {
    let settings = RSI::new(WINDOW);
    test_coll_res_2(settings, 30);
}
