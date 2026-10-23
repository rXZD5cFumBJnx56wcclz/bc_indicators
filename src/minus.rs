use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
pub struct PlusParams {
    pub fixed_values: Vec<f64>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
pub struct Minus {
    pub params: PlusParams,
}

impl W for Minus {
    fn w(&self) -> usize {
        0
    }
}

impl Indicator for Minus {
    fn ind(&self, math_operations: &[f64]) -> f64 {
        let mut res = self
            .params
            .fixed_values
            .first()
            .copied()
            .unwrap_or(math_operations.first().copied().unwrap_or_default());
        for fixed in self.params.fixed_values.iter().skip(1) {
            res -= *fixed;
        }
        for opp in math_operations.iter().skip(1) {
            res -= *opp;
        }
        res
    }
    fn init_bf(&self, _in_: &[Vec<f64>]) {}

    fn ind_vec(&self, in_: &[Vec<f64>]) -> Vec<f64> {
        in_.iter().map(|x| self.ind(x)).collect()
    }
}

impl ExecuteBf for Minus {
    fn execute_bf(&self) {}
}

impl IndicatorExt for Minus {
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

    static RES: f64 = OPEN[OPEN.len() - 1] - CLOSE[OPEN.len() - 1];
    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        (0..OPEN.len())
            .map(|i| vec![OPEN[i], CLOSE[i]])
            .collect::<Vec<Vec<f64>>>()
    });

    #[test]
    fn plus_bf_res_1() {
        test_ind_bf_res_1(Minus::default(), &IN_, RES);
    }

    #[test]
    fn plus_coll_res_1() {
        test_coll_res_1(Minus::default(), &IN_, 10);
    }
}
