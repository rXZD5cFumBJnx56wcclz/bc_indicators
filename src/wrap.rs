use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Clone, Default, Copy)]
pub struct Wrap;

impl W for Wrap {
    fn w(&self) -> usize {
        0
    }
}

impl Indicator for Wrap {
    fn ind(&self, math_operations: &[f64]) -> f64 {
        math_operations[0]
    }
    fn init_bf(&self, _in_: &[Vec<f64>]) {}
    fn execute_bf(&self) {}

    fn ind_vec(&self, in_: &[Vec<f64>]) -> Vec<f64> {
        in_.iter().map(|x| self.ind(x)).collect()
    }
}

impl IndicatorExt for Wrap {
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

    static RES: f64 = OPEN[OPEN.len() - 1];
    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        (0..OPEN.len())
            .map(|i| vec![OPEN[i]])
            .collect::<Vec<Vec<f64>>>()
    });

    #[test]
    fn wrap_bf_res_1() {
        test_ind_bf_res_1(Wrap, &IN_, RES);
    }
}
