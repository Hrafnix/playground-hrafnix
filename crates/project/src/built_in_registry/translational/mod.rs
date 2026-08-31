use crate::built_in_registry::BuiltInRegistryItem;
use crate::built_in_registry::category::category;
use crate::built_in_registry::translational::fixed_boundary_v1::fixed_boundary_definition::FixedBoundaryV1;
use crate::built_in_registry::translational::mass_v1::mass_definition::MassV1;
use crate::built_in_registry::translational::spring_v1::spring_definition::SpringV1;
use keys::component_key;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Version 1 of the fixed boundary component.
pub mod fixed_boundary_v1;
/// Version 1 of the lumped mass component.
pub mod mass_v1;
/// Version 1 of the ideal spring component.
pub mod spring_v1;

/// Registry entry for the fixed boundary.
pub static FIXED_BOUNDARY: LazyLock<BuiltInRegistryItem> = LazyLock::new(|| {
    BuiltInRegistryItem::new(
        component_key!("translational_fixed_boundary"),
        "Fixed Boundary",
        category!("Translational", "Boundaries"),
        Box::new(FixedBoundaryV1),
        HashMap::new(),
    )
});

/// Registry entry for the lumped mass.
pub static MASS: LazyLock<BuiltInRegistryItem> = LazyLock::new(|| {
    BuiltInRegistryItem::new(
        component_key!("translational_mass"),
        "Mass",
        category!("Translational"),
        Box::new(MassV1),
        HashMap::new(),
    )
});

/// Registry entry for the ideal spring.
pub static SPRING: LazyLock<BuiltInRegistryItem> = LazyLock::new(|| {
    BuiltInRegistryItem::new(
        component_key!("translational_spring"),
        "Spring",
        category!("Translational"),
        Box::new(SpringV1),
        HashMap::new(),
    )
});
