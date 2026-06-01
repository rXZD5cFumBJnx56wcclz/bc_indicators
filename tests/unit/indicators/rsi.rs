use bc_utils_lg::statics::settings::WINDOW;

use bc_indicators::indicators::rsi::*;
use bc_indicators::indicators::test_funcs::*;

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
