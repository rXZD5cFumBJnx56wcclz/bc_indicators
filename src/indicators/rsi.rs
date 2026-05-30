use crate::indicators::ready_imports::*;
use crate::indicators::rma::RMA;

pub struct RSI {
    window: usize,
}

impl Indicator for RSI {
    fn ind(math_operations: &[f64], _: &Self) -> f64 {
        (100.0 - (100.0 / (1.0 + math_operations[0] / math_operations[1]))) / 100.0
    }
    fn bf(
        in_: &[&[f64]],
        settings: &Self,
    ) -> std::cell::RefCell<Vec<FxHashMap<&'static str, f64>>> {
        let mut u = Vec::new();
        let mut d = Vec::new();
        let mut src_l = f64::NAN;
        let len_src = in_.len();
        let _w = settings.window * 10;

        for (i, el) in in_[len_src - _w - 2..len_src - 1]
        .iter()
        .enumerate()
        {
            if i == 0 {
                src_l = *el.borrow();
                continue;
            }
            let change = *el.borrow() - src_l;
            u.push(change.max(T::zero()));
            d.push((-change).max(T::zero()));
            src_l = *el.borrow();
        }
        (
            FxHashMap::from_iter([("src", src_l)]),
            bf_rma::<T, T>(u.as_slice(), window, &false),
            bf_rma::<T, T>(d.as_slice(), window, &false),
        )
    }
}
