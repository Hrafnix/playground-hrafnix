//! The `evaluation` module serves as the entry point for the expression engine, providing
//! the necessary structures to facilitate the evaluation of expressions. It
//! includes the `engine` and `expression` submodules, which contain the core logic for
//! processing and evaluating expressions based on the defined syntax and semantics.

/// The `engine` module contains the core evaluation engine, which is responsible for managing
/// the evaluation of expressions. It provides the necessary structures and methods to facilitate
/// the evaluation process, including handling global computed data and orchestrating the
/// evaluation of various types of expressions.
pub mod engine;
/// The `expression` module contains the core components for parsing and evaluating expressions
/// within the expression engine. It includes the lexer, parser, and evaluator, which work
/// together to process and compute the results of expressions based on the defined syntax
/// and semantics.
pub mod expression;
