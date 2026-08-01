//! This crate provides functionality for managing shareable strings and their translations.\
//! It includes modules for string interning, a shareable string type, and a translation map for\
//! managing translations of shareable strings.

/// Internal string interning store.
pub mod store;
/// The `ShareableString` type.
pub mod string;
/// A map for storing translations of `ShareableString`s.
pub mod translation_map;

pub use store::*;
pub use string::*;
pub use translation_map::*;
