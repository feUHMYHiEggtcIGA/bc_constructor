use bc_indicators::indicators::ready_imports::*;
use bc_indicators::indicators::{rma::RMA, rsi::RSI};
use bc_utils_lg::structs::settings::{SETTINGS_IND, SETTINGS_INDS, SETTINGS_USED_SRC};
use bc_utils_lg::types::maps::MAP;
use bc_utils_lg::types::structures::SRC_TRANSPOSE;

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
}

pub fn get_in_from_settings<'a>(
    used_ind: &Vec<String>,
    used_src: &Vec<SETTINGS_USED_SRC>,
    settings: &SETTINGS_INDS,
    src: &SRC_TRANSPOSE,
    map_indicators: &MAP<&'a str, Box<dyn Indicator>>,
) -> Vec<Vec<f64>> {
    let mut res = vec![];
    for used_src_el in used_src {
        res.push({
            let sk = &src[&used_src_el.key];
            sk[..sk.len() - used_src_el.sub_from_last_i].to_vec()
        });
    }
    for used_ind_el in used_ind {
        res.push(map_indicators[used_ind_el.as_str()].ind_vec(
            // recursive func
            &get_in_from_settings(
                &settings[used_ind_el].used_ind,
                &settings[used_ind_el].used_src,
                settings,
                src,
                map_indicators,
            ),
        ));
    }
    (0..res[0].len())
        .map(|v| res.iter().map(|v1| v1[v]).collect::<Vec<f64>>())
        .collect::<Vec<Vec<f64>>>()
}

pub fn get_indicators_from_settings_without_bf<'a>(
    settings: &'a SETTINGS_INDS,
    funcs_extract_args: &FxHashMap<&'a str, fn(&SETTINGS_IND) -> Box<dyn Indicator>>,
) -> MAP<&'a str, Box<dyn Indicator>> {
    settings
        .iter()
        .map(|(indicator_name, settings_indicator)| {
            let indicator = funcs_extract_args[settings_indicator.key.as_str()](settings_indicator);
            (indicator_name.as_str(), indicator)
        })
        .collect()
}

pub fn get_indicators_from_settings<'a>(
    settings: &'a SETTINGS_INDS,
    funcs_extract_args: &FxHashMap<&'a str, fn(&SETTINGS_IND) -> Box<dyn Indicator>>,
    in_: &SRC_TRANSPOSE,
    map_indicators: &MAP<&'a str, Box<dyn Indicator>>,
) -> MAP<&'a str, (RefCell<Vec<MAP<&'static str, f64>>>, Box<dyn Indicator>)> {
    settings
        .iter()
        .map(|(indicator_name, settings_indicator)| {
            let indicator = funcs_extract_args[settings_indicator.key.as_str()](settings_indicator);
            (
                indicator_name.as_str(),
                (
                    indicator.bf(&get_in_from_settings(
                        &settings_indicator.used_ind,
                        &settings_indicator.used_src,
                        settings,
                        &in_.into_iter()
                            .map(|(v1, v2)| (v1.clone(), v2[..v2.len() - 1].to_vec()))
                            .collect::<MAP<String, Vec<f64>>>(),
                        map_indicators,
                    )),
                    indicator,
                ),
            )
        })
        .collect()
}
