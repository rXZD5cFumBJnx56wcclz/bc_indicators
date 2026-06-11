#![allow(non_camel_case_types)]
use crate::indicators::ready_imports::*;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct OSC_MULT {
    pub diff_short: f64,
    pub diff_long: f64,
    pub max_v: f64,
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl OSC_MULT {
    pub fn new(diff_short: f64, diff_long: f64, max_v: f64) -> Self {
        Self {
            diff_short: diff_short,
            diff_long: diff_long,
            max_v: max_v,
            window: 0,
            mult_window_accuracy: 1,
            add_window_accuracy: 0,
        }
    }
    pub fn set_diff_short(&mut self, diff_short: f64) {
        self.diff_short = diff_short;
    }
    pub fn set_diff_long(&mut self, diff_long: f64) {
        self.diff_long = diff_long;
    }
    pub fn set_max_v(&mut self, max_v: f64) {
        self.max_v = max_v;
    }
}

impl Default for OSC_MULT {
    fn default() -> Self {
        OSC_MULT::new(15.0, 15.0, 100.0)
    }
}

impl Indicator for OSC_MULT {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy + 1
    }
    fn ind(&self, math_operations: &[f64]) -> f64 {
        let diff: f64;
        let v2: f64;
        let v_b = math_operations[0];

        if v_b > (self.max_v - self.diff_short) {
            diff = self.diff_short;
            v2 = self.max_v - v_b;
        } else {
            diff = self.diff_long;
            v2 = v_b;
        }
        (diff - v2) / diff
    }
    fn bf(&self, _: &[Vec<f64>]) -> std::cell::RefCell<Vec<FxHashMap<&'static str, Vec<f64>>>> {
        Default::default()
    }
    fn ind_with_bf<'a>(
        &self,
        in_: &[f64],
        _: &RefCell<Vec<FxHashMap<&'static str, Vec<f64>>>>,
        _: usize,
    ) -> f64 {
        self.ind(in_)
    }
    fn ind_f(&self, in_: &[Vec<f64>]) -> f64 {
        self.ind(in_.last().expect("no elements in slice"))
    }
    fn ind_vec(&self, in_: &[Vec<f64>]) -> Vec<f64> {
        in_.iter().map(|x| self.ind(x)).collect()
    }
}

impl IndicatorExt for OSC_MULT {
    fn ind_coll<C>(&self, in_: &[Vec<f64>]) -> C
    where
        C: FromIterator<f64>,
    {
        in_.iter().map(|x| self.ind(x)).collect()
    }
}
