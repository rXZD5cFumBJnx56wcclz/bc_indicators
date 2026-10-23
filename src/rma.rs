use crate::prelude::*;

// для этого индикатора требудется запас данных больше в 10 раз, чем его окна
// иначе значение будет не корректным

#[derive(Default, Debug, PartialEq, PartialOrd, Clone)]
pub struct RmaBf {
    pub alpha: f64,
    pub res: f64,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct RmaParams {
    pub window: usize,
    pub mult_window_accuracy: usize,
}

impl Default for RmaParams {
    fn default() -> Self {
        Self {
            window: 14,
            mult_window_accuracy: 10,
        }
    }
}

impl RmaParams {
    pub fn new(window: usize) -> Self {
        Self {
            window,
            ..Default::default()
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct Rma {
    pub params: RmaParams,
    bf: RefCell<RmaBf>,
    bf_state: RefCell<RmaBf>,
}

impl Rma {
    pub fn new(window: usize) -> Self {
        Self {
            params: RmaParams::new(window),
            ..Default::default()
        }
    }
}

fn rma(src: f64, res: f64, alpha: f64) -> f64 {
    alpha * src + (1.0 - alpha) * res
}

impl W for Rma {
    fn w(&self) -> usize {
        self.params.window * self.params.mult_window_accuracy
    }
}

impl Indicator for Rma {
    fn init_bf(&self, in_: &[Vec<f64>]) {
        let mut res = 0.0;
        let window_t = self.params.window as f64;
        let alpha = 1.0 / window_t;

        for (i, el) in in_[in_.len() - self.w()..].iter().map(|v| v[0]).enumerate() {
            if i < self.params.window {
                res += el;
                continue;
            }
            if i == self.params.window - 1 {
                res /= window_t;
            }
            res = rma(el, res, alpha);
        }
        self.bf.borrow_mut().alpha = alpha;
        self.bf.borrow_mut().res = res;
        *self.bf_state.borrow_mut() = self.bf.borrow().clone();
    }

    fn ind(&self, in_: &[f64]) -> f64 {
        self.bf_state.borrow_mut().res = rma(in_[0], self.bf.borrow().res, self.bf.borrow().alpha);
        self.bf_state.borrow_mut().res
    }
}

impl ExecuteBf for Rma {
    fn execute_bf(&self) {
        *self.bf.borrow_mut() = self.bf_state.borrow().clone();
    }
}

impl IndicatorExt for Rma {}

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
    const RES: f64 = 2.2548879972457887;

    #[test]
    fn rma_bf_res_1() {
        let settings = Rma::new(2);
        test_ind_bf_res_1(settings, &IN_, RES);
    }

    #[test]
    fn rma_coll_res_1() {
        let settings = Rma::new(2);
        test_coll_res_1(settings, &IN_, 10);
    }
}
