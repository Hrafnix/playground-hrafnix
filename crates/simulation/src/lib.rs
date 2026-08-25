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

/// Simulation-owned diagnostics and lower-layer message adaptation.
pub mod diagnostic;
/// Stable simulation identity types and injectable ID generation.
pub mod identity;
/// Datastore parameter-definition adaptation.
pub mod parameter;
/// Fixed-step endpoint and state-update semantics.
pub mod timing;
/// Runtime values and expression-result adaptation.
pub mod value;
