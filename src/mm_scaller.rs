#![allow(non_camel_case_types)]
use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct MmScallerParams {
    pub window: usize,
}

impl Default for MmScallerParams {
    fn default() -> Self {
        Self { window: 100 }
    }
}

impl MmScallerParams {
    pub fn new(window: usize) -> Self {
        Self { window }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct MmScallerBf {
    src_l: Vec<f64>,
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct MmScaller {
    pub params: MmScallerParams,
    bf: RefCell<MmScallerBf>,
    bf_state: RefCell<MmScallerBf>,
}

impl MmScaller {
    pub fn new(window: usize) -> Self {
        Self {
            params: MmScallerParams::new(window),
            ..Default::default()
        }
    }
}

fn mm_scaller(src: f64, min_: f64, max_: f64) -> f64 {
    (src - min_) / (max_ - min_)
}

impl W for MmScaller {
    fn w(&self) -> usize {
        self.params.window
    }
}

impl Indicator for MmScaller {
    fn init_bf(&self, in_: &[Vec<f64>]) {
        self.bf.borrow_mut().src_l = in_[in_.len() - self.w()..].iter().map(|v| v[0]).collect();
        *self.bf_state.borrow_mut() = self.bf.borrow().clone();
    }
    fn ind(&self, in_: &[f64]) -> f64 {
        self.bf_state.borrow_mut().src_l = {
            let mut bind = self.bf.borrow().src_l[1..].to_vec();
            bind.push(in_[0]);
            bind
        };
        let min_ = *self
            .bf_state
            .borrow()
            .src_l
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max_ = *self
            .bf_state
            .borrow()
            .src_l
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        mm_scaller(in_[in_.len() - 1], min_, max_)
    }
}

impl ExecuteBf for MmScaller {
    fn execute_bf(&self) {
        *self.bf.borrow_mut() = self.bf_state.borrow().clone();
    }
}

impl IndicatorExt for MmScaller {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;

    use crate::prelude_tests::prelude::*;

    static RES: f64 = 0.25;
    static IN_: LazyLock<Vec<Vec<f64>>> =
        LazyLock::new(|| vec![vec![30.0], vec![0.0], vec![100.0], vec![60.0], vec![70.]]);

    #[test]
    fn mm_scaller_bf_res_1() {
        let settings = MmScaller::new(3);
        test_ind_bf_res_1(settings, &IN_, RES);
    }

    #[test]
    fn mm_scaller_coll_res_1() {
        let settings = MmScaller::new(3);
        test_coll_res_1(settings, &IN_, 1);
    }
}
