//! Types for standalone component definition files.

use datastore::prelude::{ParameterObjectDefinition, VariableObjectDefinition};

use crate::{
    Connection, InstantiatedComponent, Port, ValidationError, validate_components,
    validate_port_ids,
};

/// Metadata from a component definition file's root element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentMetadata {
    /// The component file format version.
    pub file_version: u16,
    /// The minimum compatible simulation engine version.
    pub minimum_engine_version: String,
    /// The recommended simulation engine version.
    pub recommended_engine_version: String,
    /// The component definition version.
    pub component_version: u16,
    /// The component name.
    pub name: String,
    /// The optional source icon path, resolved relative to the project's `data/icons` directory.
    pub icon_path: Option<String>,
    /// The component's intrinsic length.
    pub length: u16,
    /// The component's intrinsic width.
    pub width: u16,
}

impl ComponentMetadata {
    /// Creates component definition metadata.
    #[must_use]
    pub const fn new(
        file_version: u16,
        minimum_engine_version: String,
        recommended_engine_version: String,
        component_version: u16,
        name: String,
        length: u16,
        width: u16,
    ) -> Self {
        Self {
            file_version,
            minimum_engine_version,
            recommended_engine_version,
            component_version,
            name,
            icon_path: None,
            length,
            width,
        }
    }

    /// Sets the optional source icon path.
    #[must_use]
    pub fn with_icon_path(mut self, icon_path: Option<String>) -> Self {
        self.icon_path = icon_path;
        self
    }
}

/// A complete standalone component definition file.
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentDefinitionFile {
    /// Root file metadata.
    pub metadata: ComponentMetadata,
    /// Parameter object declaration.
    pub parameter_object: ParameterObjectDefinition,
    /// Variable object declaration.
    pub variable_object: VariableObjectDefinition,
    /// Ports exposed by this component.
    pub ports: Vec<Port>,
    /// Instantiated child components.
    pub components: Vec<InstantiatedComponent>,
    /// Connections between child components.
    pub connections: Vec<Connection>,
}

impl ComponentDefinitionFile {
    /// Creates a component definition after validating port, component, and connection IDs.
    ///
    /// # Errors
    ///
    /// Returns [`ValidationError`] when a port or child component ID is duplicated,
    /// or when a connection names an unknown child component.
    pub fn new(
        metadata: ComponentMetadata,
        parameter_object: ParameterObjectDefinition,
        variable_object: VariableObjectDefinition,
        ports: Vec<Port>,
        components: Vec<InstantiatedComponent>,
        connections: Vec<Connection>,
    ) -> Result<Self, ValidationError> {
        validate_port_ids(&ports)?;
        validate_components(&components, &connections)?;
        Ok(Self {
            metadata,
            parameter_object,
            variable_object,
            ports,
            components,
            connections,
        })
    }
}

#[cfg(test)]
mod tests {
    use datastore::prelude::{
        NumberDefinition, ParameterKey, ParameterObjectDefinition, ShareableString,
        VariableObjectDefinition,
    };

    use super::*;

    /// Creates a valid datastore parameter key for a test fixture.
    fn parameter_key(value: &str) -> ParameterKey {
        let Ok(key) = ParameterKey::new(ShareableString::from(value)) else {
            panic!("test parameter key must be valid");
        };
        key
    }

    #[test]
    fn component_definition_uses_datastore_objects() {
        let parameter_object = ParameterObjectDefinition::builder("Heater parameters")
            .with(
                parameter_key("p_target_temperature"),
                NumberDefinition::new_with_default("Target temperature", "22"),
            )
            .finish();

        let component = ComponentDefinitionFile::new(
            ComponentMetadata::new(
                1,
                "1.0".to_owned(),
                "1.0".to_owned(),
                1,
                "heater".to_owned(),
                120,
                60,
            ),
            parameter_object.clone(),
            VariableObjectDefinition::builder("Heater variables").finish(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );

        assert!(
            matches!(component, Ok(component) if component.parameter_object == parameter_object)
        );
    }
}
