#![allow(non_camel_case_types)]
use bc_utils::nums::dz;

use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Default, Clone, Copy)]
pub struct ProfitFactor;

impl W for ProfitFactor {
    fn w(&self) -> usize {
        0
    }
}

impl Indicator for ProfitFactor {
    fn ind(&self, math_operations: &[f64]) -> f64 {
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
    fn init_bf(&self, _: &[Vec<f64>]) {}

    fn ind_vec(&self, in_: &[Vec<f64>]) -> Vec<f64> {
        in_.iter().map(|x| self.ind(x)).collect()
    }
}

impl ExecuteBf for ProfitFactor {
    fn execute_bf(&self) {}
}

impl IndicatorExt for ProfitFactor {
    fn ind_coll<C>(&self, in_: &[Vec<f64>]) -> C
    where
        C: FromIterator<f64>,
    {
        in_.iter().map(|x| self.ind(x)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;

    use crate::prelude_tests::prelude::*;

    static RES: f64 = 3.0;
    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![1.0, 2.0, -1.0]; 3]);

    #[test]
    fn profit_factor_bf_res_1() {
        let settings = ProfitFactor;
        test_ind_bf_res_1(settings, &IN_, RES);
    }
}
