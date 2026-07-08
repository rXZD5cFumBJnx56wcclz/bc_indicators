#![allow(non_camel_case_types)]
use crate::prelude::*;

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
            add_window_accuracy: 10,
        }
    }
    pub fn set_window(
        &mut self,
        window: usize,
    ) {
        self.window = window;
    }
    pub fn set_mult_window_accuracy(
        &mut self,
        mult_window_accuracy: usize,
    ) {
        self.mult_window_accuracy = mult_window_accuracy;
    }
    pub fn set_add_window_accuracy(
        &mut self,
        add_window_accuracy: usize,
    ) {
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
    fn ind(
        &self,
        _: &[f64],
    ) -> f64 {
        Default::default()
    }
    fn bf<'a>(
        &self,
        in_: &[Vec<f64>],
    ) -> BF_INDICATOR<'a> {
        let mut bf = MAP::default();
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
        bf: &RefCell<Vec<MAP<&'a str, Vec<f64>>>>,
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

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::test_funcs::test_funcs::*;
    use crate::trend_ma::*;

    const RES: f64 = 1.0;
    static IN_: LazyLock<Vec<Vec<f64>>> =
        LazyLock::new(|| (1..11).map(|v| vec![v as f64]).collect());

    #[test]
    fn trend_ma_bf_res_1() {
        let settings = TREND_MA::new();
        test_bf_res_1(settings, &IN_, RES);
    }

    #[test]
    fn trend_ma_f_res_1() {
        let settings = TREND_MA::new();
        test_f_res_1(settings, &IN_, RES);
    }

    #[test]
    fn trend_ma_coll_res_1() {
        let settings = TREND_MA::new();
        test_coll_res_1(settings, &IN_, RES, 10);
    }

    #[test]
    fn trend_ma_coll_res_2() {
        let settings = TREND_MA::new();
        test_coll_res_2(settings, &IN_, 10);
    }
}
