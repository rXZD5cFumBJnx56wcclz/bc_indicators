use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
pub struct Abs;

impl W for Abs {
    fn w(&self) -> usize {
        0
    }
}

impl Indicator for Abs {
    fn ind(&self, in_: &[f64]) -> f64 {
        in_[0].abs()
    }

    fn init_bf(&self, _in_: &[Vec<f64>]) {}
    fn ind_vec(&self, in_: &[Vec<f64>]) -> Vec<f64> {
        in_.iter().map(|x| self.ind(x)).collect()
    }
}

impl ExecuteBf for Abs {
    fn execute_bf(&self) {}
}

impl IndicatorExt for Abs {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude_tests::prelude::*;
    use std::sync::LazyLock;

    static RES: f64 = OPEN[OPEN.len() - 1].abs();
    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        (0..OPEN.len())
            .map(|i| vec![OPEN[i]])
            .collect::<Vec<Vec<f64>>>()
    });

    #[test]
    fn abs_bf_res_1() {
        test_ind_bf_res_1(Abs, &IN_, RES);
    }

    #[test]
    fn abs_coll_res_1() {
        test_coll_res_1(Abs, &IN_, 10);
    }

    #[test]
    fn abs_val_res_1() {
        assert_eq!(Abs.ind(&[-42.5]), 42.5);
        assert_eq!(Abs.ind(&[42.5]), 42.5);
        assert_eq!(Abs.ind(&[0.0]), 0.0);
    }
}
