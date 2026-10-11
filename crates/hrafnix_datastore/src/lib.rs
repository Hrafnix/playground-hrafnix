//! # Datastore
//!
//! Structured, hashable storage for *unevaluated values* and the definitions that describe them.
//!
//! ## Layers
//!
//! - **`definition`** – The shape of the data: objects (global / parameter / variable), maps,
//!   tables, and leaf items (boolean, integer, number, string, choice, file, folder, unit, …)
//!   together with descriptions, defaults, and constraints.
//! - **`compile_time`** – `const`-constructible mirrors of the definitions, built with the
//!   `const_*` macros, so a schema can be declared statically and converted to definitions at
//!   runtime.
//! - **`frozen`** – Immutable snapshot of definitions plus their current unevaluated values,
//!   with a pre-computed BLAKE3 hash per node for cheap diffing.
//! - **`editable`** – Mutable counterpart to `frozen`. `thaw()` a frozen tree, edit it, then
//!   `freeze()` it back.
//!
//! ## Values are unevaluated
//!
//! Every leaf stores its value as a [`ShareableString`](hrafnix_shareable_string::ShareableString)
//! holding an unevaluated expression, not a typed result. The datastore deliberately
//! knows nothing about `i64`/`f64`/`bool` and never checks constraints:
//!
//! - Definitions and frozen data are intended to be read from and written to a file and may
//!   contain malformed or out-of-range values. The datastore must load them without failing.
//! - Interpretation, type checking, and constraint enforcement happen exclusively in the
//!   expression engine, which consumes `frozen` data as input.
//!
//! ## Shareable strings
//!
//! All text is stored as interned `ShareableString`s. `launder(&SharedStringStore)` re-interns a
//! tree into a given store so independently loaded data can share storage and compare quickly.
//!

// Test code favors clarity and brevity over the strictness we require of library code:
// panicking helpers (`unwrap`/`expect`/indexing/`panic!`) and approximate float comparisons
// are idiomatic and expected in tests.
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::indexing_slicing,
        clippy::panic,
        clippy::float_cmp,
        clippy::as_conversions,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::unreadable_literal,
        clippy::unnecessary_wraps,
        clippy::similar_names,
        clippy::arithmetic_side_effects,
        clippy::wildcard_enum_match_arm
    )
)]

/// Compile time implements for compile-time checks and validations of definitions and data structures.
pub mod compile_time;
/// Data structure definitions.
pub mod definition;
/// Editable data implementation.
pub mod editable;
/// Immutable, hashed snapshots of definitions and their unevaluated values.
pub mod frozen;
/// Convenience re-exports of the most common types and macros.
pub mod prelude;
/// Traits used throughout the datastore.
pub mod traits;
