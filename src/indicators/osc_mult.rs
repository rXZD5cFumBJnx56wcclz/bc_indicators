#![allow(non_camel_case_types)]
use crate::indicators::ready_imports::*;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct OSC_MULT {
    pub th_short: f64,
    pub th_long: f64,
    pub max_value: f64,
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl OSC_MULT {
    pub fn new(th_short: f64, th_long: f64, max_value: f64) -> Self {
        Self {
            th_short: th_short,
            th_long: th_long,
            max_value: max_value,
            window: 0,
            mult_window_accuracy: 0,
            add_window_accuracy: 0,
        }
    }
    pub fn set_th_short(&mut self, th_short: f64) {
        self.th_short = th_short;
    }
    pub fn set_th_long(&mut self, th_long: f64) {
        self.th_long = th_long;
    }
    pub fn set_max_value(&mut self, max_value: f64) {
        self.max_value = max_value;
    }
}

impl Default for OSC_MULT {
    fn default() -> Self {
        OSC_MULT::new(0.15, 0.15, 1.0)
    }
}

impl Indicator for OSC_MULT {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn ind(&self, math_operations: &[f64]) -> f64 {
        let diff: f64;
        let v2: f64;
        let v_b = math_operations[0];

        if v_b >= (self.max_value - self.th_short) {
            diff = self.th_short;
            v2 = self.max_value - v_b;
        } else if v_b <= self.th_long {
            diff = self.th_long;
            v2 = v_b;
        } else {
            diff = v_b;
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
