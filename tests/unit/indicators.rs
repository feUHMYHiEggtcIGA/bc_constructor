use bc_indicators::indicators::ready_imports::Indicator;
use bc_indicators::indicators::{rma::RMA, rsi::RSI};
use bc_utils::nums::round_f;
use bc_utils_lg::statics::prices::{CLOSE, OPEN, OPEN_LAST, SRC_TRANSPOSE};
use bc_utils_lg::structs::settings::{SETTINGS_IND, SETTINGS_INDS, SETTINGS_USED_SRC};
use bc_utils_lg::types::maps::MAP;

use bc_constructor::indicators::*;
use bc_constructor::map::indicators::*;

#[test]
fn indication_res_1() {
    let settings = SETTINGS_INDS::from_iter([
        (
            "rsi_1".to_string(),
            SETTINGS_IND {
                key: "rsi".to_string(),
                kwargs_usize: MAP::from_iter([("window".to_string(), 2)]),
                kwargs_f64: MAP::default(),
                kwargs_string: MAP::default(),
                used_src: vec![SETTINGS_USED_SRC { key: "open".to_string(), sub_from_last_i: 0 }],
                used_ind: vec![],
            },
        ),
        (
            "rma_1".to_string(),
            SETTINGS_IND {
                key: "rma".to_string(),
                kwargs_usize: MAP::from_iter([("window".to_string(), 2)]),
                kwargs_f64: MAP::default(),
                kwargs_string: MAP::default(),
                used_src: vec![],
                used_ind: vec!["rsi_1".to_string()],
            },
        ),
        (
            "avg_1".to_string(),
            SETTINGS_IND {
                key: "avg".to_string(),
                kwargs_usize: MAP::from_iter([]),
                kwargs_f64: MAP::default(),
                kwargs_string: MAP::default(),
                used_src: vec![
                    SETTINGS_USED_SRC { key: "open".to_string(), sub_from_last_i: 0 },
                    SETTINGS_USED_SRC { key: "close".to_string(), sub_from_last_i: 2 },
                ],
                used_ind: vec!["rma_1".to_string()],
            },
        ),
    ]);
    let funcs_extract_args = FUNCS_EXTRACT_ARGS();
    let indicators_without_bf =
        get_indicators_from_settings_without_bf(&settings, &funcs_extract_args);
    let indicators = get_indicators_from_settings(
        &settings,
        &funcs_extract_args,
        &SRC_TRANSPOSE,
        &indicators_without_bf,
    );
    let res_1 = get_indications_from_settings(&settings, &SRC_TRANSPOSE, &indicators);
    let res_2 = (RMA::new(2).ind_f(
        &RSI::new(2)
            .ind_vec(&OPEN.into_iter().map(|v| vec![v]).collect::<Vec<Vec<f64>>>())
            .into_iter()
            .map(|v| vec![v])
            .collect::<Vec<Vec<f64>>>(),
    ) + CLOSE[47]
        + OPEN_LAST)
        / 3.;
    assert_eq!(round_f(res_1["avg_1"], &4,), round_f(res_2, &4,),);
}
