//! Keys and associated traits.

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

/// Common utilities for keys.
pub(crate) mod common;
/// Keys for global objects.
pub mod global_key;
/// Keys for parameters.
pub mod parameter_key;
/// Keys for store objects.
pub mod store_key;
/// Keys for units.
pub mod unit_key;
/// Keys for variables.
pub mod variable_key;

pub use global_key::*;
pub use parameter_key::*;
pub use store_key::*;
pub use unit_key::*;
pub use variable_key::*;
