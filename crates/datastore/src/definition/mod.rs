/// Definitions for basic data types (strings, numbers, etc.).
pub mod basic_definition;
/// Definitions for map-based data structures.
pub mod map_definition;
/// Definitions for object-based data structures.
pub mod object_definition;
/// Definitions for properties within objects or containers.
pub mod property_definition;
/// Definitions for struct-like data structures.
pub mod struct_definition;
/// Definitions for table-based data structures.
pub mod table_definition;

pub use basic_definition::*;
pub use map_definition::*;
pub use object_definition::*;
pub use property_definition::*;
pub use struct_definition::*;
pub use table_definition::*;
