use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct CountApplyParams {
    pub count: usize,
    pub fixed_val: Option<f64>,
}

impl Default for CountApplyParams {
    fn default() -> Self {
        Self {
            count: 10,
            fixed_val: None,
        }
    }
}

impl CountApplyParams {
    pub fn new(count: usize) -> Self {
        Self {
            count,
            fixed_val: None,
        }
    }

    pub fn with_fixed_val(mut self, fixed_val: Option<f64>) -> Self {
        self.fixed_val = fixed_val;
        self
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone, Copy)]
pub struct CountApplyBf {
    pub count: usize,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
pub struct CountApply {
    pub params: CountApplyParams,
    bf: RefCell<CountApplyBf>,
    bf_state: RefCell<CountApplyBf>,
}

impl CountApply {
    pub fn new(count: usize) -> Self {
        Self {
            params: CountApplyParams::new(count),
            ..Default::default()
        }
    }

    pub fn new_with_fixed(count: usize, fixed_val: Option<f64>) -> Self {
        Self {
            params: CountApplyParams::new(count).with_fixed_val(fixed_val),
            ..Default::default()
        }
    }

    pub fn new_full(count: usize, fixed_val: Option<f64>) -> Self {
        Self::new_with_fixed(count, fixed_val)
    }
}

impl W for CountApply {
    fn w(&self) -> usize {
        self.params.count
    }
}

impl Indicator for CountApply {
    fn init_bf(&self, in_: &[Vec<f64>]) {
        let count = if self.params.count > 0 {
            in_.len() % self.params.count
        } else {
            0
        };
        self.bf.borrow_mut().count = count;
        self.bf_state.borrow_mut().count = count;
    }

    fn ind(&self, in_: &[f64]) -> f64 {
        let target = self.params.count.max(1);
        let cur = self.bf.borrow().count + 1;
        let reached = cur >= target;

        let res = if reached {
            if let Some(fixed) = self.params.fixed_val {
                fixed
            } else if in_.len() > 2 {
                in_[2]
            } else if in_.len() > 1 {
                in_[1]
            } else if !in_.is_empty() {
                in_[0]
            } else {
                0.0
            }
        } else if !in_.is_empty() {
            in_[0]
        } else {
            0.0
        };

        // When target count is reached, it outputs once and then automatically resets to 0
        let next_count = if reached { 0 } else { cur };

        self.bf_state.borrow_mut().count = next_count;
        res
    }
}

impl ExecuteBf for CountApply {
    fn execute_bf(&self) {
        *self.bf.borrow_mut() = *self.bf_state.borrow();
    }
}

impl IndicatorExt for CountApply {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;

    use crate::prelude_tests::prelude::*;

    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        OPEN.iter()
            .copied()
            .zip(HIGH.iter().copied())
            .zip(LOW.iter().copied())
            .map(|((o, h), l)| vec![o, h, l])
            .collect::<Vec<Vec<f64>>>()
    });

    #[test]
    fn count_apply_cycle_res_1() {
        let ind = CountApply::new(3);

        // Call 1: count=1 < 3 -> returns src[0]
        let res1 = ind.ind(&[10.0, 50.0, 99.0]);
        assert_eq!(res1, 10.0);
        ind.execute_bf();

        // Call 2: count=2 < 3 -> returns src[0]
        let res2 = ind.ind(&[20.0, 50.0, 99.0]);
        assert_eq!(res2, 20.0);
        ind.execute_bf();

        // Call 3: count=3 == 3 -> returns src[2] (99.0)
        let res3 = ind.ind(&[30.0, 50.0, 99.0]);
        assert_eq!(res3, 99.0);
        ind.execute_bf();

        // Call 4: count resets to 0 -> next is count 1 < 3 -> returns src[0]
        let res4 = ind.ind(&[40.0, 50.0, 99.0]);
        assert_eq!(res4, 40.0);
        ind.execute_bf();
    }

    #[test]
    fn count_apply_fixed_res_1() {
        let ind = CountApply::new_with_fixed(2, Some(123.45));

        // Call 1: count=1 < 2 -> src[0]
        assert_eq!(ind.ind(&[10.0, 50.0, 99.0]), 10.0);
        ind.execute_bf();

        // Call 2: count=2 reached -> fixed value 123.45
        assert_eq!(ind.ind(&[20.0, 50.0, 99.0]), 123.45);
        ind.execute_bf();

        // Call 3: automatically reset to 0 -> returns src[0]
        assert_eq!(ind.ind(&[30.0, 50.0, 99.0]), 30.0);
        ind.execute_bf();
    }

    #[test]
    fn count_apply_src2_res_1() {
        let ind = CountApply::new(2);

        // Call 1: src[0]
        assert_eq!(ind.ind(&[100.0, 200.0, 300.0]), 100.0);
        ind.execute_bf();

        // Call 2: reached count -> returns src[2] = 300.0
        assert_eq!(ind.ind(&[110.0, 210.0, 310.0]), 310.0);
        ind.execute_bf();

        // Call 3: reset -> returns src[0]
        assert_eq!(ind.ind(&[120.0, 220.0, 320.0]), 120.0);
        ind.execute_bf();
    }

    #[test]
    fn count_apply_recalc_res_1() {
        let ind = CountApply::new(2);

        // Bar 1 tick 1
        assert_eq!(ind.ind(&[10.0, 50.0, 99.0]), 10.0);
        // Bar 1 tick 2 (no execute_bf in between)
        assert_eq!(ind.ind(&[15.0, 50.0, 99.0]), 15.0);
        // Bar 1 close
        ind.execute_bf();

        // Bar 2 tick 1 -> target reached -> returns src[2]
        assert_eq!(ind.ind(&[20.0, 50.0, 99.0]), 99.0);
        // Bar 2 tick 2 (still within bar 2)
        assert_eq!(ind.ind(&[25.0, 50.0, 99.0]), 99.0);
        // Bar 2 close
        ind.execute_bf();

        // Bar 3 tick 1 -> reset -> returns src[0]
        assert_eq!(ind.ind(&[30.0, 50.0, 99.0]), 30.0);
    }

    #[test]
    fn count_apply_coll_res_1() {
        let ind = CountApply::new(5);
        test_coll_res_1(ind, &IN_, 20);
    }
}
