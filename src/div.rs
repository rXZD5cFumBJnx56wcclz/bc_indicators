use bc_utils::nums::dz;

use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub struct DIV {
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl DIV {
    pub fn new() -> Self {
        Self {
            window: 0,
            mult_window_accuracy: 0,
            add_window_accuracy: 0,
        }
    }
}

impl Default for DIV {
    fn default() -> Self {
        DIV::new()
    }
}

impl Indicator for DIV {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn ind(
        &self,
        math_operations: &[f64],
    ) -> f64 {
        math_operations[0] / dz(math_operations[1])
    }
    fn bf<'a>(
        &self,
        _: &[Vec<f64>],
    ) -> BF_INDICATOR<'a> {
        Default::default()
    }
    fn ind_with_bf<'a>(
        &self,
        in_: &[f64],
        _: &RefCell<Vec<MAP<&'a str, Vec<f64>>>>,
        _: usize,
    ) -> f64 {
        self.ind(in_)
    }
    fn ind_f(
        &self,
        in_: &[Vec<f64>],
    ) -> f64 {
        self.ind(in_.last().expect("no elements in slice"))
    }
    fn ind_vec(
        &self,
        in_: &[Vec<f64>],
    ) -> Vec<f64> {
        in_.iter().map(|x| self.ind(x)).collect()
    }
}

impl IndicatorExt for DIV {
    fn ind_coll<C>(
        &self,
        in_: &[Vec<f64>],
    ) -> C
    where
        C: FromIterator<f64>,
    {
        in_.iter().map(|x| self.ind(x)).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use bc_utils_lg::statics::prices::{CLOSE, OPEN};

    use crate::div::*;
    use crate::test_funcs::test_funcs::*;

    static RES: f64 = OPEN[OPEN.len() - 1] / CLOSE[OPEN.len() - 1];
    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        (0..OPEN.len())
            .map(|i| vec![OPEN[i], CLOSE[i]])
            .collect::<Vec<Vec<f64>>>()
    });

    #[test]
    fn div_bf_res_1() {
        let settings = DIV::new();
        test_bf_res_1(settings, &IN_, RES);
    }

    #[test]
    fn div_f_res_1() {
        let settings = DIV::new();
        test_f_res_1(settings, &IN_, RES);
    }

    #[test]
    fn div_coll_res_1() {
        let settings = DIV::new();
        test_coll_res_1(settings, &IN_, RES, 21);
    }

    #[test]
    fn div_coll_res_2() {
        let settings = DIV::new();
        test_coll_res_2(settings, &IN_, 30);
    }
}
