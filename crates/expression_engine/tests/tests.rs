//! Tests for expression engine.

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
    clippy::arithmetic_side_effects
)]

mod evaluation;
mod input;
