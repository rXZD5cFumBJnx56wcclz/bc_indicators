use crate::prelude::*;
use crate::rma::RMA;

#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub struct RSI {
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl RSI {
    pub fn new(window: usize) -> Self {
        Self {
            window,
            mult_window_accuracy: 10,
            add_window_accuracy: 2,
        }
    }
    pub fn set_window(
        &mut self,
        window: usize,
    ) {
        self.window = window;
    }
    pub fn set_mult_window_accuracy(
        &mut self,
        mult_window_accuracy: usize,
    ) {
        self.mult_window_accuracy = mult_window_accuracy;
    }
    pub fn set_add_window_accuracy(
        &mut self,
        add_window_accuracy: usize,
    ) {
        self.add_window_accuracy = add_window_accuracy;
    }
}

impl Default for RSI {
    fn default() -> Self {
        RSI::new(14)
    }
}

impl Indicator for RSI {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn ind(
        &self,
        math_operations: &[f64],
    ) -> f64 {
        (100.0 - (100.0 / (1.0 + math_operations[0] / math_operations[1]))) / 100.0
    }
    fn bf<'a>(
        &self,
        in_: &[Vec<f64>],
    ) -> RefCell<Vec<MAP<&'a str, Vec<f64>>>> {
        let mut u = Vec::new();
        let mut d = Vec::new();
        let mut src_l = f64::NAN;
        let len_src = in_.len();
        let _w = self.w() - 1;

        for (i, el) in in_[len_src - _w..].iter().map(|v| v[0]).enumerate() {
            if i == 0 {
                src_l = el;
                continue;
            }
            let change = el - src_l;
            u.push(change.max(0.0));
            d.push((-change).max(0.0));
            src_l = el;
        }
        let settings_rma = RMA::new(self.window);
        RefCell::new(vec![
            RMA::bf(
                &settings_rma,
                &u.into_iter().map(|v| vec![v]).collect::<Vec<Vec<f64>>>(),
            )
            .take()
            .pop()
            .expect("there is no data inside the RMA buffer"),
            RMA::bf(
                &settings_rma,
                &d.into_iter().map(|v| vec![v]).collect::<Vec<Vec<f64>>>(),
            )
            .take()
            .pop()
            .expect("there is no data inside the RMA buffer"),
            MAP::from_iter([("src_l", vec![src_l])]),
        ])
    }
    fn ind_with_bf<'a>(
        &self,
        in_: &[f64],
        bf: &RefCell<Vec<MAP<&'a str, Vec<f64>>>>,
        index_: usize,
    ) -> f64 {
        let settings_rma = RMA::new(self.window);
        let change = in_[0] - bf.borrow()[2]["src_l"][0];
        let u = 0.0f64.max(change);
        let d = 0.0f64.max(-change);
        bf.borrow_mut()[2].insert("src_l", in_.to_vec());
        self.ind(&[
            RMA::ind_with_bf(&settings_rma, &[u], bf, index_),
            RMA::ind_with_bf(&settings_rma, &[d], bf, index_ + 1),
        ])
    }
}

impl IndicatorExt for RSI {}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use bc_utils_lg::statics::prices::OPEN;

    use crate::rsi::*;
    use crate::test_funcs::test_funcs::*;

    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        OPEN.iter()
            .copied()
            .map(|v| vec![v])
            .collect::<Vec<Vec<f64>>>()
    });
    const RES: f64 = 40.410730678054115 / 100.0;

    #[test]
    fn rsi_bf_res_1() {
        let settings = RSI::new(2);
        test_bf_res_1(settings, &IN_, RES);
    }

    #[test]
    fn rsi_f_res_1() {
        let settings = RSI::new(2);
        test_f_res_1(settings, &IN_, RES);
    }

    #[test]
    fn rsi_coll_res_1() {
        let settings = RSI::new(2);
        test_coll_res_1(settings, &IN_, RES, 22);
    }

    #[test]
    fn rsi_coll_res_2() {
        let settings = RSI::new(2);
        test_coll_res_2(settings, &IN_, 30);
    }
}
