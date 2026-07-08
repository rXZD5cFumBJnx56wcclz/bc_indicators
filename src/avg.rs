use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub struct AVG {
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl AVG {
    pub fn new() -> Self {
        Self {
            window: 0,
            mult_window_accuracy: 0,
            add_window_accuracy: 0,
        }
    }
}

impl Default for AVG {
    fn default() -> Self {
        AVG::new()
    }
}

impl Indicator for AVG {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn ind(
        &self,
        math_operations: &[f64],
    ) -> f64 {
        math_operations.into_iter().sum::<f64>() / math_operations.len() as f64
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

impl IndicatorExt for AVG {
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

    use bc_utils_lg::statics::prices::{CLOSE, HIGH, LOW, OPEN};

    use crate::avg::*;
    use crate::test_funcs::test_funcs::*;

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
        let settings = AVG::new();
        test_bf_res_1(settings, &IN_, RES);
    }

    #[test]
    fn avg_f_res_1() {
        let settings = AVG::new();
        test_f_res_1(settings, &IN_, RES);
    }

    #[test]
    fn avg_coll_res_1() {
        let settings = AVG::new();
        test_coll_res_1(settings, &IN_, RES, 21);
    }

    #[test]
    fn avg_coll_res_2() {
        let settings = AVG::new();
        test_coll_res_2(settings, &IN_, 30);
    }
}
