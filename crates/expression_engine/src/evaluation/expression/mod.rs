//! This module contains the core components for evaluating expressions, including the lexer, parser, and evaluator.

/// The `expression` module is responsible for parsing and evaluating expressions within the expression engine,
/// providing the necessary structures and methods to facilitate the evaluation process.
pub mod ast;
/// The `evaluator` computes the final result based on the AST.
pub mod evaluator;
/// The `function_definition` module contains the definition of functions that can be invoked
/// from within expressions. It provides the necessary structures and methods to define and
/// manage functions, including their names, descriptions, and the logic for evaluating them
/// based on the provided arguments.
pub mod function_definition;
/// The `function_definitions_default` module provides a set of default function definitions that
/// can be used within the expression engine.
pub(crate) mod function_definitions_default;
