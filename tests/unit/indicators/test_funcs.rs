use bc_utils::nums::round_f;

use bc_indicators::indicators::ready_imports::*;

pub fn test_bf_res_1<T>(settings_indicator: T, in_: &[Vec<f64>], eq: f64)
where
    T: Indicator,
    T: IndicatorExt,
{
    let bf = settings_indicator.bf(in_
        .into_iter()
        .cloned()
        .take(in_.len() - 1)
        .collect::<Vec<Vec<f64>>>()
        .as_slice());
    assert_eq!(
        settings_indicator.ind_with_bf(in_.last().unwrap(), &bf, 0),
        eq,
    );
}

pub fn test_f_res_1<T>(settings_indicator: T, in_: &[Vec<f64>], eq: f64)
where
    T: Indicator,
    T: IndicatorExt,
{
    assert_eq!(settings_indicator.ind_f(in_), eq,);
}

pub fn test_coll_res_1<T>(settings_indicator: T, in_: &[Vec<f64>], eq: f64, len_elements: usize)
where
    T: Indicator,
    T: IndicatorExt,
{
    assert_eq!(
        dbg!(settings_indicator.ind_coll::<Vec<_>>(&in_[dbg!(in_.len() - len_elements)..]))
            [len_elements - 1],
        eq,
    );
}

pub fn test_coll_res_2<T>(settings_indicator: T, in_: &[Vec<f64>], len_elements: usize)
where
    T: IndicatorExt,
{
    let in_ = &in_[in_.len() - len_elements..];
    assert_eq!(
        round_f(
            settings_indicator.ind_coll::<Vec<_>>(in_)[len_elements - 1],
            &4
        ),
        round_f(settings_indicator.ind_f(in_), &4),
    );
}
