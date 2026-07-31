use crate::expression::parser::ParserToken;
use crate::{ExpressionCategory, ExpressionError};
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
            Literal::Integer(value) => write!(f, "{}", value),
            Literal::Float(value) => write!(f, "{}", value),
            Literal::String(value) => write!(f, "{}", value),
            Literal::Boolean(value) => write!(f, "{}", value),
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
            Operators::Subtract => "-",
            Operators::Multiply => "*",
            Operators::Divide => "/",
            Operators::Modulus => "%",
            Operators::Power => "^",
            Operators::Negate => "-",
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
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Expression {
    Literal(Literal),
    BinaryOperation {
        left: Box<Expression>,
        operator: Operators,
        right: Box<Expression>,
    },
    UnaryOperation {
        operator: Operators,
        operand: Box<Expression>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
    Index {
        name: String,
        index: Vec<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(literal) => write!(f, "{}", literal),
            Expression::BinaryOperation {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", left, operator, right),
            Expression::UnaryOperation { operator, operand } => {
                write!(f, "({}{})", operator, operand)
            }
            Expression::FunctionCall { name, arguments } => {
                let args = arguments
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{}({})", name, args)
            }
            Expression::Index { name, index } => {
                write!(f, "{}", name)?;
                for idx in index {
                    write!(f, "[{}]", idx)?;
                }
                Ok(())
            }
        }
    }
}

/// Translates a binary `ParserToken::Operator` into a `BinaryOperation` expression.
fn translate_binary(
    operands: &[ParserToken],
    operator: Operators,
) -> Result<Expression, ExpressionError> {
    Ok(Expression::BinaryOperation {
        left: Box::new(translate(operands[0].clone())?),
        operator,
        right: Box::new(translate(operands[1].clone())?),
    })
}

/// Translates a unary `ParserToken::Operator` into a `UnaryOperation` expression.
fn translate_unary(
    operands: &[ParserToken],
    operator: Operators,
) -> Result<Expression, ExpressionError> {
    Ok(Expression::UnaryOperation {
        operator,
        operand: Box::new(translate(operands[0].clone())?),
    })
}

/// Translates a `ParserToken::Operator("[", ...)` into an `Index` expression.
///
/// When the collection being indexed is itself an `Index` expression (i.e., this is a chained
/// index such as `arr[0][1]`), the new index is appended to the existing `Index`'s vector of
/// indices rather than wrapping it in another `Index` expression.
fn translate_index(operands: &[ParserToken]) -> Result<Expression, ExpressionError> {
    let target = translate(operands[0].clone())?;
    let new_index = translate(operands[1].clone())?;

    match target {
        Expression::Index { name, mut index } => {
            index.push(new_index);
            Ok(Expression::Index { name, index })
        }
        other => Ok(Expression::Index {
            name: other.to_string(),
            index: vec![new_index],
        }),
    }
}

/// Translates a `ParserToken::Operator` whose head is a function name (rather than a known
/// operator symbol) into a `FunctionCall` expression.
fn translate_call(
    name: String,
    arguments: Vec<ParserToken>,
) -> Result<Expression, ExpressionError> {
    Ok(Expression::FunctionCall {
        name,
        arguments: arguments
            .into_iter()
            .map(translate)
            .collect::<Result<Vec<_>, _>>()?,
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
fn translate_atom(value: String) -> Result<Expression, ExpressionError> {
    if !is_numeric_literal(&value) {
        if let Ok(boolean) = value.parse::<bool>() {
            return Ok(Expression::Literal(Literal::Boolean(boolean)));
        }

        return Ok(Expression::Literal(Literal::String(value)));
    }

    if let Ok(integer) = value.parse::<i64>() {
        return Ok(Expression::Literal(Literal::Integer(integer)));
    }

    if let Ok(float) = value.parse::<f64>() {
        return Ok(Expression::Literal(Literal::Float(float)));
    }

    Err(ExpressionError::new(
        ExpressionCategory::Parse,
        format!("Invalid numeric literal: {}", value),
    ))
}

pub(crate) fn translate(parser_token: ParserToken) -> Result<Expression, ExpressionError> {
    match parser_token {
        ParserToken::Atom(_index, value) => translate_atom(value),
        ParserToken::Operator(_index, op, operands) => match (op.as_str(), operands.len()) {
            ("+", 1) => translate(operands[0].clone()),
            ("-", 1) => translate_unary(&operands, Operators::Negate),
            ("!", 1) => translate_unary(&operands, Operators::Not),
            ("+", 2) => translate_binary(&operands, Operators::Add),
            ("-", 2) => translate_binary(&operands, Operators::Subtract),
            ("*", 2) => translate_binary(&operands, Operators::Multiply),
            ("/", 2) => translate_binary(&operands, Operators::Divide),
            ("%", 2) => translate_binary(&operands, Operators::Modulus),
            ("^", 2) => translate_binary(&operands, Operators::Power),
            ("==", 2) => translate_binary(&operands, Operators::Equal),
            ("!=", 2) => translate_binary(&operands, Operators::NotEqual),
            ("<", 2) => translate_binary(&operands, Operators::LessThan),
            ("<=", 2) => translate_binary(&operands, Operators::LessThanOrEqual),
            (">", 2) => translate_binary(&operands, Operators::GreaterThan),
            (">=", 2) => translate_binary(&operands, Operators::GreaterThanOrEqual),
            ("&&", 2) => translate_binary(&operands, Operators::And),
            ("||", 2) => translate_binary(&operands, Operators::Or),
            ("[", 2) => translate_index(&operands),
            _ if is_function_name(op.as_str()) => translate_call(op, operands),
            _ => Err(ExpressionError::new(
                ExpressionCategory::Parse,
                format!("Unsupported operator: {}", op),
            )),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::parser::parse;
    use crate::expression::span::Span;

    fn translate_str(s: &str) -> Result<Expression, ExpressionError> {
        let lexer = crate::expression::lexer::Lexer::new(s)?;
        let parser_token = parse(&lexer)?;
        translate(parser_token.get_token().clone())
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
            let err = translate(token).unwrap_err().to_string();
            assert!(err.starts_with("[Parse]"));
            assert!(err.contains(&format!("Unsupported operator: {}", op)));
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
            Expression::Literal(Literal::Integer(42))
        );
        assert_eq!(translate_str("42").unwrap().to_string(), "42");
    }

    #[test]
    fn translates_float_literals() {
        assert_eq!(
            translate_str("2.5").unwrap(),
            Expression::Literal(Literal::Float(2.5))
        );
        assert_eq!(translate_str("2.5").unwrap().to_string(), "2.5");

        assert_eq!(
            translate_str(".87").unwrap(),
            Expression::Literal(Literal::Float(0.87))
        );
    }

    #[test]
    fn translates_scientific_notation_literals() {
        assert_eq!(
            translate_str("1e10").unwrap(),
            Expression::Literal(Literal::Float(1e10))
        );

        assert_eq!(
            translate_str("1.5e-3").unwrap(),
            Expression::Literal(Literal::Float(1.5e-3))
        );

        assert_eq!(
            translate_str(".5e+2").unwrap(),
            Expression::Literal(Literal::Float(0.5e2))
        );

        assert_eq!(
            translate_str("6.022e23").unwrap(),
            Expression::Literal(Literal::Float(6.022e23))
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
}
