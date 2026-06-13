#![allow(non_camel_case_types)]
use crate::indicators::ready_imports::*;

#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub struct TREND_MA {
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl TREND_MA {
    pub fn new() -> Self {
        Self {
            window: 0,
            mult_window_accuracy: 1,
            add_window_accuracy: 2 + 1,
        }
    }
    pub fn set_window(&mut self, window: usize) {
        self.window = window;
    }
    pub fn set_mult_window_accuracy(&mut self, mult_window_accuracy: usize) {
        self.mult_window_accuracy = mult_window_accuracy;
    }
    pub fn set_add_window_accuracy(&mut self, add_window_accuracy: usize) {
        self.add_window_accuracy = add_window_accuracy;
    }
}

impl Default for TREND_MA {
    fn default() -> Self {
        TREND_MA::new()
    }
}

impl Indicator for TREND_MA {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn ind(&self, _: &[f64]) -> f64 {
        Default::default()
    }
    fn bf(&self, in_: &[Vec<f64>]) -> std::cell::RefCell<Vec<FxHashMap<&'static str, Vec<f64>>>> {
        let mut bf = FxHashMap::default();
        let src_l = in_[in_.len() - 2][0];
        let src = in_[in_.len() - 1][0];
        if src > src_l {
            bf.insert("trend", vec![1.0]);
        } else if src < src_l {
            bf.insert("trend", vec![-1.0]);
        } else {
            bf.insert("trend", vec![0.0]);
        }
        bf.insert("src_l", vec![src]);
        RefCell::new(vec![bf])
    }
    fn ind_with_bf<'a>(
        &self,
        in_: &[f64],
        bf: &RefCell<Vec<FxHashMap<&'static str, Vec<f64>>>>,
        index_: usize,
    ) -> f64 {
        let src_l = bf.borrow()[index_]["src_l"][0];
        let src = in_[0];
        if src > src_l {
            bf.borrow_mut()[index_].insert("trend", vec![1.0]);
        } else if src < src_l {
            bf.borrow_mut()[index_].insert("trend", vec![-1.0]);
        }
        bf.borrow_mut()[index_].insert("src_l", vec![src]);
        bf.borrow()[index_]["trend"][0]
    }
}

impl IndicatorExt for TREND_MA {}
