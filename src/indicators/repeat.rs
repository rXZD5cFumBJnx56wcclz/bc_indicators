use crate::indicators::ready_imports::*;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct REPEAT {
    pub value: f64,
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl REPEAT {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            window: 0,
            mult_window_accuracy: 1,
            add_window_accuracy: 0,
        }
    }
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }
}

impl Default for REPEAT {
    fn default() -> Self {
        REPEAT::new(0.0)
    }
}

impl Indicator for REPEAT {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn ind(&self, _: &[f64]) -> f64 {
        self.value
    }
    fn bf(&self, _: &[Vec<f64>]) -> std::cell::RefCell<Vec<FxHashMap<&'static str, Vec<f64>>>> {
        Default::default()
    }
    fn ind_with_bf<'a>(
        &self,
        _: &[f64],
        _: &RefCell<Vec<FxHashMap<&'static str, Vec<f64>>>>,
        _: usize,
    ) -> f64 {
        self.value
    }
    fn ind_f(&self, _: &[Vec<f64>]) -> f64 {
        self.value
    }
    fn ind_vec(&self, in_: &[Vec<f64>]) -> Vec<f64> {
        (0..in_.len()).map(|_| self.value).collect()
    }
}

impl IndicatorExt for REPEAT {
    fn ind_coll<C>(&self, in_: &[Vec<f64>]) -> C
    where
        C: FromIterator<f64>,
    {
        (0..in_.len()).map(|_| self.value).collect()
    }
}
