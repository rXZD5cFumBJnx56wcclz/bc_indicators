#![allow(non_camel_case_types)]

use crate::prelude::*;

fn ind_coll<C, T>(indicator: &T, in_: &[Vec<f64>]) -> C
where
    C: FromIterator<f64>,
    T: Indicator,
    T: ?Sized,
{
    in_.iter()
        .enumerate()
        .map(|v| {
            let res = indicator.ind(v.1);
            indicator.execute_bf();
            res
        })
        .collect()
}

pub trait Indicator: Any + W + ExecuteBf + DynClone {
    fn init_bf(&self, in_: &[Vec<f64>]);
    fn ind(&self, in_: &[f64]) -> f64;
    fn ind_vec(&self, in_: &[Vec<f64>]) -> Vec<f64> {
        ind_coll(self, in_)
    }
}

clone_trait_object!(Indicator);

pub trait IndicatorExt: Indicator {
    fn ind_coll<C>(&self, in_: &[Vec<f64>]) -> C
    where
        C: FromIterator<f64>,
    {
        ind_coll(self, in_)
    }
}
