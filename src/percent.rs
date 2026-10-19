use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Clone, Default, Copy)]
pub struct Percent;

impl W for Percent {
    fn w(&self) -> usize {
        0
    }
}

// how much 0 differs from 1 relative to 0
impl Indicator for Percent {
    fn ind(&self, math_operations: &[f64]) -> f64 {
        (math_operations[0] - math_operations[1]) / math_operations[0]
    }
    fn init_bf(&self, _in_: &[Vec<f64>]) {}
    fn execute_bf(&self) {}

    fn ind_vec(&self, in_: &[Vec<f64>]) -> Vec<f64> {
        in_.iter().map(|x| self.ind(x)).collect()
    }
}

impl IndicatorExt for Percent {
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

    static RES: f64 = (OPEN[OPEN.len() - 1] - CLOSE[OPEN.len() - 1]) / OPEN[OPEN.len() - 1];
    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        (0..OPEN.len())
            .map(|i| vec![OPEN[i], CLOSE[i]])
            .collect::<Vec<Vec<f64>>>()
    });

    #[test]
    fn percent_bf_res_1() {
        test_ind_bf_res_1(Percent, &IN_, RES);
    }

    #[test]
    fn percent_coll_res_1() {
        test_coll_res_1(Percent, &IN_, 10);
    }
}
