//! This module defines error types for the store operations.

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
