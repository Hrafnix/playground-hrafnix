//! Convenience re-exports for common types and macros.
//!
//! Using the prelude allows you to quickly import everything you need:
//!
//! ```rust
//! use datastore::prelude::*;
//! ```

// Macros
pub use crate::{
    boolean_compile_time, choice_compile_time, choice_item_compile_time, file_compile_time,
    folder_compile_time, global_object_compile_time, integer_compile_time, item_compile_time,
    map_compile_time, map_item_compile_time, number_compile_time, number_with_units_compile_time,
    parameter_object_compile_time, separator_compile_time, string_compile_time, tab_compile_time,
    table_compile_time, table_with_units_compile_time, unit_compile_time,
    variable_object_compile_time,
};
pub use keys::{global_key, parameter_key, store_key, unit_key, variable_key};

// Core types
pub use keys::{
    global_key::{ConstGlobalKey, GlobalKey},
    parameter_key::{ConstParameterKey, ParameterKey},
    store_key::{ConstStoreKey, StoreKey},
    unit_key::{ConstUnitKey, UnitKey},
    variable_key::{ConstVariableKey, VariableKey},
};
pub use message::message::{Message, MessageCategory};

// Compile time
pub use crate::compile_time::{
    BooleanCompileTime, ChoiceCompileTime, ChoiceItemCompileTime, FileCompileTime,
    FolderCompileTime, GlobalObjectCompileTime, IntegerCompileTime, ItemCompileTime,
    MapCompileTime, MapItemCompileTime, NumberCompileTime, NumberWithUnitsCompileTime,
    ParameterObjectCompileTime, SeparatorCompileTime, StringCompileTime, TableCompileTime,
    TableWithUnitsCompileTime, UnitCompileTime, VariableObjectCompileTime,
};

// Definitions
pub use crate::definition::{
    BooleanDefinition, ChoiceDefinition, ChoiceItemDefinition, FileDefinition,
    GlobalObjectDefinition, GlobalObjectDefinitionBuilder, IntegerConstraint,
    IntegerConstraintEnum, IntegerDefinition, ItemDefinitionType, MapDefinition, MapItemDefinition,
    NumberConstraint, NumberConstraintEnum, NumberDefinition, NumberWithUnitsDefinition,
    ParameterObjectDefinition, ParameterObjectDefinitionBuilder, StringDefinition, TableDefinition,
    TableWithUnitsDefinition, UnitDefinition, VariableObjectDefinition,
    VariableObjectDefinitionBuilder,
};

// Shareable strings
pub use shareable_string::{ShareableString, SharedStringStore, SharedStringTranslationMap};

// Frozen data
pub use crate::frozen::{
    BooleanFrozen, ChoiceFrozen, FileFrozen, GlobalObjectFrozen, IntegerFrozen, ItemFrozen,
    MapEntryFrozen, MapFrozen, MapItemFrozen, NumberFrozen, NumberWithUnitsFrozen,
    ParameterObjectFrozen, StringFrozen, TableFrozen, TableWithUnitsFrozen, UnitFrozen,
    VariableObjectFrozen,
};

// Editable data
pub use crate::editable::{
    BooleanEditable, ChoiceEditable, FileEditable, GlobalObjectEditable, IntegerEditable,
    ItemEditable, MapEditable, MapEntryEditable, MapItemEditable, NumberEditable,
    NumberWithUnitsEditable, ParameterObjectEditable, StringEditable, TableEditable,
    TableWithUnitsEditable, UnitEditable, VariableObjectEditable, editable_set_map_value,
    editable_set_value,
};
