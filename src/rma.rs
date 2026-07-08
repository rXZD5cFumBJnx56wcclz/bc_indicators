use crate::prelude::*;

// для этого индикатора требудется запас данных больше в 10 раз, чем его окна
// иначе значение будет не корректным
#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub struct RMA {
    pub window: usize,
    pub mult_window_accuracy: usize,
    pub add_window_accuracy: usize,
}

impl RMA {
    pub fn new(window: usize) -> Self {
        Self {
            window,
            mult_window_accuracy: 10,
            add_window_accuracy: 1,
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

impl Default for RMA {
    fn default() -> Self {
        RMA::new(14)
    }
}

impl Indicator for RMA {
    fn w(&self) -> usize {
        self.window * self.mult_window_accuracy + self.add_window_accuracy
    }
    fn ind(
        &self,
        math_operations: &[f64],
    ) -> f64 {
        math_operations[2] * math_operations[0] + (1.0 - math_operations[2]) * math_operations[1]
    }
    fn bf<'a>(
        &self,
        in_: &[Vec<f64>],
    ) -> BF_INDICATOR<'a> {
        let mut res = 0.0;
        let len = in_.len();
        let window_t = self.window as f64;
        let alpha = 1.0 / window_t;

        for (i, el) in in_[len - self.window * self.mult_window_accuracy..]
            .iter()
            .map(|v| v[0])
            .enumerate()
        {
            if i < self.window {
                res += el;
                continue;
            }
            if i == self.window - 1 {
                res /= window_t;
            }
            res = self.ind(&[el, res, alpha]);
        }
        RefCell::new(vec![MAP::from_iter([
            ("alpha", vec![alpha]),
            ("res", vec![res]),
        ])])
    }
    fn ind_with_bf<'a>(
        &self,
        in_: &[f64],
        bf: &RefCell<Vec<MAP<&'a str, Vec<f64>>>>,
        index_: usize,
    ) -> f64 {
        let res =
            self.ind(&[in_[0], bf.borrow()[index_]["res"][0], bf.borrow()[index_]["alpha"][0]]);
        bf.borrow_mut()[index_].insert("res", vec![res]);
        res
    }
}

impl IndicatorExt for RMA {}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use bc_utils_lg::statics::prices::OPEN;

    use crate::rma::*;
    use crate::test_funcs::test_funcs::*;

    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        OPEN.iter()
            .copied()
            .map(|v| vec![v])
            .collect::<Vec<Vec<f64>>>()
    });
    const RES: f64 = 2.2548879972457887;

    #[test]
    fn rma_bf_res_1() {
        let settings = RMA::new(2);
        test_bf_res_1(settings, &IN_, RES);
    }

    #[test]
    fn rma_f_res_1() {
        let settings = RMA::new(2);
        test_f_res_1(settings, &IN_, RES);
    }

    #[test]
    fn rma_coll_res_1() {
        let settings = RMA::new(2);
        test_coll_res_1(settings, &IN_, RES, 21);
    }

    #[test]
    fn rma_coll_res_2() {
        let settings = RMA::new(2);
        test_coll_res_2(settings, &IN_, 30);
    }
}
