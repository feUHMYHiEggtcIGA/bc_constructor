use std::sync::LazyLock;

use bc_utils_lg::statics::prices::SRC_TRANSPOSE;
use bc_utils_lg::{
    structs::settings::{SETTINGS_IND, SETTINGS_INDS, SETTINGS_USED_SRC},
    types::maps::MAP,
};
use criterion::{Criterion, criterion_group, criterion_main};

use bc_constructor::{
    indicators::get_indications_from_settings,
    map::indicators::{
        FUNCS_EXTRACT_ARGS, get_indicators_from_settings, get_indicators_from_settings_without_bf,
    },
};

static SETTINGS_1: LazyLock<SETTINGS_INDS> = LazyLock::new(|| {
    SETTINGS_INDS::from_iter([(
        "rsi_1".to_string(),
        SETTINGS_IND {
            key: "rsi".to_string(),
            kwargs_usize: MAP::from_iter([("window".to_string(), 2)]),
            kwargs_f64: MAP::default(),
            kwargs_string: MAP::default(),
            used_src: vec![SETTINGS_USED_SRC { key: "open".to_string(), sub_from_last_i: 0 }],
            used_ind: vec![],
        },
    )])
});

static SETTINGS_2: LazyLock<SETTINGS_INDS> = LazyLock::new(|| {
    SETTINGS_INDS::from_iter([
        (
            "avg_1".to_string(),
            SETTINGS_IND {
                key: "avg".to_string(),
                kwargs_usize: MAP::from_iter([]),
                kwargs_f64: MAP::default(),
                kwargs_string: MAP::default(),
                used_src: vec![
                    SETTINGS_USED_SRC { key: "open".to_string(), sub_from_last_i: 0 },
                    SETTINGS_USED_SRC { key: "high".to_string(), sub_from_last_i: 1 },
                    SETTINGS_USED_SRC { key: "low".to_string(), sub_from_last_i: 1 },
                    SETTINGS_USED_SRC { key: "close".to_string(), sub_from_last_i: 1 },
                ],
                used_ind: vec![],
            },
        ),
        (
            "ema_1".to_string(),
            SETTINGS_IND {
                key: "ema".to_string(),
                kwargs_usize: MAP::from_iter([("window".to_string(), 4)]),
                kwargs_f64: MAP::default(),
                kwargs_string: MAP::default(),
                used_src: vec![],
                used_ind: vec!["avg_1".to_string()],
            },
        ),
        (
            "trend_ma_1".to_string(),
            SETTINGS_IND {
                key: "trend_ma".to_string(),
                kwargs_usize: MAP::from_iter([]),
                kwargs_f64: MAP::default(),
                kwargs_string: MAP::default(),
                used_src: vec![],
                used_ind: vec!["ema_1".to_string()],
            },
        ),
    ])
});

fn get_indications_from_settings_1(c: &mut Criterion) {
    let indicators_no_bf =
        get_indicators_from_settings_without_bf(&SETTINGS_1, &FUNCS_EXTRACT_ARGS());
    let indicators = get_indicators_from_settings(
        &SETTINGS_1,
        &FUNCS_EXTRACT_ARGS(),
        &SRC_TRANSPOSE,
        &indicators_no_bf,
    );
    c.bench_function("get_indications_from_settings_1", |b| {
        b.iter(|| get_indications_from_settings(&SETTINGS_1, &SRC_TRANSPOSE, &indicators))
    });
}

fn get_indications_from_settings_2(c: &mut Criterion) {
    let indicators_no_bf =
        get_indicators_from_settings_without_bf(&SETTINGS_2, &FUNCS_EXTRACT_ARGS());
    let indicators = get_indicators_from_settings(
        &SETTINGS_2,
        &FUNCS_EXTRACT_ARGS(),
        &SRC_TRANSPOSE,
        &indicators_no_bf,
    );
    c.bench_function("get_indications_from_settings_2", |b| {
        b.iter(|| get_indications_from_settings(&SETTINGS_2, &SRC_TRANSPOSE, &indicators))
    });
}

criterion_group!(
    benches,
    get_indications_from_settings_1,
    get_indications_from_settings_2
);
criterion_main!(benches);
