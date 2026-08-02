use crate::expression::parser::{Parser, ParserToken};
use crate::expression::span::{Span, SpanSet};
use crate::{ExpressionCategory, ExpressionError};
use shareable_string::ShareableString;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Integer(value) => write!(f, "{value}"),
            Literal::Float(value) => write!(f, "{value}"),
            Literal::String(value) => write!(f, "{value}"),
            Literal::Boolean(value) => write!(f, "{value}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Operators {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Power,
    Negate,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
    Not,
}

impl fmt::Display for Operators {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Operators::Add => "+",
            Operators::Subtract | Operators::Negate => "-",
            Operators::Multiply => "*",
            Operators::Divide => "/",
            Operators::Modulus => "%",
            Operators::Power => "^",
            Operators::Equal => "==",
            Operators::NotEqual => "!=",
            Operators::LessThan => "<",
            Operators::LessThanOrEqual => "<=",
            Operators::GreaterThan => ">",
            Operators::GreaterThanOrEqual => ">=",
            Operators::And => "&&",
            Operators::Or => "||",
            Operators::Not => "!",
        };
        write!(f, "{symbol}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Expression {
    Literal(Span, Literal),
    BinaryOperation {
        span: Span,
        operator_span: Span,
        left: Box<Expression>,
        operator: Operators,
        right: Box<Expression>,
    },
    UnaryOperation {
        span: Span,
        operator: Operators,
        operand: Box<Expression>,
    },
    FunctionCall {
        span: Span,
        name: String,
        arguments: Vec<Expression>,
    },
    Index {
        span: Span,
        name: String,
        index: Vec<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(_, literal) => write!(f, "{literal}"),
            Expression::BinaryOperation {
                span: _,
                operator_span: _,
                left,
                operator,
                right,
            } => write!(f, "({left} {operator} {right})"),
            Expression::UnaryOperation {
                span: _,
                operator,
                operand,
            } => {
                write!(f, "({operator}{operand})")
            }
            Expression::FunctionCall {
                span: _,
                name,
                arguments,
            } => {
                let args = arguments
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{name}({args})")
            }
            Expression::Index {
                span: _,
                name,
                index,
            } => {
                write!(f, "{name}")?;
                for idx in index {
                    write!(f, "[{idx}]")?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct Translator {
    expression: Expression,
    source: ShareableString,
}

impl Translator {
    pub(crate) fn new(expression: Expression, source: ShareableString) -> Self {
        Self { expression, source }
    }

    pub(crate) fn expression(&self) -> &Expression {
        &self.expression
    }

    pub(crate) fn source(&self) -> &ShareableString {
        &self.source
    }
}

/// Returns the span associated with the given expression.
pub(crate) fn expression_span(expression: &Expression) -> Span {
    match expression {
        Expression::Literal(span, _)
        | Expression::BinaryOperation { span, .. }
        | Expression::UnaryOperation { span, .. }
        | Expression::FunctionCall { span, .. }
        | Expression::Index { span, .. } => *span,
    }
}

/// Translates a binary `ParserToken::Operator` into a `BinaryOperation` expression.
fn translate_binary(
    span: Span,
    operands: &[ParserToken],
    operator: Operators,
    source: &ShareableString,
) -> Result<Expression, ExpressionError> {
    let left = translate_token(
        operands.first().cloned().ok_or_else(|| {
            ExpressionError::new_complex(
                ExpressionCategory::Parse,
                "Binary operator is missing its left operand.".to_string(),
                source.clone(),
                SpanSet::from_span(span),
            )
        })?,
        source,
    )?;
    let right = translate_token(
        operands.get(1).cloned().ok_or_else(|| {
            ExpressionError::new_complex(
                ExpressionCategory::Parse,
                "Binary operator is missing its right operand.".to_string(),
                source.clone(),
                SpanSet::from_span(span),
            )
        })?,
        source,
    )?;
    let operator_span = span;
    let combined_span = span
        .join(&expression_span(&left))
        .join(&expression_span(&right));

    Ok(Expression::BinaryOperation {
        span: combined_span,
        operator_span,
        left: Box::new(left),
        operator,
        right: Box::new(right),
    })
}

/// Translates a unary `ParserToken::Operator` into a `UnaryOperation` expression.
fn translate_unary(
    span: Span,
    operands: &[ParserToken],
    operator: Operators,
    source: &ShareableString,
) -> Result<Expression, ExpressionError> {
    let operand = translate_token(
        operands.first().cloned().ok_or_else(|| {
            ExpressionError::new_complex(
                ExpressionCategory::Parse,
                "Unary operator is missing its operand.".to_string(),
                source.clone(),
                SpanSet::from_span(span),
            )
        })?,
        source,
    )?;
    let combined_span = span.join(&expression_span(&operand));

    Ok(Expression::UnaryOperation {
        span: combined_span,
        operator,
        operand: Box::new(operand),
    })
}

/// Translates a `ParserToken::Operator("[", ...)` into an `Index` expression.
///
/// When the collection being indexed is itself an `Index` expression (i.e., this is a chained
/// index such as `arr[0][1]`), the new index is appended to the existing `Index`'s vector of
/// indices rather than wrapping it in another `Index` expression.
fn translate_index(
    span: Span,
    operands: &[ParserToken],
    source: &ShareableString,
) -> Result<Expression, ExpressionError> {
    let target = translate_token(
        operands.first().cloned().ok_or_else(|| {
            ExpressionError::new_complex(
                ExpressionCategory::Parse,
                "Index operator is missing its target.".to_string(),
                source.clone(),
                SpanSet::from_span(span),
            )
        })?,
        source,
    )?;
    let new_index = translate_token(
        operands.get(1).cloned().ok_or_else(|| {
            ExpressionError::new_complex(
                ExpressionCategory::Parse,
                "Index operator is missing its index.".to_string(),
                source.clone(),
                SpanSet::from_span(span),
            )
        })?,
        source,
    )?;
    let combined_span = span
        .join(&expression_span(&target))
        .join(&expression_span(&new_index));

    match target {
        Expression::Index {
            span: _,
            name,
            mut index,
        } => {
            index.push(new_index);
            Ok(Expression::Index {
                span: combined_span,
                name,
                index,
            })
        }
        other => Ok(Expression::Index {
            span: combined_span,
            name: other.to_string(),
            index: vec![new_index],
        }),
    }
}

/// Translates a `ParserToken::Operator` whose head is a function name (rather than a known
/// operator symbol) into a `FunctionCall` expression.
fn translate_call(
    span: Span,
    name: String,
    arguments: Vec<ParserToken>,
    source: &ShareableString,
) -> Result<Expression, ExpressionError> {
    let arguments = arguments
        .into_iter()
        .map(|argument| translate_token(argument, source))
        .collect::<Result<Vec<_>, _>>()?;
    let combined_span = arguments
        .iter()
        .fold(span, |acc, argument| acc.join(&expression_span(argument)));
    // Extend the span by one to account for the closing `)`, which isn't captured by any
    // operand's span but is still part of the call's textual representation.
    let combined_span = combined_span.join(&Span::new(combined_span.end(), 1));

    Ok(Expression::FunctionCall {
        span: combined_span,
        name,
        arguments,
    })
}

/// Returns whether `name` looks like a function/variable name (i.e., what the lexer would have
/// produced as an `Atom`), as opposed to an operator symbol such as `+` or `!`.
fn is_function_name(name: &str) -> bool {
    name.chars()
        .next()
        .is_some_and(|c| c.is_alphanumeric() || c == '_')
}

/// Returns whether `value` looks like a numeric literal (i.e., what the lexer would have
/// produced as an `Atom` starting with a digit or a `.`), as opposed to a variable/function
/// name.
fn is_numeric_literal(value: &str) -> bool {
    value
        .chars()
        .next()
        .is_some_and(|c| c.is_numeric() || c == '.')
}

/// Translates a `ParserToken::Atom` into either a numeric `Literal` expression (when the atom
/// looks like a number) or a `Variable` expression (otherwise).
fn translate_atom(span: Span, value: String) -> Result<Expression, ExpressionError> {
    if !is_numeric_literal(&value) {
        if let Ok(boolean) = value.parse::<bool>() {
            return Ok(Expression::Literal(span, Literal::Boolean(boolean)));
        }

        return Ok(Expression::Literal(span, Literal::String(value)));
    }

    if let Ok(integer) = value.parse::<i64>() {
        return Ok(Expression::Literal(span, Literal::Integer(integer)));
    }

    if let Ok(float) = value.parse::<f64>() {
        return Ok(Expression::Literal(span, Literal::Float(float)));
    }

    Err(ExpressionError::new(
        ExpressionCategory::Parse,
        format!("Invalid numeric literal: {value}"),
    ))
}

fn translate_token(
    parser_token: ParserToken,
    source: &ShareableString,
) -> Result<Expression, ExpressionError> {
    match parser_token {
        ParserToken::Atom(span, value) => translate_atom(span, value),
        ParserToken::Operator(span, op, operands) => match (op.as_str(), operands.len()) {
            ("+", 1) => translate_token(
                operands.first().cloned().ok_or_else(|| {
                    ExpressionError::new_complex(
                        ExpressionCategory::Parse,
                        "Unary '+' operator is missing its operand.".to_string(),
                        source.clone(),
                        SpanSet::from_span(span),
                    )
                })?,
                source,
            ),
            ("-", 1) => translate_unary(span, &operands, Operators::Negate, source),
            ("!", 1) => translate_unary(span, &operands, Operators::Not, source),
            ("+", 2) => translate_binary(span, &operands, Operators::Add, source),
            ("-", 2) => translate_binary(span, &operands, Operators::Subtract, source),
            ("*", 2) => translate_binary(span, &operands, Operators::Multiply, source),
            ("/", 2) => translate_binary(span, &operands, Operators::Divide, source),
            ("%", 2) => translate_binary(span, &operands, Operators::Modulus, source),
            ("^", 2) => translate_binary(span, &operands, Operators::Power, source),
            ("==", 2) => translate_binary(span, &operands, Operators::Equal, source),
            ("!=", 2) => translate_binary(span, &operands, Operators::NotEqual, source),
            ("<", 2) => translate_binary(span, &operands, Operators::LessThan, source),
            ("<=", 2) => translate_binary(span, &operands, Operators::LessThanOrEqual, source),
            (">", 2) => translate_binary(span, &operands, Operators::GreaterThan, source),
            (">=", 2) => translate_binary(span, &operands, Operators::GreaterThanOrEqual, source),
            ("&&", 2) => translate_binary(span, &operands, Operators::And, source),
            ("||", 2) => translate_binary(span, &operands, Operators::Or, source),
            ("[", 2) => translate_index(span, &operands, source),
            _ if is_function_name(op.as_str()) => translate_call(span, op, operands, source),
            _ => Err(ExpressionError::new(
                ExpressionCategory::Parse,
                format!("Unsupported operator: {op}"),
            )),
        },
    }
}

pub(crate) fn translate(parser: &Parser) -> Result<Translator, ExpressionError> {
    let parser_token = parser.get_token().clone();
    let source = parser.get_source().clone();

    translate_token(parser_token, &source).map(|expression| Translator::new(expression, source))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::parser::parse;
    use crate::expression::span::Span;

    fn translate_str(s: &str) -> Result<Expression, ExpressionError> {
        let lexer = crate::expression::lexer::Lexer::new(s)?;
        let parser = parse(&lexer)?;
        translate(&parser).map(|translator| translator.expression().clone())
    }

    #[test]
    fn translates_arithmetic_operators() {
        assert_eq!(translate_str("a + b").unwrap().to_string(), "(a + b)");
        assert_eq!(translate_str("a - b").unwrap().to_string(), "(a - b)");
        assert_eq!(translate_str("a * b").unwrap().to_string(), "(a * b)");
        assert_eq!(translate_str("a / b").unwrap().to_string(), "(a / b)");
        assert_eq!(translate_str("a % b").unwrap().to_string(), "(a % b)");
        assert_eq!(translate_str("a ^ b").unwrap().to_string(), "(a ^ b)");
    }

    #[test]
    fn translates_unary_operators() {
        assert_eq!(translate_str("-a").unwrap().to_string(), "(-a)");
        assert_eq!(translate_str("+a").unwrap().to_string(), "a");
        assert_eq!(translate_str("--a").unwrap().to_string(), "(-(-a))");
    }

    #[test]
    fn translates_comparison_operators() {
        assert_eq!(translate_str("a == b").unwrap().to_string(), "(a == b)");
        assert_eq!(translate_str("a != b").unwrap().to_string(), "(a != b)");
        assert_eq!(translate_str("a < b").unwrap().to_string(), "(a < b)");
        assert_eq!(translate_str("a <= b").unwrap().to_string(), "(a <= b)");
        assert_eq!(translate_str("a > b").unwrap().to_string(), "(a > b)");
        assert_eq!(translate_str("a >= b").unwrap().to_string(), "(a >= b)");
    }

    #[test]
    fn translates_logical_operators() {
        assert_eq!(translate_str("a && b").unwrap().to_string(), "(a && b)");
        assert_eq!(translate_str("a || b").unwrap().to_string(), "(a || b)");
        assert_eq!(
            translate_str("a == b && c != d").unwrap().to_string(),
            "((a == b) && (c != d))"
        );
    }

    #[test]
    fn translates_not_operator() {
        assert_eq!(translate_str("!a").unwrap().to_string(), "(!a)");
        assert_eq!(translate_str("!!a").unwrap().to_string(), "(!(!a))");
        assert_eq!(translate_str("!a && b").unwrap().to_string(), "((!a) && b)");
    }

    #[test]
    fn unsupported_operator_returns_error() {
        for op in &["=", "&", "|"] {
            let token = ParserToken::Operator(
                Span::new(0, 0),
                op.to_string(),
                vec![
                    ParserToken::Atom(Span::new(0, 0), "a".to_string()),
                    ParserToken::Atom(Span::new(0, 0), "b".to_string()),
                ],
            );
            let err = translate_token(token, &ShareableString::from(""))
                .unwrap_err()
                .to_string();
            assert!(err.starts_with("[Parse]"));
            assert!(err.contains(&format!("Unsupported operator: {op}")));
        }
    }

    #[test]
    fn translates_function_calls() {
        assert_eq!(translate_str("f()").unwrap().to_string(), "f()");
        assert_eq!(translate_str("f(a)").unwrap().to_string(), "f(a)");
        assert_eq!(
            translate_str("f(a, b, c)").unwrap().to_string(),
            "f(a, b, c)"
        );
        assert_eq!(
            translate_str("f(a + 1, b * 2)").unwrap().to_string(),
            "f((a + 1), (b * 2))"
        );
    }

    #[test]
    fn translates_nested_function_calls() {
        assert_eq!(
            translate_str("f(g(a), h())").unwrap().to_string(),
            "f(g(a), h())"
        );
    }

    #[test]
    fn translates_array_indexing() {
        assert_eq!(translate_str("arr[0]").unwrap().to_string(), "arr[0]");
        assert_eq!(
            translate_str("arr[i + 1]").unwrap().to_string(),
            "arr[(i + 1)]"
        );

        // indexing can be chained.
        assert_eq!(translate_str("arr[0][1]").unwrap().to_string(), "arr[0][1]");
    }

    #[test]
    fn dot_operator_is_no_longer_supported() {
        // the `.` operator has been removed; field access must now go through bracket
        // indexing (e.g. `p_map[key1][item1]`) instead of `p_map[key1].item1`.
        let err = translate_str("a . b").unwrap_err().to_string();
        assert!(err.starts_with("[Lexer]"));
        assert!(err.contains("Invalid operator in expression: '.'"));
    }

    #[test]
    fn translates_field_access_via_bracket_indexing() {
        // field access is now expressed as a second level of bracket indexing, and can be
        // chained just like array/table indexing.
        assert_eq!(
            translate_str("p_map[key1][item1]").unwrap().to_string(),
            "p_map[key1][item1]"
        );
    }

    #[test]
    fn translates_integer_literals() {
        assert_eq!(
            translate_str("42").unwrap(),
            Expression::Literal(Span::new(0, 2), Literal::Integer(42))
        );
        assert_eq!(translate_str("42").unwrap().to_string(), "42");
    }

    #[test]
    fn translates_float_literals() {
        assert_eq!(
            translate_str("2.5").unwrap(),
            Expression::Literal(Span::new(0, 3), Literal::Float(2.5))
        );
        assert_eq!(translate_str("2.5").unwrap().to_string(), "2.5");

        assert_eq!(
            translate_str(".87").unwrap(),
            Expression::Literal(Span::new(0, 3), Literal::Float(0.87))
        );
    }

    #[test]
    fn translates_scientific_notation_literals() {
        assert_eq!(
            translate_str("1e10").unwrap(),
            Expression::Literal(Span::new(0, 4), Literal::Float(1e10))
        );

        assert_eq!(
            translate_str("1.5e-3").unwrap(),
            Expression::Literal(Span::new(0, 6), Literal::Float(1.5e-3))
        );

        assert_eq!(
            translate_str(".5e+2").unwrap(),
            Expression::Literal(Span::new(0, 5), Literal::Float(0.5e2))
        );

        assert_eq!(
            translate_str("6.022e23").unwrap(),
            Expression::Literal(Span::new(0, 8), Literal::Float(6.022e23))
        );
    }

    #[test]
    fn translates_expressions_mixing_literals_and_variables() {
        assert_eq!(translate_str("a + 1").unwrap().to_string(), "(a + 1)");
        assert_eq!(
            translate_str("f(1, b, 2.5)").unwrap().to_string(),
            "f(1, b, 2.5)"
        );
    }

    #[test]
    fn function_call_span_includes_closing_parenthesis() {
        let expr = translate_str("sin(1.0)").unwrap();
        let span = expression_span(&expr);
        assert_eq!(span.start(), 0);
        assert_eq!(span.end(), 8);
    }

    #[test]
    fn binary_operation_operator_span() {
        let expr = translate_str("5.0*sin(3.)+1.0+1").unwrap();
        match expr {
            Expression::BinaryOperation {
                span,
                operator_span,
                ..
            } => {
                assert_eq!(span.start(), 0);
                assert_eq!(span.end(), 17);
                assert_eq!(operator_span.start(), 15);
                assert_eq!(operator_span.end(), 16);
            }
            other => panic!("expected BinaryOperation, got {other:?}"),
        }
    }
}
