#![allow(non_camel_case_types)]
use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct TrendMaParams {
    pub add_window_accuracy: usize,
    pub trend_short: f64,
    pub trend_long: f64,
    pub trend_hold: f64,
}

impl Default for TrendMaParams {
    fn default() -> Self {
        Self {
            add_window_accuracy: 10,
            trend_short: -1.,
            trend_long: 1.,
            trend_hold: 0.,
        }
    }
}

impl TrendMaParams {
    pub fn new(
        add_window_accuracy: usize,
        trend_short: f64,
        trend_long: f64,
        trend_hold: f64,
    ) -> Self {
        Self {
            add_window_accuracy,
            trend_short,
            trend_long,
            trend_hold,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
pub struct TrendMaBf {
    trend: f64,
    src_l: f64,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
pub struct TrendMa {
    pub params: TrendMaParams,
    bf: RefCell<TrendMaBf>,
    bf_state: RefCell<TrendMaBf>,
}

impl TrendMa {
    pub fn new(
        add_window_accuracy: usize,
        trend_short: f64,
        trend_long: f64,
        trend_hold: f64,
    ) -> Self {
        Self {
            params: TrendMaParams::new(add_window_accuracy, trend_short, trend_long, trend_hold),
            ..Default::default()
        }
    }
}

impl W for TrendMa {
    fn w(&self) -> usize {
        // window 1
        self.params.add_window_accuracy + 1 + 1
    }
}

impl Indicator for TrendMa {
    fn init_bf(&self, in_: &[Vec<f64>]) {
        for src in &in_[in_.len() - (self.w() - 1)..] {
            let src = src[0];
            if src > self.bf.borrow().src_l {
                self.bf.borrow_mut().trend = self.params.trend_long;
            } else if src < self.bf.borrow().src_l {
                self.bf.borrow_mut().trend = self.params.trend_short;
            } else {
                self.bf.borrow_mut().trend = self.params.trend_hold;
            }
            self.bf.borrow_mut().src_l = src;
        }
        *self.bf_state.borrow_mut() = self.bf.borrow().clone();
    }
    fn execute_bf(&self) {
        *self.bf.borrow_mut() = self.bf_state.borrow().clone();
    }
    fn ind(&self, in_: &[f64]) -> f64 {
        let src_l = self.bf.borrow().src_l;
        let src = in_[0];
        if src > src_l {
            self.bf_state.borrow_mut().trend = self.params.trend_long;
        } else if src < src_l {
            self.bf_state.borrow_mut().trend = self.params.trend_short;
        }
        self.bf_state.borrow_mut().src_l = src;
        self.bf_state.borrow_mut().trend
    }
}

impl IndicatorExt for TrendMa {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;

    use crate::prelude_tests::prelude::*;

    const RES: f64 = 1.0;
    static IN_: LazyLock<Vec<Vec<f64>>> =
        LazyLock::new(|| (1..14).map(|v| vec![v as f64]).collect());

    #[test]
    fn trend_ma_bf_res_1() {
        let settings = TrendMa::default();
        test_ind_bf_res_1(settings, &IN_, RES);
    }

    #[test]
    fn trend_ma_coll_res_1() {
        let settings = TrendMa::default();
        test_coll_res_1(settings, &IN_, 1);
    }
}
