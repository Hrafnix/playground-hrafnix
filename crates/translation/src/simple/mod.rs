/// This module contains Hrafnix command-line translations.
pub(crate) mod hrafnix;

use crate::simple::hrafnix::add_hrafnix_translation_map;
use shareable_string::SharedStringTranslationMap;

/// Adds simple translations to the provided `SharedStringTranslationMap`.
pub(crate) fn add_simple_translations(translation_map: &mut SharedStringTranslationMap) {
    add_hrafnix_translation_map(translation_map);
}
