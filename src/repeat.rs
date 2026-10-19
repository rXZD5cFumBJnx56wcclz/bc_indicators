use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
pub struct Repeat {
    pub value: f64,
}

impl W for Repeat {
    fn w(&self) -> usize {
        0
    }
}

impl Repeat {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Indicator for Repeat {
    fn ind(&self, _: &[f64]) -> f64 {
        self.value
    }
    fn init_bf(&self, _in_: &[Vec<f64>]) {}
    fn execute_bf(&self) {}
    fn ind_vec(&self, in_: &[Vec<f64>]) -> Vec<f64> {
        (0..in_.len()).map(|_| self.value).collect()
    }
}

impl IndicatorExt for Repeat {
    fn ind_coll<C>(&self, in_: &[Vec<f64>]) -> C
    where
        C: FromIterator<f64>,
    {
        (0..in_.len()).map(|_| self.value).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;

    use crate::prelude_tests::prelude::*;

    static RES: f64 = 1.0;
    static IN_: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
        (0..OPEN.len())
            .map(|i| vec![OPEN[i], CLOSE[i]])
            .collect::<Vec<Vec<f64>>>()
    });
    static SETTINGS_: LazyLock<Repeat> = LazyLock::new(|| Repeat::new(1.0));

    #[test]
    fn repeat_bf_res_1() {
        test_ind_bf_res_1((*SETTINGS_).clone(), &IN_, RES);
    }
}
