//! Convenience re-exports for common simulation file types.
//!
//! ```rust
//! use simulation_files::prelude::*;
//! ```

pub use crate::{
    AbsolutePoint, BuiltInComponent, ComponentDefinitionFile, ComponentInstance, ComponentMetadata,
    ComponentRectangle, Connection, ConnectionEndpoints, ConnectionLine, ConnectionStyle,
    FileBackedComponent, Identifier, IdentifierCollection, InstantiatedComponent, ModelFile,
    ModelMetadata, ModelSetting, NormalizedCoordinate, ParameterObjectDefinition,
    ParameterObjectFrozen, Port, PortLocation, PortType, Project, ProjectError, ProjectManifest,
    ProjectMetadata, SourceBlake3Digest, ValidationError, VariableObjectDefinition,
    VariableObjectFrozen, XmlError, component_definition_from_xml_str,
    component_definition_to_xml_string, model_from_xml_str, model_to_xml_string,
};
