use bc_indicators::indicators::ready_imports::Indicator;
use bc_utils_lg::statics::prices::OPEN;
use bc_utils_lg::statics::settings::WINDOW;
use bc_indicators::indicators::rma::*;


#[test]
fn rma_bf_res_1() {
    let settings = RMA::new(WINDOW, 10);
    let bf = settings.bf(
        OPEN
            .iter()
            .take(OPEN.len() - 1)
            .map(|v| vec![*v])
            .collect::<Vec<Vec<f64>>>().as_slice()
    );
    assert_eq!(
        settings.ind_with_bf(&[2.2547], &bf),
        2.2548879972457887,
    );
}

#[test]
fn rma_f_res_1() {
    let settings = RMA::new(WINDOW, 10);
    assert_eq!(
        settings.ind_f(OPEN.into_iter().map(|v| vec![v]).collect::<Vec<Vec<f64>>>().as_slice()),
        2.2548879972457887,
    );
}

#[test]
fn rma_coll_res_1() {
    let settings = RMA::new(WINDOW, 10);
    assert_eq!(
        dbg!(settings.ind_coll::<Vec<_>>(
            OPEN[OPEN.len() - 21..].into_iter().map(|v| vec![*v]).collect::<Vec<Vec<f64>>>().as_slice()
        ))[20],
        2.2548879972457887,
    );
}
