use bc_utils::{nums::avg, other::roll_slice1};

use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub struct SMA {
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl SMA {
    pub fn new(window: usize) -> Self {
        Self {
            window,
            mult_window_accuracy: 1,
            add_window_accuracy: 1,
        }
    }
    pub fn set_window(
        &mut self,
        window: usize,
    ) {
        self.window = window;
    }
}

impl Default for SMA {
    fn default() -> Self {
        SMA::new(14)
    }
}

impl Indicator for SMA {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn ind(
        &self,
        math_operations: &[f64],
    ) -> f64 {
        avg(math_operations)
    }
    fn bf<'a>(
        &self,
        in_: &[Vec<f64>],
    ) -> BF_INDICATOR<'a> {
        RefCell::new(vec![MAP::from_iter([(
            "src_l_vec",
            in_[in_.len() - self.window..]
                .into_iter()
                .map(|v| v[0])
                .collect::<Vec<f64>>(),
        )])])
    }
    fn ind_with_bf<'a>(
        &self,
        in_: &[f64],
        bf: &RefCell<Vec<MAP<&'a str, Vec<f64>>>>,
        index_: usize,
    ) -> f64 {
        roll_slice1(
            bf.borrow_mut()
                .get_mut(index_)
                .unwrap()
                .get_mut("src_l_vec")
                .unwrap(),
            -1,
        );
        bf.borrow_mut()
            .get_mut(index_)
            .unwrap()
            .get_mut("src_l_vec")
            .unwrap()[self.window - 1] = in_[0];
        avg(&bf.borrow()[index_]["src_l_vec"][..])
    }
}

impl IndicatorExt for SMA {}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use bc_utils_lg::statics::prices::OPEN;

    use crate::sma::*;
    use crate::test_funcs::test_funcs::*;

    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        OPEN.iter()
            .copied()
            .map(|v| vec![v])
            .collect::<Vec<Vec<f64>>>()
    });
    static RES: LazyLock<f64> =
        LazyLock::new(|| OPEN[OPEN.len() - 10..].into_iter().sum::<f64>() / 10.0);

    #[test]
    fn sma_bf_res_1() {
        let settings = SMA::new(10);
        test_bf_res_1(settings, &IN_, *RES);
    }

    #[test]
    fn sma_f_res_1() {
        let settings = SMA::new(10);
        test_f_res_1(settings, &IN_, *RES);
    }

    #[test]
    fn sma_coll_res_1() {
        let settings = SMA::new(10);
        test_coll_res_1(settings, &IN_, *RES, 21);
    }

    #[test]
    fn sma_coll_res_2() {
        let settings = SMA::new(10);
        test_coll_res_2(settings, &IN_, 30);
    }
}
