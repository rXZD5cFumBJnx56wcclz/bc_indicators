use crate::indicators::ready_imports::*;

#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub struct EMA {
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl EMA {
    pub fn new(window: usize) -> Self {
        Self {
            window,
            mult_window_accuracy: 10,
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

impl Default for EMA {
    fn default() -> Self {
        EMA::new(14)
    }
}

impl Indicator for EMA {
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
        math_operations[0] * math_operations[2] + math_operations[1] * (1.0 - math_operations[2])
    }
    fn bf(&self, in_: &[Vec<f64>]) -> std::cell::RefCell<Vec<FxHashMap<&'static str, Vec<f64>>>> {
        let mut res = 0.0;
        let len = in_.len();
        let window_t = self.window as f64;
        let alpha = 2.0 / (window_t + 1.0);

        for (i, el) in in_[len - self.window * self.mult_window_accuracy..]
            .iter()
            .map(|v| v[0])
            .enumerate()
        {
            if i < self.window {
                res += el;
                continue;
            }
            if i == self.window - 1 {
                res /= window_t;
            }
            res = self.ind(&[el, res, alpha]);
        }
        RefCell::new(vec![FxHashMap::from_iter([
            ("alpha", vec![alpha]),
            ("res", vec![res]),
        ])])
    }
    fn ind_with_bf<'a>(
        &self,
        in_: &[f64],
        bf: &RefCell<Vec<FxHashMap<&'static str, Vec<f64>>>>,
        index_: usize,
    ) -> f64 {
        let res = self.ind(&[
            in_[0],
            bf.borrow()[index_]["res"][0],
            bf.borrow()[index_]["alpha"][0],
        ]);
        bf.borrow_mut()[index_].insert("res", vec![res]);
        res
    }
}

impl IndicatorExt for EMA {}
