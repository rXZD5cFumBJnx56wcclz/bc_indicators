#![allow(non_camel_case_types)]
use bc_utils::other::roll_slice1;

use crate::indicators::ready_imports::*;

#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub struct MM_SCALLER {
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl MM_SCALLER {
    pub fn new(window: usize) -> Self {
        Self {
            window: window,
            mult_window_accuracy: 1,
            add_window_accuracy: 0,
        }
    }
}

impl Default for MM_SCALLER {
    fn default() -> Self {
        MM_SCALLER::new(100)
    }
}

impl Indicator for MM_SCALLER {
    fn get_window(&self) -> usize {
        self.window
    }
    fn get_mult_window_accuracy(&self) -> usize {
        self.mult_window_accuracy
    }
    fn get_add_window_accuracy(&self) -> usize {
        self.add_window_accuracy
    }
    fn ind(&self, math_operations: &[f64]) -> f64 {
        (math_operations[0] - math_operations[1]) / (math_operations[2] - math_operations[1])
    }
    fn bf(&self, in_: &[Vec<f64>]) -> std::cell::RefCell<Vec<FxHashMap<&'static str, Vec<f64>>>> {
        RefCell::new(vec![FxHashMap::from_iter([(
            "src_l_vec",
            in_[in_.len() - self.window..]
                .iter()
                .map(|v| v[0])
                .collect(),
        )])])
    }
    fn ind_with_bf<'a>(
        &self,
        in_: &[f64],
        bf: &RefCell<Vec<FxHashMap<&'static str, Vec<f64>>>>,
        index_: usize,
    ) -> f64 {
        roll_slice1(
            bf.borrow_mut()
                .get_mut(index_)
                .unwrap()
                .get_mut("src_l_vec")
                .unwrap(),
            &-1,
        );
        bf.borrow_mut()
            .get_mut(index_)
            .unwrap()
            .get_mut("src_l_vec")
            .unwrap()[self.window - 1] = in_[0];
        let min_ = *bf.borrow()[index_]["src_l_vec"]
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max_ = *bf.borrow()[index_]["src_l_vec"]
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        self.ind(&vec![in_[in_.len() - 1], min_, max_])
    }
}

impl IndicatorExt for MM_SCALLER {}
