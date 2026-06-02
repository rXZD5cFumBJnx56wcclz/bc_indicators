use bc_utils::nums::round_f;
use bc_utils_lg::statics::prices::OPEN;

use bc_indicators::indicators::ready_imports::*;

pub fn test_bf_res_1<T>(settings_indicator: T, eq: f64)
where
    T: Indicator,
    T: IndicatorExt,
{
    let bf = settings_indicator.bf(OPEN
        .into_iter()
        .take(OPEN.len() - 1)
        .map(|v| vec![v])
        .collect::<Vec<Vec<f64>>>()
        .as_slice());
    assert_eq!(settings_indicator.ind_with_bf(&[2.2547], &bf, 0), eq,);
}

pub fn test_f_res_1<T>(settings_indicator: T, eq: f64)
where
    T: Indicator,
    T: IndicatorExt,
{
    assert_eq!(
        settings_indicator.ind_f(
            OPEN.into_iter()
                .map(|v| vec![v])
                .collect::<Vec<Vec<f64>>>()
                .as_slice()
        ),
        eq,
    );
}

pub fn test_coll_res_1<T>(settings_indicator: T, eq: f64, len_elements: usize)
where
    T: Indicator,
    T: IndicatorExt,
{
    assert_eq!(
        dbg!(
            settings_indicator.ind_coll::<Vec<_>>(
                OPEN[OPEN.len() - len_elements..]
                    .into_iter()
                    .map(|v| vec![*v])
                    .collect::<Vec<Vec<f64>>>()
                    .as_slice()
            )
        )[len_elements - 1],
        eq,
    );
}

pub fn test_coll_res_2<T>(settings_indicator: T, len_elements: usize)
where
    T: IndicatorExt,
{
    let in_ = OPEN[OPEN.len() - len_elements..]
        .into_iter()
        .map(|v| vec![*v])
        .collect::<Vec<Vec<f64>>>();
    assert_eq!(
        round_f(
            settings_indicator.ind_coll::<Vec<_>>(&in_)[len_elements - 1],
            &4
        ),
        round_f(settings_indicator.ind_f(&in_), &4),
    );
}
