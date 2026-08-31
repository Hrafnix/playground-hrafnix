//! Domain models for calculator components and canvases.

/// Registry of built-in component definitions.
pub mod built_in_registry;
/// Canvas dimensions and related types.
pub mod canvas;
/// Active component instances.
pub mod component;
/// Evaluated component runtime types.
pub mod computed;
/// Component definitions.
pub mod definitions;
/// Port types.
pub mod ports;
/// Quarter-turn rotation values.
pub mod rotation;
/// Deterministic fixed-step signal simulation.
pub mod simulation;
/// Deterministic one-dimensional spring-mass mechanics.
pub mod translational;

pub use canvas::*;
pub use component::*;
pub use computed::*;
pub use definitions::*;
pub use ports::*;
pub use rotation::*;
pub use simulation::*;
pub use translational::*;
