#![allow(non_camel_case_types)]
use bc_utils::nums::dz;

use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub struct PROFIT_FACTOR {
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl PROFIT_FACTOR {
    pub fn new() -> Self {
        Self {
            window: 0,
            mult_window_accuracy: 0,
            add_window_accuracy: 0,
        }
    }
}

impl Default for PROFIT_FACTOR {
    fn default() -> Self {
        PROFIT_FACTOR::new()
    }
}

impl Indicator for PROFIT_FACTOR {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn ind(
        &self,
        math_operations: &[f64],
    ) -> f64 {
        let mut negative = 0.;
        let mut positive = 0.;
        let zero_ = &0.;

        for el in math_operations {
            if el < zero_ {
                negative += *el
            } else if el > zero_ {
                positive += *el
            }
        }
        negative = negative.abs();
        if negative == *zero_ {
            positive / dz(negative)
        } else {
            positive / negative
        }
    }
    fn bf<'a>(
        &self,
        _: &[Vec<f64>],
    ) -> BF_INDICATOR<'a> {
        Default::default()
    }
    fn ind_with_bf<'a>(
        &self,
        in_: &[f64],
        _: &RefCell<Vec<MAP<&'a str, Vec<f64>>>>,
        _: usize,
    ) -> f64 {
        self.ind(in_)
    }
    fn ind_f(
        &self,
        in_: &[Vec<f64>],
    ) -> f64 {
        self.ind(in_.last().expect("no elements in slice"))
    }
    fn ind_vec(
        &self,
        in_: &[Vec<f64>],
    ) -> Vec<f64> {
        in_.iter().map(|x| self.ind(x)).collect()
    }
}

impl IndicatorExt for PROFIT_FACTOR {
    fn ind_coll<C>(
        &self,
        in_: &[Vec<f64>],
    ) -> C
    where
        C: FromIterator<f64>,
    {
        in_.iter().map(|x| self.ind(x)).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::profit_factor::*;
    use crate::test_funcs::test_funcs::*;

    static RES: f64 = 3.0;
    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![1.0, 2.0, -1.0]; 3]);

    #[test]
    fn profit_factor_bf_res_1() {
        let settings = PROFIT_FACTOR::default();
        test_bf_res_1(settings, &IN_, RES);
    }

    #[test]
    fn profit_factor_f_res_1() {
        let settings = PROFIT_FACTOR::default();
        test_f_res_1(settings, &IN_, RES);
    }

    #[test]
    fn profit_factor_coll_res_1() {
        let settings = PROFIT_FACTOR::default();
        test_coll_res_1(settings, &IN_, RES, 3);
    }

    #[test]
    fn profit_factor_coll_res_2() {
        let settings = PROFIT_FACTOR::default();
        test_coll_res_2(settings, &IN_, 3);
    }
}
