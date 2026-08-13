//! Integration test root module.
//!
//! Declares submodules containing tests organized by topic.
//! Each submodule focuses on a specific area of the crate's functionality:
//!
//! - [`definition`] – tests for datastore definition types and their builders, ensuring
//!   they correctly represent the intended structures and parameters.
//!
//! - [`store`] – tests for the dynamic store, covering proxy access, error handling,
//!   object copying, data recovery, and JSON serialization / deserialization.

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

mod definition;
mod editable;
mod frozen;
