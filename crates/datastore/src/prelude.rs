//! Convenience re-exports for common types and macros.
//!
//! Using the prelude allows you to quickly import everything you need:
//!
//! ```rust
//! use datastore::prelude::*;
//! ```

// Macros
pub use crate::{parameter_key, path, store_key, variable_key};

// Core types
pub use crate::StoreError;
pub use crate::key::{
    ConstParameterKey, ConstStoreKey, ConstVariableKey, ParameterKey, StoreKey, VariableKey,
};
pub use crate::path::StorePath;

// Definitions
pub use crate::definition::{
    ChoiceDefinition, ChoiceItemDefinition, FileDefinition, ItemDefinition, ItemDefinitionType,
    MapDefinition, NumberConstraint, NumberDefinition, ObjectDefinition, ObjectDefinitionBuilder,
    ParameterObjectDefinition, ParameterObjectDefinitionBuilder, StringDefinition,
    StructDefinition, StructItemDefinition, TableDefinition, VariableObjectDefinition,
    VariableObjectDefinitionBuilder,
};

// Shareable strings
pub use shareable_string::{ShareableString, SharedStringStore, SharedStringTranslationMap};

// Frozen data
pub use crate::frozen::{
    ChoiceFrozen, FileFrozen, ItemFrozen, MapFrozen, NumberFrozen, ObjectFrozen,
    ParameterObjectFrozen, StringFrozen, StructFrozen, StructItemFrozen, TableFrozen,
    VariableObjectFrozen,
};
