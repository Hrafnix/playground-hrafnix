//! # Datastore
//!
//! A hierarchical, thread-safe, and observable data store with proxy-based access.
//!
//! ## Core Concepts
//!
//! - **Store**: The root container for all data objects. It manages thread safety and persistence.
//! - **Definitions**: Define the structure of your data (Objects, Structs, Maps, Tables, and Basic values).
//! - **Proxies**: Lightweight handles to data within the store. They provide a way to read and update data while maintaining sync with the store.
//! - **Shareable Strings**: Interned, thread-safe strings used throughout the store to reduce memory overhead and enable fast comparisons.
//!
//! ## Thread Safety and Invariants
//!
//! - **Thread Safety**: The `Store` is thread-safe (`Send` + `Sync`) and uses internal locking (`parking_lot::RwLock`).
//! - **Proxy Validity**: A proxy becomes "invalid" (expired) if its underlying data is removed from the store. Use `proxy.is_valid()` to check.
//! - **Cloning**: Cloning a `Store` or a `Proxy` creates a new handle to the *same* underlying data (shallow copy).
//! - **Change Tracking**: Use `has_changed()` on a proxy to check if the store has been updated since the proxy was last synced.
//! - **Updates**: Updates via proxies are pushed to the store. Other proxies must `pull()` to see these changes.
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
        clippy::arithmetic_side_effects
    )
)]

/// Data structure definitions.
pub mod definition;
/// Editable data implementation.
pub mod editable;
/// Frozen data implementation for efficient persistence and access.
pub mod frozen;
/// Convenience re-exports of the most common types and macros.
pub mod prelude;
/// Traits used throughout the store.
pub mod traits;
