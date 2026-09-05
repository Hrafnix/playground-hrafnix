//! Convenience re-exports for common types and macros.
//!
//! Using the prelude allows you to quickly import everything you need:
//!
//! ```rust
//! use datastore::prelude::*;
//! ```

// Macros
pub use crate::{
    const_boolean, const_choice, const_choice_item, const_file, const_folder, const_global_object,
    const_integer, const_item, const_map, const_map_item, const_number, const_number_with_units,
    const_parameter_object, const_separator, const_string, const_tab, const_table,
    const_table_with_units, const_unit, const_variable_object,
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
    IntegerConstraintEnum, IntegerDefinition, ItemDefinitionType, MapDefinition, MapEntryDefault,
    MapItemDefault, MapItemDefinition, NumberConstraint, NumberConstraintEnum, NumberDefinition,
    NumberWithUnitsDefinition, ParameterObjectDefinition, ParameterObjectDefinitionBuilder,
    StringDefinition, TableDefinition, TableWithUnitsDefinition, UnitDefinition,
    VariableObjectDefinition, VariableObjectDefinitionBuilder,
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
