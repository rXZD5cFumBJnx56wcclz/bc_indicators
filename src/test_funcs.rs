#[cfg(test)]
pub mod funcs {

    use pretty_assertions::assert_eq as assert_eq_pr;

    use crate::prelude::*;

    pub fn test_ind_bf_res_1<T>(ind: T, src: &[Vec<f64>], eq: f64)
    where
        T: Indicator,
        T: IndicatorExt,
    {
        ind.init_bf(
            &src.iter()
                .take(src.len() - 1)
                .cloned()
                .collect::<Vec<Vec<f64>>>(),
        );
        assert_eq_pr!(ind.ind(src.last().unwrap(),), eq,);
    }
    pub fn test_coll_res_1<T>(ind: T, src: &[Vec<f64>], interval_len: usize)
    where
        T: Indicator + Clone,
        T: IndicatorExt,
    {
        let ind2 = ind.clone();
        for el in [&ind, &ind2] {
            el.init_bf(
                src.iter()
                    .take(src.len() - 1 - interval_len)
                    .cloned()
                    .collect::<Vec<Vec<f64>>>()
                    .as_slice(),
            );
        }
        for el in &src[src.len() - interval_len..src.len() - 1] {
            ind2.ind(el);
            ind2.execute_bf();
        }
        assert_eq_pr!(
            *ind.ind_vec(&src[src.len() - interval_len..])
                .last()
                .unwrap(),
            ind2.ind(&src[src.len() - 1]),
        );
    }
}
