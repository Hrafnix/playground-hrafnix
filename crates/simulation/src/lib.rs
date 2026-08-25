//! Headless contracts and runtime services for component-based simulation.
//!
//! # Ownership boundaries
//!
//! - `datastore` owns generic parameter definitions and editable/frozen values.
//! - `expression_engine` owns parsing and configuration-time expression evaluation.
//! - `units` owns unit identities and conversion rules.
//! - `message` owns diagnostics produced by those generic lower layers.
//! - This crate owns simulation identities, runtime values, component graphs,
//!   scheduling semantics, simulation diagnostics, and results.
//! - Application crates own documents-in-use, commands, UI state, and platform I/O.
//!
//! The simulation crate adapts lower-layer types at explicit boundaries and does
//! not introduce simulation or UI concepts into the generic foundation crates.

#![cfg_attr(
    test,
    allow(
        clippy::arithmetic_side_effects,
        clippy::float_cmp,
        clippy::indexing_slicing,
        clippy::unwrap_used
    )
)]

pub mod benchmark_models;
/// Executable signal/control primitive definitions and behaviors.
pub mod builtins;
pub mod catalog_docs;
/// Component metadata and public interface contracts.
pub mod component;
/// Headless execution of persisted custom-component test cases.
pub mod custom_tests;
/// Simulation-owned diagnostics and lower-layer message adaptation.
pub mod diagnostic;
/// Persisted model and custom-component document schemas.
pub mod document;
/// Deterministic resolved-hierarchy to executable-graph adaptation.
pub mod flatten;
/// Stable simulation identity types and injectable ID generation.
pub mod identity;
/// Datastore parameter-definition adaptation.
pub mod parameter;
/// Version-aware native JSON loading and saving.
pub mod persistence;
/// Physical-domain equations, causality, initialization, and solver contracts.
pub mod physical;
/// Built-in component definition registry.
pub mod registry;
/// Custom-component dependency resolution and immutable graph expansion.
pub mod resolve;
/// Simulation run metadata and sampled signal results.
pub mod results;
/// Synchronous fixed-step simulation runtime.
pub mod runtime;
/// Deterministic direct-feedthrough execution-plan construction.
pub mod schedule;
/// Internal configuration-time compiler for signal expressions.
mod signal_expression;
/// Fixed-step endpoint and state-update semantics.
pub mod timing;
/// Executable one-dimensional translational mechanics slice.
pub mod translational;
/// Recursive resolved-graph and model-settings validation.
pub mod validation;
/// Runtime values and expression-result adaptation.
pub mod value;
