use bc_indicators::indicators::ready_imports::*;
use bc_indicators::indicators::{rma::RMA, rsi::RSI};
use bc_utils_lg::structs::settings::{SETTINGS_IND, SETTINGS_INDS};
use bc_utils_lg::types::maps::{
    // MAP,
    MAP_LINK,
};

pub fn get_indicators_default() -> FxHashMap<&'static str, Box<dyn Indicator>> {
    FxHashMap::from_iter([
        ("rsi", Box::new(RSI::default()) as Box<dyn Indicator>),
        ("rma", Box::new(RMA::default()) as Box<dyn Indicator>),
    ])
}

pub fn get_funcs_extract_args() -> FxHashMap<&'static str, fn(&SETTINGS_IND) -> Box<dyn Indicator>>
{
    FxHashMap::from_iter([
        (
            "rsi",
            (|v: &SETTINGS_IND| {
                let mut df = RSI::default();
                df.set_window(*v.kwargs_usize.get("window").unwrap_or(&df.window));
                df.set_mult_window_accuracy(
                    *v.kwargs_usize
                        .get("mult_window_accuracy")
                        .unwrap_or(&df.mult_window_accuracy),
                );
                df.set_add_window_accuracy(
                    *v.kwargs_usize
                        .get("add_window_accuracy")
                        .unwrap_or(&df.add_window_accuracy),
                );
                Box::new(df) as Box<dyn Indicator>
            }) as fn(&SETTINGS_IND) -> Box<dyn Indicator>,
        ),
        (
            "rma",
            (|v: &SETTINGS_IND| {
                let mut df = RMA::default();
                df.set_window(*v.kwargs_usize.get("window").unwrap_or(&df.window));
                df.set_mult_window_accuracy(
                    *v.kwargs_usize
                        .get("mult_window_accuracy")
                        .unwrap_or(&df.mult_window_accuracy),
                );
                df.set_add_window_accuracy(
                    *v.kwargs_usize
                        .get("add_window_accuracy")
                        .unwrap_or(&df.add_window_accuracy),
                );
                Box::new(df) as Box<dyn Indicator>
            }) as fn(&SETTINGS_IND) -> Box<dyn Indicator>,
        ),
    ])
    // FxHashMap::default()
}

pub fn get_indicators_from_settings<'a>(
    settings: &'a SETTINGS_INDS,
    funcs_extract_args: &FxHashMap<&'a str, fn(&SETTINGS_IND) -> Box<dyn Indicator>>,
) -> MAP_LINK<&'a str, Box<dyn Indicator>> {
    settings
        .iter()
        .map(|(indicator_name, settings_indicator)| {
            (
                indicator_name.as_str(),
                funcs_extract_args[settings_indicator.key.as_str()](settings_indicator),
            )
        })
        .collect()
}
