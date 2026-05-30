use bc_utils_lg::statics::settings::WINDOW;

use bc_indicators::indicators::rma::*;
use bc_indicators::indicators::test_funcs::*;


#[test]
fn rma_bf_res_1() {
   
	let settings = RMA::new(WINDOW,);
    test_bf_res_1(settings, 2.2548879972457887,);
}

#[test]
fn rma_f_res_1() {
   
	let settings = RMA::new(WINDOW,);
    test_f_res_1(settings, 2.2548879972457887,);
}

#[test]
fn rma_coll_res_1() {
   
	let settings = RMA::new(WINDOW,);
    test_coll_res_1(settings, 2.2548879972457887, 21);
}
