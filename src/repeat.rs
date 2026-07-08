use crate::prelude::*;

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
            mult_window_accuracy: 0,
            add_window_accuracy: 0,
        }
    }
    pub fn set_value(
        &mut self,
        value: f64,
    ) {
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
    fn ind(
        &self,
        _: &[f64],
    ) -> f64 {
        self.value
    }
    fn bf<'a>(
        &self,
        _: &[Vec<f64>],
    ) -> BF_INDICATOR<'a> {
        Default::default()
    }
    fn ind_with_bf<'a>(
        &self,
        _: &[f64],
        _: &RefCell<Vec<MAP<&'a str, Vec<f64>>>>,
        _: usize,
    ) -> f64 {
        self.value
    }
    fn ind_f(
        &self,
        _: &[Vec<f64>],
    ) -> f64 {
        self.value
    }
    fn ind_vec(
        &self,
        in_: &[Vec<f64>],
    ) -> Vec<f64> {
        (0..in_.len()).map(|_| self.value).collect()
    }
}

impl IndicatorExt for REPEAT {
    fn ind_coll<C>(
        &self,
        in_: &[Vec<f64>],
    ) -> C
    where
        C: FromIterator<f64>,
    {
        (0..in_.len()).map(|_| self.value).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use bc_utils_lg::statics::prices::{CLOSE, OPEN};

    use crate::repeat::*;
    use crate::test_funcs::test_funcs::*;

    static RES: f64 = 1.0;
    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        (0..OPEN.len())
            .map(|i| vec![OPEN[i], CLOSE[i]])
            .collect::<Vec<Vec<f64>>>()
    });
    static SETTINGS_: LazyLock<REPEAT> = LazyLock::new(|| REPEAT::new(1.0));

    #[test]
    fn repeat_bf_res_1() {
        test_bf_res_1((*SETTINGS_).clone(), &IN_, RES);
    }

    #[test]
    fn repeat_f_res_1() {
        test_f_res_1((*SETTINGS_).clone(), &IN_, RES);
    }

    #[test]
    fn repeat_coll_res_1() {
        test_coll_res_1((*SETTINGS_).clone(), &IN_, RES, 21);
    }

    #[test]
    fn repeat_coll_res_2() {
        test_coll_res_2((*SETTINGS_).clone(), &IN_, 30);
    }
}
