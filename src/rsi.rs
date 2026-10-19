use bc_utils_lg::traits::w::W;

use crate::prelude::*;
use crate::rma::Rma;

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct RsiBf {
    pub src_l: f64,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct RsiParams {
    pub window: usize,
    pub mult_window_accuracy: usize,
}

impl Default for RsiParams {
    fn default() -> Self {
        Self {
            window: 14,
            mult_window_accuracy: 10,
        }
    }
}

impl RsiParams {
    pub fn new(window: usize) -> Self {
        Self {
            window,
            ..Default::default()
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct Rsi {
    pub params: RsiParams,
    bf: RefCell<RsiBf>,
    bf_state: RefCell<RsiBf>,
    pub rma1: Rma,
    pub rma2: Rma,
}

impl Rsi {
    pub fn new(window: usize) -> Self {
        Self {
            params: RsiParams::new(window),
            rma1: Rma::new(window),
            rma2: Rma::new(window),
            ..Default::default()
        }
    }
}

fn rsi(rma1: f64, rma2: f64) -> f64 {
    (100.0 - (100.0 / (1.0 + rma1 / rma2))) / 100.0
}

impl W for Rsi {
    fn w(&self) -> usize {
        // 1 for rma
        self.params.window * self.params.mult_window_accuracy + 1
    }
}

impl Indicator for Rsi {
    fn init_bf(&self, in_: &[Vec<f64>]) {
        let mut u = Vec::new();
        let mut d = Vec::new();
        let mut src_l = f64::NAN;
        let len_src = in_.len();

        for (i, el) in in_[len_src - self.w()..].iter().map(|v| v[0]).enumerate() {
            if i == 0 {
                src_l = el;
                continue;
            }
            let change = el - src_l;
            u.push(change.max(0.0));
            d.push((-change).max(0.0));
            src_l = el;
        }
        self.bf.borrow_mut().src_l = src_l;
        *self.bf_state.borrow_mut() = self.bf.borrow().clone();
        self.rma1
            .init_bf(&u.into_iter().map(|v| vec![v]).collect::<Vec<Vec<f64>>>());
        self.rma2
            .init_bf(&d.into_iter().map(|v| vec![v]).collect::<Vec<Vec<f64>>>());
    }
    fn execute_bf(&self) {
        *self.bf.borrow_mut() = self.bf_state.borrow().clone();
        self.rma1.execute_bf();
        self.rma2.execute_bf();
    }

    fn ind(&self, in_: &[f64]) -> f64 {
        let change = in_[0] - self.bf.borrow().src_l;
        let u = 0.0f64.max(change);
        let d = 0.0f64.max(-change);
        self.bf_state.borrow_mut().src_l = in_[0];
        rsi(self.rma1.ind(&[u]), self.rma2.ind(&[d]))
    }
}

impl IndicatorExt for Rsi {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;

    use crate::prelude_tests::prelude::*;

    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        OPEN.iter()
            .copied()
            .map(|v| vec![v])
            .collect::<Vec<Vec<f64>>>()
    });
    const RES: f64 = 40.410730678054115 / 100.0;

    #[test]
    fn rsi_bf_res_1() {
        let settings = Rsi::new(2);
        test_ind_bf_res_1(settings, &IN_, RES);
    }

    #[test]
    fn rsi_coll_res_1() {
        let settings = Rsi::new(2);
        test_coll_res_1(settings, &IN_, 10);
    }
}
