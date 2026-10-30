/// Model file data structures.
pub mod data;
/// Model file readers.
pub mod reader;
/// Model file writers.
pub mod writer;

/// The key of the top-level entry that holds every model file's contents.
const ROOT: &str = "model";
