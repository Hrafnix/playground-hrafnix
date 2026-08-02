//! This crate provides functionality for managing shareable strings and their translations.\
//! It includes modules for string interning, a shareable string type, and a translation map for\
//! managing translations of shareable strings.

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
        clippy::arithmetic_side_effects
    )
)]

/// Internal string interning store.
pub mod store;
/// The `ShareableString` type.
pub mod string;
/// A map for storing translations of `ShareableString`s.
pub mod translation_map;

pub use store::*;
pub use string::*;
pub use translation_map::*;
