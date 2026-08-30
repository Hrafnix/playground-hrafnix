use crate::definitions::built_in_component_definition::built_in_component_definition;
use crate::definitions::icon_definition::icon_definition;
use crate::definitions::port_definition::port_definition;
use crate::{BuiltInComponentDefinition, PortKind, Rotation};
use datastore::{item_compile_time, number_compile_time};

/// Version 1 definition of the gain component.
pub static GAIN_V1: BuiltInComponentDefinition = built_in_component_definition!(
    "gain",
    1,
    "Gain",
    datastore::parameter_object_compile_time!(
        "Parameters",
        [(
            "p_gain",
            item_compile_time!(number = number_compile_time!("Gain")),
        ),]
    ),
    datastore::variable_object_compile_time!("Variables", []),
    icon_definition!(include_str!("gain.svg"), (32, 32)),
    [
        port_definition!(
            "input",
            "Input",
            PortKind::SignalInput,
            (0, 16),
            (32, 32),
            Rotation::Degrees0,
            true
        ),
        port_definition!(
            "output",
            "Output",
            PortKind::SignalOutput,
            (32, 16),
            (32, 32),
            Rotation::Degrees0,
            false
        ),
    ],
);
