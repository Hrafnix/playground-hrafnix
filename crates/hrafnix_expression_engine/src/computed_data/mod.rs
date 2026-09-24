/// Represents a computed object that can be a float, string, or table.
pub mod computed_item;
/// Represents computed data for an object, mapping field names to their
/// corresponding computed data items.
pub mod computed_object;

pub use computed_item::*;
pub use computed_object::*;
