/// The `lexer` is responsible for tokenizing the input expression.
pub mod lexer;
/// The `precedence_parser` implements a parser that respects operator precedence and associativity.
pub mod parser;
/// The `span` module provides structures and methods for managing ranges of indices, which can be used to represent spans of text or other sequential data.
pub(crate) mod span;
/// The 'translator' implements a conversion from the AST to a more efficient representation for evaluation, optimizing the evaluation process.
pub mod translator;
