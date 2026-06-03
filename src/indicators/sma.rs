use bc_utils::{nums::avg, other::roll_slice1};

use crate::indicators::ready_imports::*;

#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub struct SMA {
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl SMA {
    pub fn new(window: usize) -> Self {
        Self {
            window,
            mult_window_accuracy: 1,
            add_window_accuracy: 0,
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

impl Default for SMA {
    fn default() -> Self {
        SMA::new(14)
    }
}

impl Indicator for SMA {
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
        avg(math_operations)
    }
    fn bf(&self, in_: &[Vec<f64>]) -> std::cell::RefCell<Vec<FxHashMap<&'static str, Vec<f64>>>> {
        RefCell::new(vec![FxHashMap::from_iter([(
            "src_l_vec",
            in_[in_.len() - self.window..]
                .into_iter()
                .map(|v| v[0])
                .collect::<Vec<f64>>(),
        )])])
    }
    fn ind_with_bf<'a>(
        &self,
        in_: &[f64],
        bf: &RefCell<Vec<FxHashMap<&'static str, Vec<f64>>>>,
        index_: usize,
    ) -> f64 {
        dbg!(bf, bf.borrow()[0]["src_l_vec"].len());
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
        dbg!(bf, bf.borrow()[0]["src_l_vec"].len());
        avg(&bf.borrow()[index_]["src_l_vec"][..])
    }
}

impl IndicatorExt for SMA {}
