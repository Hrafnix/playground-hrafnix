/// Component file data structures.
pub mod data;
/// Component file readers.
pub mod reader;
/// Component file writers.
pub mod writer;

/// The key of the top-level entry that holds every component file's contents.
const ROOT: &str = "component";
