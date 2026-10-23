use bc_utils::nums::dz;

use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct PnaParams {
    pub window: usize,
    pub fixed_value: Option<f64>,
    pub min_new: f64,
    pub max_new: f64,
}

impl Default for PnaParams {
    fn default() -> Self {
        Self {
            window: 14,
            fixed_value: None,
            min_new: 0.0,
            max_new: 1.0,
        }
    }
}

impl PnaParams {
    pub fn new(window: usize, fixed_value: Option<f64>, min_new: f64, max_new: f64) -> Self {
        Self {
            window,
            fixed_value,
            min_new,
            max_new,
        }
    }

    pub fn with_fixed_value(mut self, fixed_value: Option<f64>) -> Self {
        self.fixed_value = fixed_value;
        self
    }

    pub fn with_range(mut self, min_new: f64, max_new: f64) -> Self {
        self.min_new = min_new;
        self.max_new = max_new;
        self
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct PnaBf {
    pub src_l: Vec<f64>,
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct Pna {
    pub params: PnaParams,
    bf: RefCell<PnaBf>,
    bf_state: RefCell<PnaBf>,
}

impl Pna {
    pub fn new(window: usize, fixed_value: Option<f64>, min_new: f64, max_new: f64) -> Self {
        Self {
            params: PnaParams::new(window, fixed_value, min_new, max_new),
            ..Default::default()
        }
    }

    fn calc_raw_pct_abs(&self, row: &[f64]) -> f64 {
        let rhs = if let Some(fixed) = self.params.fixed_value {
            fixed
        } else if row.len() > 1 {
            row[1]
        } else {
            0.0
        };
        let lhs = if !row.is_empty() { row[0] } else { 0.0 };
        let pct = (lhs - rhs) / dz(lhs);
        pct.abs()
    }
}

impl W for Pna {
    fn w(&self) -> usize {
        self.params.window
    }
}

impl Indicator for Pna {
    fn init_bf(&self, in_: &[Vec<f64>]) {
        let w = self.w();
        let slice = if in_.len() >= w {
            &in_[in_.len() - w..]
        } else {
            in_
        };
        let vals: Vec<f64> = slice.iter().map(|row| self.calc_raw_pct_abs(row)).collect();
        self.bf.borrow_mut().src_l = vals.clone();
        self.bf_state.borrow_mut().src_l = vals;
    }

    fn ind(&self, in_: &[f64]) -> f64 {
        let val = self.calc_raw_pct_abs(in_);
        let mut bind =
            if self.bf.borrow().src_l.len() >= self.w() && !self.bf.borrow().src_l.is_empty() {
                self.bf.borrow().src_l[1..].to_vec()
            } else {
                self.bf.borrow().src_l.clone()
            };
        bind.push(val);

        let min_historic = bind.iter().copied().fold(f64::INFINITY, f64::min);
        let max_historic = bind.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        let denom = dz(max_historic - min_historic);
        let normalized = (val - min_historic) / denom * (self.params.max_new - self.params.min_new)
            + self.params.min_new;
        let res = normalized.abs();

        self.bf_state.borrow_mut().src_l = bind;
        res
    }
}

impl ExecuteBf for Pna {
    fn execute_bf(&self) {
        *self.bf.borrow_mut() = self.bf_state.borrow().clone();
    }
}

impl IndicatorExt for Pna {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;

    use crate::prelude_tests::prelude::*;

    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        OPEN.iter()
            .copied()
            .zip(CLOSE.iter().copied())
            .map(|(o, c)| vec![o, c])
            .collect::<Vec<Vec<f64>>>()
    });

    #[test]
    fn pna_range_res_1() {
        let ind = Pna {
            params: PnaParams {
                window: 5,
                ..Default::default()
            },
            ..Default::default()
        };
        for row in IN_.iter().take(20) {
            let res = ind.ind(row);
            ind.execute_bf();
            assert!(res >= 0.0, "Result must be non-negative: {}", res);
            assert!(
                res <= 1.0 + 1e-4,
                "Result must not exceed max range: {}",
                res
            );
        }
    }

    #[test]
    fn pna_fixed_res_1() {
        let ind = Pna {
            params: PnaParams {
                window: 3,
                fixed_value: Some(100.),
                ..Default::default()
            },
            ..Default::default()
        };
        let rows = [vec![90.0], vec![110.0], vec![120.0], vec![100.0]];
        for r in &rows {
            let res = ind.ind(r);
            ind.execute_bf();
            assert!(res >= 0.0);
        }
    }

    #[test]
    fn pna_custom_res_1() {
        let ind = Pna {
            params: PnaParams {
                window: 3,
                fixed_value: None,
                min_new: 10.,
                max_new: 50.,
            },
            ..Default::default()
        };
        for row in IN_.iter().take(15) {
            let res = ind.ind(row);
            ind.execute_bf();
            assert!(res >= 10.0 - 1e-4);
            assert!(res <= 50.0 + 1e-4);
        }
    }

    #[test]
    fn pna_realtime_res_1() {
        let ind = Pna {
            params: PnaParams {
                window: 3,
                ..Default::default()
            },
            ..Default::default()
        };
        ind.init_bf(&IN_[..10]);

        let row1 = &IN_[10];
        let val1_tick1 = ind.ind(row1);
        let val1_tick2 = ind.ind(row1);
        assert_eq!(val1_tick1, val1_tick2);
        ind.execute_bf();

        let row2 = &IN_[11];
        let val2 = ind.ind(row2);
        assert!(val2 >= 0.0);
    }

    #[test]
    fn pna_coll_res_1() {
        let ind = Pna {
            params: PnaParams {
                window: 5,
                ..Default::default()
            },
            ..Default::default()
        };
        test_coll_res_1(ind, &IN_, 10);
    }
}
