use bc_indicators::indicators::{rma::RMA, rsi::RSI};
use bc_utils_lg::structs::settings::{SETTINGS_IND, SETTINGS_INDS, SETTINGS_USED_SRC};
use bc_utils_lg::types::maps::{MAP, MAP_LINK};

use bc_constructor::map::indicators::*;

use std::any::Any;

#[test]
fn indicators_from_settings_res_1() {
    let settings = SETTINGS_INDS::from_iter([(
        "rsi_1".to_string(),
        SETTINGS_IND {
            key: "rsi".to_string(),
            kwargs_usize: MAP::from_iter([("window".to_string(), 10)]),
            kwargs_f64: MAP::default(),
            kwargs_string: MAP::default(),
            used_src: vec![],
            used_ind: vec![],
        },
    )]);
    let funcs_extract_args = get_funcs_extract_args();
    let res = get_indicators_from_settings_without_bf(&settings, &funcs_extract_args);
    let res_1 = res.get("rsi_1").unwrap().as_ref();
    let rsi_test_1 = RSI::new(10);
    let rsi_test_2 = (res_1 as &dyn Any).downcast_ref::<RSI>().unwrap();
    assert_eq!(&rsi_test_1, rsi_test_2);
}
