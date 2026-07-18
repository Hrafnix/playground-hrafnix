/// Definitions for basic data types (strings, numbers, etc.).
pub mod definition_basic;
/// Definitions for parameter within objects or containers.
pub mod definition_item;
/// Definitions for map-based data structures.
pub mod definition_map;
/// Definitions for object-based data structures.
pub mod definition_object_general;
/// Definitions for object parameter configurations within data structures.
pub mod definition_object_parameter;
/// Definitions for object variable configurations within data structures.
pub mod definition_object_variable;
/// Definitions for struct-like data structures.
pub mod definition_struct;
/// Definitions for table-based data structures.
pub mod definition_table;

pub use definition_basic::*;
pub use definition_item::*;
pub use definition_map::*;
pub use definition_object_general::*;
pub use definition_object_parameter::*;
pub use definition_object_variable::*;
pub use definition_struct::*;
pub use definition_table::*;
