use bc_utils::nums::dz;

use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Clone, Default, Copy)]
pub struct Div;

impl W for Div {
    fn w(&self) -> usize {
        0
    }
}

impl Indicator for Div {
    fn ind(&self, in_: &[f64]) -> f64 {
        in_[0] / dz(in_[1])
    }
    fn init_bf(&self, _in_: &[Vec<f64>]) {}
    fn execute_bf(&self) {}
    fn ind_vec(&self, in_: &[Vec<f64>]) -> Vec<f64> {
        in_.iter().map(|x| self.ind(x)).collect()
    }
}

impl IndicatorExt for Div {
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
    use crate::prelude_tests::prelude::*;
    use std::sync::LazyLock;

    static RES: f64 = OPEN[OPEN.len() - 1] / CLOSE[OPEN.len() - 1];
    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        (0..OPEN.len())
            .map(|i| vec![OPEN[i], CLOSE[i]])
            .collect::<Vec<Vec<f64>>>()
    });

    #[test]
    fn div_bf_res_1() {
        test_ind_bf_res_1(Div, &IN_, RES);
    }

    #[test]
    fn div_coll_res_1() {
        test_coll_res_1(Div, &IN_, 10);
    }
}
