use crate::built_in_registry::BuiltInRegistryItem;
use crate::built_in_registry::category::category;
use crate::built_in_registry::signal::gain_v1::gain_definition::GAIN_V1;
use keys::component_key;

/// Version 1 of the gain component.
pub mod gain_v1;

/// Registry entry for the gain component.
pub static GAIN: BuiltInRegistryItem = BuiltInRegistryItem::new(
    component_key!("gain"),
    "Gain",
    category!("Signal"),
    &GAIN_V1,
    &[],
);
