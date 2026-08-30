//! Domain models for calculator components and canvases.

/// Registry of built-in component definitions.
pub mod built_in_registry;
/// Canvas dimensions and related types.
pub mod canvas;
/// Active component instances.
pub mod component;
/// Component definitions.
pub mod definitions;
/// Port types.
pub mod ports;
/// Quarter-turn rotation values.
pub mod rotation;

pub use canvas::*;
pub use component::*;
pub use definitions::*;
pub use ports::*;
pub use rotation::*;
