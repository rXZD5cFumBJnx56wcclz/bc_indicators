use bc_utils_lg::traits::w::W;

use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Clone, Default, Copy)]
pub struct Avg;

impl W for Avg {
    fn w(&self) -> usize {
        0
    }
}
impl Indicator for Avg {
    fn init_bf(&self, _in_: &[Vec<f64>]) {}
    fn ind(&self, in_: &[f64]) -> f64 {
        in_.iter().sum::<f64>() / in_.len() as f64
    }
    fn ind_vec(&self, in_: &[Vec<f64>]) -> Vec<f64> {
        in_.iter().map(|x| self.ind(x)).collect()
    }
}

impl ExecuteBf for Avg {
    fn execute_bf(&self) {}
}

impl IndicatorExt for Avg {
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

    static RES: f64 =
        (OPEN[OPEN.len() - 1] + CLOSE[OPEN.len() - 1] + HIGH[OPEN.len() - 1] + LOW[OPEN.len() - 1])
            / 4.0;
    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        (0..OPEN.len())
            .map(|i| vec![OPEN[i], CLOSE[i], HIGH[i], LOW[i]])
            .collect::<Vec<Vec<f64>>>()
    });

    #[test]
    fn avg_bf_res_1() {
        test_ind_bf_res_1(Avg, &IN_, RES);
    }

    #[test]
    fn avg_coll_res_1() {
        test_coll_res_1(Avg, &IN_, 10);
    }
}
