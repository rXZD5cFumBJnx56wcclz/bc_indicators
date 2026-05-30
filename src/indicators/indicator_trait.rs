use std::cell::RefCell;

use rustc_hash::FxHashMap;

pub trait Indicator {
    fn get_window(&self) -> usize;
    fn get_mult_window_accuracy(&self) -> usize;
    fn get_add_window_accuracy(&self) -> usize;
    fn ind(&self, math_operations: &[f64],) -> f64;
    fn bf(&self, in_: &[Vec<f64>],) -> RefCell<Vec<FxHashMap<&'static str, f64>>>;
    fn ind_with_bf<'a>(
        &self,
        in_: &[f64],
        bf: &RefCell<Vec<FxHashMap<&'static str, f64>>>,
        index_: usize,
    ) -> f64;
    fn ind_f(&self, in_:&[Vec<f64>],) -> f64 {
        let bf = self.bf(&in_[
            in_.len() - 1 
            - self.get_window() * self.get_mult_window_accuracy() - self.get_add_window_accuracy()
            ..in_.len() - 1
        ],);
        self.ind_with_bf(&in_[in_.len() - 1], &bf, 0)
    }
    fn ind_coll<C>(&self, in_: &[Vec<f64>],) -> C
    where
        C: FromIterator<f64>,
    {
        let bf = self.bf(
            &in_[..self.get_window() * self.get_mult_window_accuracy() + self.get_add_window_accuracy()],
        );
        in_
            .iter()
            .enumerate()
            .map(
                |v| 
                if v.0 < self.get_window() * self.get_mult_window_accuracy() + self.get_add_window_accuracy() {
                    f64::NAN
                } else {
                    self.ind_with_bf(v.1, &bf.clone(), 0)
                }
            )
            .collect()
    }
}
