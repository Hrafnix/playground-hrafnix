//! Convenience re-exports for common types and macros.
//!
//! Using the prelude allows you to quickly import everything you need:
//!
//! ```rust
//! use datastore::prelude::*;
//! ```

// Macros
pub use crate::{path, store_key};

// Core types
pub use crate::StoreError;
pub use crate::key::{ConstStoreKey, StoreKey};
pub use crate::path::StorePath;

// Definitions
pub use crate::definition::{
    BasicDefinition, BasicDefinitionType, ChoiceDefinition, FileDefinition, MapDefinition,
    ObjectDefinition, ObjectDefinitionBuilder, PropertyDefinition, PropertyDefinitionType,
    StructDefinition, StructItemDefinition, TableDefinition,
};

// Store and proxies
pub use crate::store::traits::TreeDisplay;
pub use crate::store::{BasicProxy, ContainerProxy, ObjectProxy, Store, TableProxy, TreePrint};

// Shareable strings
pub use shareable_string::{ShareableString, SharedStringStore, SharedStringTranslationMap};

// Static store
pub use crate::static_store::{
    StaticBasic, StaticMap, StaticObject, StaticProperty, StaticStore, StaticStruct,
    StaticStructItem, StaticTable,
};
