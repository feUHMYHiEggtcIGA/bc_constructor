use std::cell::RefCell;

use bc_indicators::indicators::ready_imports::Indicator;
use bc_utils_lg::{
    structs::settings::SETTINGS_INDS,
    types::{maps::MAP, structures::SRC_TRANSPOSE},
};

pub fn get_indications_from_settings<'a>(
    settings: &'a SETTINGS_INDS,
    buffer_in: &SRC_TRANSPOSE,
    map_indicators: &MAP<&'a str, (RefCell<Vec<MAP<&'static str, f64>>>, Box<dyn Indicator>)>,
) -> MAP<&'a str, f64> {
    settings.iter().fold(MAP::default(), |mut map, setting| {
        let key_uniq_str = setting.0.as_str();
        let mut src_arg = vec![];
        for us_el in &setting.1.used_src {
            src_arg.push({
                let sk = &buffer_in[&us_el.key];
                sk[sk.len() - 1 - us_el.sub_from_last_i]
            });
        }
        for ui_el in &setting.1.used_ind {
            src_arg.push(map[ui_el.as_str()]);
        }
        let indicator = &map_indicators[key_uniq_str];
        map.insert(
            key_uniq_str,
            indicator.1.ind_with_bf(src_arg.as_slice(), &indicator.0, 0),
        );
        map
    })
}
