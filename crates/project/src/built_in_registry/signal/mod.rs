use crate::built_in_registry::BuiltInRegistryItem;
use crate::built_in_registry::category::category;
use crate::built_in_registry::signal::add_v1::add_definition::AddV1;
use crate::built_in_registry::signal::constant_v1::constant_definition::ConstantV1;
use crate::built_in_registry::signal::delay_v1::delay_definition::DelayV1;
use crate::built_in_registry::signal::gain_v1::gain_definition::GainV1;
use keys::component_key;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Version 1 of the add component.
pub mod add_v1;
/// Version 1 of the constant component.
pub mod constant_v1;
/// Version 1 of the delay component.
pub mod delay_v1;
/// Version 1 of the gain component.
pub mod gain_v1;

/// Registry entry for the add component.
pub static ADD: LazyLock<BuiltInRegistryItem> = LazyLock::new(|| {
    BuiltInRegistryItem::new(
        component_key!("add"),
        "Add",
        category!("Signal", "Math"),
        Box::new(AddV1),
        HashMap::new(),
    )
});

/// Registry entry for the constant component.
pub static CONSTANT: LazyLock<BuiltInRegistryItem> = LazyLock::new(|| {
    BuiltInRegistryItem::new(
        component_key!("constant"),
        "Constant",
        category!("Signal", "Sources"),
        Box::new(ConstantV1),
        HashMap::new(),
    )
});

/// Registry entry for the one-step delay component.
pub static DELAY: LazyLock<BuiltInRegistryItem> = LazyLock::new(|| {
    BuiltInRegistryItem::new(
        component_key!("delay"),
        "Delay",
        category!("Signal", "Control"),
        Box::new(DelayV1),
        HashMap::new(),
    )
});

/// Registry entry for the gain component.
pub static GAIN: LazyLock<BuiltInRegistryItem> = LazyLock::new(|| {
    BuiltInRegistryItem::new(
        component_key!("gain"),
        "Gain",
        category!("Signal"),
        Box::new(GainV1),
        HashMap::new(),
    )
});
