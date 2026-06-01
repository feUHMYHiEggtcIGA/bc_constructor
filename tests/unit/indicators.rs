use std::any::Any;

use bc_indicators::indicators::ready_imports::Indicator;
use bc_indicators::indicators::{rma::RMA, rsi::RSI};
use bc_utils_lg::statics::prices::{OPEN, OPEN_LAST, SRC_TRANSPOSE};
use bc_utils_lg::structs::settings::{SETTINGS_IND, SETTINGS_INDS, SETTINGS_USED_SRC};
use bc_utils_lg::types::maps::{MAP, MAP_LINK};

use bc_constructor::indicators::*;
use bc_constructor::map::indicators::*;

#[test]
fn indication_res_1() {
    let settings = SETTINGS_INDS::from_iter([(
        "rsi_1".to_string(),
        SETTINGS_IND {
            key: "rsi".to_string(),
            kwargs_usize: MAP::from_iter([("window".to_string(), 2)]),
            kwargs_f64: MAP::default(),
            kwargs_string: MAP::default(),
            used_src: vec![SETTINGS_USED_SRC { key: "open".to_string(), sub_from_last_i: 0 }],
            used_ind: vec![],
        },
    )]);
    let funcs_extract_args = get_funcs_extract_args();
    let indicators_without_bf =
        get_indicators_from_settings_without_bf(&settings, &funcs_extract_args);
    let indicators = get_indicators_from_settings(
        &settings,
        &funcs_extract_args,
        &SRC_TRANSPOSE,
        &indicators_without_bf,
    );
    let res_1 = get_indications_from_settings(&settings, &SRC_TRANSPOSE, &indicators);
    let res_2 = RSI::new(2).ind_f(&OPEN.into_iter().map(|v| vec![v]).collect::<Vec<Vec<f64>>>());
    assert_eq!(res_1["rsi_1"], res_2,);
}
