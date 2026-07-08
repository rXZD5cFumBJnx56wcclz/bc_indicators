#![allow(non_camel_case_types)]

use std::cell::RefCell;

use bc_utils_lg::types::maps::MAP;

use std::any::Any;

fn ind_coll<C, T>(
    indicator: &T,
    in_: &[Vec<f64>],
) -> C
where
    C: FromIterator<f64>,
    T: Indicator,
    T: ?Sized,
{
    let bf;
    if indicator.w() != 0 {
        bf = indicator.bf(&in_[..indicator.w() - 1]);
    } else {
        bf = Default::default();
    }
    in_.iter()
        .enumerate()
        .map(|v| {
            if v.0 < indicator.w().checked_sub(1).unwrap_or(indicator.w()) {
                f64::NAN
            } else {
                indicator.ind_with_bf(v.1, &bf, 0)
            }
        })
        .collect()
}

pub trait Indicator: Any {
    fn w(&self) -> usize;
    fn ind(
        &self,
        math_operations: &[f64],
    ) -> f64;
    fn bf<'a>(
        &self,
        in_: &[Vec<f64>],
    ) -> RefCell<Vec<MAP<&'a str, Vec<f64>>>>;
    fn ind_with_bf<'a>(
        &self,
        in_: &[f64],
        bf: &RefCell<Vec<MAP<&'a str, Vec<f64>>>>,
        index_: usize,
    ) -> f64;
    fn ind_f(
        &self,
        in_: &[Vec<f64>],
    ) -> f64 {
        let bf;
        if self.w() != 0 {
            bf = self.bf(&in_[in_.len() - self.w()..in_.len() - 1]);
        } else {
            bf = Default::default();
        }
        self.ind_with_bf(&in_[in_.len() - 1], &bf, 0)
    }
    fn ind_vec(
        &self,
        in_: &[Vec<f64>],
    ) -> Vec<f64> {
        ind_coll(self, in_)
    }
}

pub trait IndicatorExt: Indicator {
    fn ind_coll<C>(
        &self,
        in_: &[Vec<f64>],
    ) -> C
    where
        C: FromIterator<f64>,
    {
        ind_coll(self, in_)
    }
}

pub type BF_INDICATOR<'a> = RefCell<Vec<MAP<&'a str, Vec<f64>>>>;
