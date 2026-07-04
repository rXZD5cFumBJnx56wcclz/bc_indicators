use bc_utils::{nums::avg, other::roll_slice1};

use crate::ready_imports::*;

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
            add_window_accuracy: 1,
        }
    }
    pub fn set_window(&mut self, window: usize) {
        self.window = window;
    }
}

impl Default for SMA {
    fn default() -> Self {
        SMA::new(14)
    }
}

impl Indicator for SMA {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn ind(&self, math_operations: &[f64]) -> f64 {
        avg(math_operations)
    }
    fn bf<'a>(&self, in_: &[Vec<f64>]) -> BF_INDICATOR<'a> {
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
        bf: &RefCell<Vec<FxHashMap<&'a str, Vec<f64>>>>,
        index_: usize,
    ) -> f64 {
        roll_slice1(
            bf.borrow_mut()
                .get_mut(index_)
                .unwrap()
                .get_mut("src_l_vec")
                .unwrap(),
            -1,
        );
        bf.borrow_mut()
            .get_mut(index_)
            .unwrap()
            .get_mut("src_l_vec")
            .unwrap()[self.window - 1] = in_[0];
        avg(&bf.borrow()[index_]["src_l_vec"][..])
    }
}

impl IndicatorExt for SMA {}
