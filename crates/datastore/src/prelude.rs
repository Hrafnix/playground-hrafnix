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
    BasicDefinition, BasicDefinitionType, ChoiceDefinition, FileDefinition, ItemDefinition,
    ItemDefinitionType, MapDefinition, ObjectDefinition, ObjectDefinitionBuilder, StructDefinition,
    StructItemDefinition, TableDefinition,
};

// Store and proxies
pub use crate::store::traits::TreeDisplay;

// Shareable strings
pub use shareable_string::{ShareableString, SharedStringStore, SharedStringTranslationMap};

// Static store
pub use crate::static_store::{
    ItemParameter, StaticBasic, StaticMap, StaticObject, StaticStore, StaticStruct,
    StaticStructItem, StaticTable,
};
