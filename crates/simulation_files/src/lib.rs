//! Strongly typed domain models for simulation model and component files.
//!
//! This crate mirrors the structures documented by the simulation XML schemas
//! and provides XML serialization and parsing.

/// Component definition file types.
pub mod component;
/// Model file types.
pub mod model;
/// Convenience re-exports for common simulation file types.
pub mod prelude;
/// Project manifests and filesystem layout support.
pub mod project;
/// Shared simulation file domain types.
pub mod types;
/// XML serialization and deserialization for simulation files.
pub mod xml;

pub use component::*;
pub use datastore::prelude::{
    ParameterObjectDefinition, ParameterObjectFrozen, VariableObjectDefinition,
    VariableObjectFrozen,
};
pub use model::*;
pub use project::*;
pub use types::*;
pub use xml::*;
