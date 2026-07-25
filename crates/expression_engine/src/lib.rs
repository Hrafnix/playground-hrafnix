//! Expression engine crate.

use core::fmt;
use datastore::definition::{ChoiceDefinition, FileDefinition, NumberDefinition, StringDefinition};
use shareable_string::{ShareableString, SharedStringStore};

/// Processed data.
pub mod computed_data;
/// Evaluation engine.
pub mod evaluation;
/// Preprocessed data.
pub mod preprocessed_data;

pub use computed_data::*;
pub use evaluation::*;
pub use preprocessed_data::*;

/// A definition for one of the basic (non-composite) data types supported by the
/// expression engine.
#[derive(Debug, Clone, PartialEq)]
pub enum BasicDefinition {
    /// Holds a value from a fixed set of choices.
    Choice(ChoiceDefinition),
    /// Holds a file reference.
    File(FileDefinition),
    /// Holds numeric value.
    Number(NumberDefinition),
    /// Holds a string value.
    String(StringDefinition),
}

impl BasicDefinition {
    /// Returns a new `BasicDefinition` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        match self {
            BasicDefinition::Choice(choice) => BasicDefinition::Choice(choice.launder(store)),
            BasicDefinition::File(file) => BasicDefinition::File(file.launder(store)),
            BasicDefinition::Number(number) => BasicDefinition::Number(number.launder(store)),
            BasicDefinition::String(string) => BasicDefinition::String(string.launder(store)),
        }
    }
}

/// An enumeration of the different categories of expression errors.
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionCategory {
    /// An error that occurred during the lexing phase of expression processing.
    Lexer,
    /// An error that occurred during the parsing phase of expression processing.
    Parse,
    /// An error that occurred during the evaluation phase of expression processing.
    Evaluation,
}

/// An error produced while parsing or evaluating an expression.
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionError {
    /// The phase in which the error occurred (e.g. `"parse"` or `"evaluation"`).
    category: ExpressionCategory,
    /// The name of the expression, parameter, or function involved.
    /// A human-readable description of the error.
    message: ShareableString,
}

impl ExpressionError {
    /// Creates a new `ExpressionError`.
    pub fn new(category: ExpressionCategory, message: impl Into<ShareableString>) -> Self {
        Self {
            category,
            message: message.into(),
        }
    }

    /// Returns a new `ExpressionError` with the message laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            category: self.category.clone(),
            message: store.launder(&self.message),
        }
    }
}

impl fmt::Display for ExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}] {}", self.category, self.message)
    }
}

impl std::error::Error for ExpressionError {}
