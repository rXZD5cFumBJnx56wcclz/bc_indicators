use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct EmaParams {
    pub window: usize,
    pub mult_window_accuracy: usize,
}

impl Default for EmaParams {
    fn default() -> Self {
        Self {
            window: 14,
            mult_window_accuracy: 10,
        }
    }
}

impl EmaParams {
    pub fn new(window: usize) -> Self {
        Self {
            window,
            ..Default::default()
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct EmaBf {
    alpha: f64,
    res: f64,
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct Ema {
    pub params: EmaParams,
    bf: RefCell<EmaBf>,
    bf_state: RefCell<EmaBf>,
}

impl Ema {
    pub fn new(window: usize) -> Self {
        Self {
            params: EmaParams::new(window),
            ..Default::default()
        }
    }
}

fn ema(src: f64, res: f64, alpha: f64) -> f64 {
    src * alpha + res * (1.0 - alpha)
}

impl W for Ema {
    fn w(&self) -> usize {
        self.params.window * self.params.mult_window_accuracy
    }
}

impl Indicator for Ema {
    fn init_bf(&self, in_: &[Vec<f64>]) {
        let mut res = 0.0;
        let len = in_.len();
        let window_t = self.params.window as f64;
        let alpha = 2.0 / (window_t + 1.0);

        for (i, el) in in_[len - self.params.window * self.params.mult_window_accuracy..]
            .iter()
            .map(|v| v[0])
            .enumerate()
        {
            if i < self.params.window {
                res += el;
                continue;
            }
            if i == self.params.window - 1 {
                res /= window_t;
            }
            res = ema(el, res, alpha);
        }
        self.bf.borrow_mut().alpha = alpha;
        self.bf.borrow_mut().res = res;
        *self.bf_state.borrow_mut() = self.bf.borrow().clone();
    }

    fn ind(&self, in_: &[f64]) -> f64 {
        self.bf_state.borrow_mut().res = ema(in_[0], self.bf.borrow().res, self.bf.borrow().alpha);
        self.bf_state.borrow().res
    }
}

impl ExecuteBf for Ema {
    fn execute_bf(&self) {
        *self.bf.borrow_mut() = self.bf_state.borrow().clone();
    }
}

impl IndicatorExt for Ema {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;

    use crate::prelude_tests::prelude::*;

    static RES: f64 = 2.254711084891796;
    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        OPEN.iter()
            .copied()
            .map(|v| vec![v])
            .collect::<Vec<Vec<f64>>>()
    });

    #[test]
    fn ema_bf_res_1() {
        let settings = Ema::new(2);
        test_ind_bf_res_1(settings, &IN_, RES);
    }

    #[test]
    fn ema_coll_res_1() {
        let settings = Ema::new(2);
        test_coll_res_1(settings, &IN_, 10);
    }
}
