//! Integration test root module.
//!
//! Declares submodules containing tests organized by topic.
//! Each submodule focuses on a specific area of the crate's functionality:
//!
//! - [`compile_time`] – tests for the `const_*` macros and their conversion into runtime
//!   definitions.
//!
//! - [`definition`] – tests for datastore definition types and their builders, ensuring
//!   they correctly represent the intended structures and parameters.
//!
//! - [`frozen`] – tests for immutable snapshots: construction from definitions, hashing,
//!   and merging values into parameter objects.
//!
//! - [`editable`] – tests for the mutable counterpart: thaw / edit / freeze round trips and
//!   keyed updates via `editable_set_value`.

// Integration tests favor clarity and brevity over the strictness we require of library
// code: panicking helpers (`unwrap`/`expect`/indexing/`panic!`) and approximate float
// comparisons are idiomatic and expected in tests.
#![allow(
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
)]

mod compile_time;
mod definition;
mod editable;
mod frozen;
