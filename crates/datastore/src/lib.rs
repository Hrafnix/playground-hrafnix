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
/// Keys and associated traits.
pub mod key;
/// Convenience re-exports of the most common types and macros.
pub mod prelude;
/// Traits used throughout the store.
pub mod traits;

use std::fmt::{Display, Formatter};

/// Error types for the store operations.
#[derive(Debug, Clone, PartialEq)]
pub enum StoreError {
    /// The provided key is empty.
    KeyEmpty,
    /// The key contains an invalid character.
    KeyInvalidCharacter(String),
    /// The key is missing the required prefix (e.g. `p_` for parameter keys, `v_` for variable keys).
    KeyInvalidPrefix(String),
    /// A key already exists.
    KeyConflict(String),
    /// The key is a reserved keyword.
    KeyReserved(String),
    /// The requested object was not found.
    ObjectNotFound,
    /// An object with the specified key already exists.
    ObjectKeyAlreadyExists,
    /// The requested parameter was not found.
    ParameterNotFound,
    /// The requested variable was not found.
    VariableNotFound,
    /// The proxy has expired or is no longer valid.
    ExpiredProxy,
    /// The key was not found in the map.
    KeyNotFound,
    /// The provided path is invalid.
    InvalidPath,
    /// The provided path segment is invalid.
    InvalidPathSegment(String),
    /// The requested index was not found.
    IndexNotFound,
    /// Undo operation is not available.
    UndoNotAvailable,
    /// Redo operation is not available.
    RedoNotAvailable,
    /// Failed to serialize or deserialize the store state.
    SerializationError(String),
    /// A schema mismatch occurred during update or conversion.
    SchemaMismatch(String),
    /// Nested containers are not supported in this context.
    NestedContainerNotSupported,
    /// The schema is missing.
    MissingSchema(String),
    /// Invalid Type: The type of the value does not match the expected type.
    InvalidType(String),
}

impl Display for StoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::KeyEmpty => write!(f, "Invalid key: Key cannot be empty"),
            StoreError::KeyInvalidCharacter(s) => write!(
                f,
                "Invalid key: '{s}'. Keys must only contain a-z, 0-9 and _"
            ),
            StoreError::KeyInvalidPrefix(s) => {
                write!(f, "Invalid key: '{s}'. Key is missing the required prefix")
            }
            StoreError::KeyConflict(s) => write!(f, "Key conflict: {s}"),
            StoreError::KeyReserved(s) => write!(f, "Key reserved: {s}"),
            StoreError::ObjectNotFound => write!(f, "Object not found"),
            StoreError::ObjectKeyAlreadyExists => write!(f, "Object key already exists"),
            StoreError::ParameterNotFound => write!(f, "Parameter not found"),
            StoreError::VariableNotFound => write!(f, "Variable not found"),
            StoreError::ExpiredProxy => write!(f, "Proxy is invalid"),
            StoreError::KeyNotFound => write!(f, "Key not found"),
            StoreError::InvalidPath => write!(f, "Invalid path"),
            StoreError::InvalidPathSegment(s) => write!(f, "Invalid path segment: {s}"),
            StoreError::IndexNotFound => write!(f, "Index not found"),
            StoreError::UndoNotAvailable => write!(f, "Undo not available"),
            StoreError::RedoNotAvailable => write!(f, "Redo not available"),
            StoreError::SerializationError(s) => write!(f, "Serialization error: {s}"),
            StoreError::SchemaMismatch(s) => write!(f, "Schema mismatch: {s}"),
            StoreError::NestedContainerNotSupported => {
                write!(f, "Nested containers are not supported in this context")
            }
            StoreError::MissingSchema(s) => write!(f, "Missing schema: {s}"),
            StoreError::InvalidType(s) => write!(f, "Invalid type: {s}"),
        }
    }
}

impl std::error::Error for StoreError {}
