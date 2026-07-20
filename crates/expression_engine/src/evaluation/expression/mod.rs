//! This module contains the core components for evaluating expressions, including the lexer, parser, and evaluator.

/// The `evaluator` computes the final result based on the AST.
pub mod evaluator;
/// The `lexer` is responsible for tokenizing the input expression.
pub mod lexer;
/// The `precedence_parser` implements a parser that respects operator precedence and associativity.
pub mod parser;
/// The 'translator' implements a conversion from the AST to a more efficient representation for evaluation, optimizing the evaluation process.
pub mod translator;
