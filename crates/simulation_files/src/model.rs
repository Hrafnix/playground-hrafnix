//! Types for standalone simulation model files.

use datastore::prelude::{ParameterObjectFrozen, VariableObjectFrozen};

use crate::{Connection, InstantiatedComponent, ValidationError, validate_components};

/// Metadata from a model file's root element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelMetadata {
    /// The model file format version.
    pub file_version: u16,
    /// The minimum compatible simulation engine version.
    pub minimum_engine_version: String,
    /// The simulation engine version that built the model.
    pub built_engine_version: String,
    /// The model name.
    pub name: String,
}

impl ModelMetadata {
    /// Creates model metadata.
    #[must_use]
    pub const fn new(
        file_version: u16,
        minimum_engine_version: String,
        built_engine_version: String,
        name: String,
    ) -> Self {
        Self {
            file_version,
            minimum_engine_version,
            built_engine_version,
            name,
        }
    }
}

/// One setting recorded by a model file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelSetting {
    /// The setting key.
    pub key: String,
    /// The setting value.
    pub value: String,
}

impl ModelSetting {
    /// Creates a model setting.
    #[must_use]
    pub const fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

/// A complete standalone simulation model file.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelFile {
    /// Root file metadata.
    pub metadata: ModelMetadata,
    /// Model settings.
    pub settings: Vec<ModelSetting>,
    /// Frozen model parameter object values.
    pub parameter_object: ParameterObjectFrozen,
    /// Frozen model variable object values.
    pub variable_object: VariableObjectFrozen,
    /// Instantiated model components.
    pub components: Vec<InstantiatedComponent>,
    /// Connections between model components.
    pub connections: Vec<Connection>,
}

impl ModelFile {
    /// Creates a model file after validating component IDs and connection targets.
    ///
    /// # Errors
    ///
    /// Returns [`ValidationError`] when a component ID is duplicated or when a
    /// connection names an unknown component.
    pub fn new(
        metadata: ModelMetadata,
        settings: Vec<ModelSetting>,
        parameter_object: ParameterObjectFrozen,
        variable_object: VariableObjectFrozen,
        components: Vec<InstantiatedComponent>,
        connections: Vec<Connection>,
    ) -> Result<Self, ValidationError> {
        validate_components(&components, &connections)?;
        Ok(Self {
            metadata,
            settings,
            parameter_object,
            variable_object,
            components,
            connections,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        AbsolutePoint, BuiltInComponent, ComponentInstance, ComponentRectangle, Connection,
        ConnectionEndpoints, ConnectionLine, ConnectionStyle, Identifier, InstantiatedComponent,
    };
    use datastore::prelude::{
        ParameterObjectDefinition, ParameterObjectFrozen, VariableObjectDefinition,
        VariableObjectFrozen,
    };

    use super::*;

    #[test]
    fn model_rejects_connections_to_unknown_components() {
        let Ok(clock) = Identifier::new("clock") else {
            panic!("test identifier must be valid");
        };
        let Ok(time) = Identifier::new("time") else {
            panic!("test identifier must be valid");
        };
        let Ok(heater) = Identifier::new("heater") else {
            panic!("test identifier must be valid");
        };
        let Ok(input) = Identifier::new("input") else {
            panic!("test identifier must be valid");
        };
        let Ok(instance) = ComponentInstance::new(
            clock.clone(),
            ComponentRectangle::new(0, 0, 100, 50),
            None,
            ParameterObjectFrozen::new(ParameterObjectDefinition::builder("Parameters").finish()),
            VariableObjectFrozen::new(VariableObjectDefinition::builder("Variables").finish()),
            Vec::new(),
        ) else {
            panic!("test component instance must be valid");
        };
        let Ok(line) = ConnectionLine::new(vec![AbsolutePoint::new(10, 10)]) else {
            panic!("test connection line must be valid");
        };
        let component =
            InstantiatedComponent::BuiltIn(BuiltInComponent::new(instance, "clock".to_owned(), 1));
        let connection = Connection::new(
            ConnectionEndpoints::new(clock, time, heater, input),
            ConnectionStyle::Solid,
            line,
        );

        assert!(matches!(
            ModelFile::new(
                ModelMetadata::new(1, "1.0".to_owned(), "1.0".to_owned(), "test".to_owned()),
                Vec::new(),
                ParameterObjectFrozen::new(
                    ParameterObjectDefinition::builder("Parameters").finish()
                ),
                VariableObjectFrozen::new(VariableObjectDefinition::builder("Variables").finish()),
                vec![component],
                vec![connection],
            ),
            Err(ValidationError::UnknownConnectionComponent { .. })
        ));
    }
}
