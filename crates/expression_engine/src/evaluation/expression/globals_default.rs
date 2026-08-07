use crate::{ComputedItem, GlobalObjectComputedData};
use std::collections::BTreeMap;

/// Returns a `GlobalObjectComputedData` containing default global constants.
pub(crate) fn default_globals() -> GlobalObjectComputedData {
    let mut globals_map = BTreeMap::new();

    for (name, value) in [
        ("g_e", std::f64::consts::E),
        ("g_frac_1_pi", std::f64::consts::FRAC_1_PI),
        ("g_frac_1_sqrt_2", std::f64::consts::FRAC_1_SQRT_2),
        ("g_frac_2_pi", std::f64::consts::FRAC_2_PI),
        ("g_frac_2_sqrt_pi", std::f64::consts::FRAC_2_SQRT_PI),
        ("g_frac_pi_2", std::f64::consts::FRAC_PI_2),
        ("g_frac_pi_3", std::f64::consts::FRAC_PI_3),
        ("g_frac_pi_4", std::f64::consts::FRAC_PI_4),
        ("g_frac_pi_6", std::f64::consts::FRAC_PI_6),
        ("g_frac_pi_8", std::f64::consts::FRAC_PI_8),
        ("g_ln_2", std::f64::consts::LN_2),
        ("g_ln_10", std::f64::consts::LN_10),
        ("g_log2_e", std::f64::consts::LOG2_E),
        ("g_log10_e", std::f64::consts::LOG10_E),
        ("g_pi", std::f64::consts::PI),
        ("g_sqrt_2", std::f64::consts::SQRT_2),
        ("g_tau", std::f64::consts::TAU),
    ] {
        globals_map.insert(name.into(), ComputedItem::Float(value));
    }

    GlobalObjectComputedData::new(globals_map)
}
