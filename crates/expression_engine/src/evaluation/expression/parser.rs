use crate::expression::lexer::{Lexer, LexerToken};
use crate::expression::span::{Span, SpanSet};
use crate::{ExpressionCategory, ExpressionError};
use shareable_string::ShareableString;
use std::fmt;

/// The result of parsing an expression: either a single atom (an identifier or literal) or a
/// compound expression consisting of an operator applied to one or more operands.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ParserToken {
    /// An atomic value (e.g., a number or identifier).
    Atom(Span, String),
    /// An operator applied to one or more operand expressions.
    Operator(Span, String, Vec<ParserToken>),
}

impl fmt::Display for ParserToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserToken::Atom(i, value) => write!(f, "{{{}}}{}", i, value),
            ParserToken::Operator(i, op, rest) => {
                write!(f, "({{{}}}{}", i, op)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct Parser {
    token: ParserToken,
    source: ShareableString,
}

impl Parser {
    fn new(token: ParserToken, source: ShareableString) -> Self {
        Parser { token, source }
    }

    pub(crate) fn get_token(&self) -> &ParserToken {
        &self.token
    }

    pub(crate) fn get_source(&self) -> &ShareableString {
        &self.source
    }
}

/// Returns the set of source indices covered by `lexer_token`, or an empty set for
/// `LexerToken::EndOfInput`, which doesn't correspond to any position in the source.
fn token_index_set(lexer_token: &LexerToken) -> SpanSet {
    match lexer_token {
        LexerToken::Atom(index, _) => SpanSet::from_span(*index),
        LexerToken::Operator(index, _) => SpanSet::from_span(*index),
        LexerToken::EndOfInput => SpanSet::new(),
    }
}

/// Parses the tokens produced by `lexer` into a fully parenthesized expression tree,
/// respecting operator precedence and associativity.
pub(crate) fn parse(lexer: &Lexer) -> Result<Parser, ExpressionError> {
    let mut lexer = lexer.clone();
    let source = lexer.source().to_string();
    let result = expr_bp(&mut lexer, 0)?;

    match lexer.peek() {
        LexerToken::EndOfInput => Ok(Parser::new(result, source.into())),
        t => {
            let index_set = token_index_set(&t);

            Err(ExpressionError::new_complex(
                ExpressionCategory::Parse,
                format!(
                    "Invalid expression: expected end of input, found {}",
                    describe_token(&t),
                ),
                source,
                index_set,
            ))
        }
    }
}

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> Result<ParserToken, ExpressionError> {
    let mut lhs = match lexer.next() {
        LexerToken::Atom(index, value) => ParserToken::Atom(index, value),
        LexerToken::Operator(_index, op) if op == "(" => {
            let lhs = expr_bp(lexer, 0)?;
            expect_operator(lexer, ")")?;
            lhs
        }
        LexerToken::Operator(index, op) => {
            let ((), r_bp) = prefix_binding_power(&op, index, lexer.source())?;
            let rhs = expr_bp(lexer, r_bp)?;
            ParserToken::Operator(index, op, vec![rhs])
        }
        t => {
            let index_set = token_index_set(&t);
            return Err(ExpressionError::new_complex(
                ExpressionCategory::Parse,
                format!(
                    "Invalid expression: expected an atom or a prefix operator, found {}",
                    describe_token(&t)
                ),
                lexer.source(),
                index_set,
            ));
        }
    };

    loop {
        let (op_index, op) = match lexer.peek() {
            LexerToken::EndOfInput => break,
            LexerToken::Operator(index, value) => (index, value),
            t => {
                let index_set = token_index_set(&t);
                return Err(ExpressionError::new_complex(
                    ExpressionCategory::Parse,
                    format!(
                        "Invalid expression: expected an operator, found {}",
                        describe_token(&t)
                    ),
                    lexer.source(),
                    index_set,
                ));
            }
        };

        if let Some((l_bp, ())) = postfix_binding_power(&op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            lhs = if op == "[" {
                let rhs = expr_bp(lexer, 0)?;
                expect_operator(lexer, "]")?;
                ParserToken::Operator(op_index, op, vec![lhs, rhs])
            } else if op == "(" {
                let (name_index, name) = match lhs {
                    ParserToken::Atom(index, name) => (index, name),
                    other => {
                        return Err(ExpressionError::new_complex(
                            ExpressionCategory::Parse,
                            format!(
                                "Invalid expression: function calls require a function name, found {}",
                                other
                            ),
                            lexer.source(),
                            SpanSet::from_span(op_index),
                        ));
                    }
                };
                let arguments = parse_call_arguments(lexer)?;
                ParserToken::Operator(name_index, name, arguments)
            } else {
                ParserToken::Operator(op_index, op, vec![lhs])
            };
            continue;
        }

        if let Some((l_bp, r_bp)) = infix_binding_power(&op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            let rhs = expr_bp(lexer, r_bp)?;
            lhs = ParserToken::Operator(op_index, op, vec![lhs, rhs]);
            continue;
        }

        break;
    }

    Ok(lhs)
}

/// Parses a comma-separated list of call arguments, up to (but not including) the closing `)`.
fn parse_call_arguments(lexer: &mut Lexer) -> Result<Vec<ParserToken>, ExpressionError> {
    let mut arguments = Vec::new();
    if let LexerToken::Operator(_index, value) = lexer.peek() {
        if value == ")" {
            lexer.next();
            return Ok(arguments);
        }
    }

    loop {
        arguments.push(expr_bp(lexer, 0)?);
        match lexer.peek() {
            LexerToken::Operator(_index, value) if value == "," => {
                lexer.next();
            }
            _ => break,
        }
    }

    expect_operator(lexer, ")")?;
    Ok(arguments)
}

/// Consumes the next token from `lexer`, returning an error if it isn't the expected operator.
fn expect_operator(lexer: &mut Lexer, expected: &str) -> Result<(), ExpressionError> {
    match lexer.next() {
        LexerToken::Operator(_index, value) if value == expected => Ok(()),
        t => {
            let index_set = token_index_set(&t);
            Err(ExpressionError::new_complex(
                ExpressionCategory::Parse,
                format!(
                    "Invalid expression: expected operator '{}', found {}",
                    expected,
                    describe_token(&t)
                ),
                lexer.source(),
                index_set,
            ))
        }
    }
}

/// Returns a human-readable description of `token`, suitable for use in error messages.
fn describe_token(token: &LexerToken) -> String {
    match token {
        LexerToken::Atom(_index, value) => format!("atom '{}'", value),
        LexerToken::Operator(_index, value) => format!("operator '{}'", value),
        LexerToken::EndOfInput => "end of input".to_string(),
    }
}

fn prefix_binding_power(
    op: &str,
    index: Span,
    source: &ShareableString,
) -> Result<((), u8), ExpressionError> {
    match op {
        "+" | "-" => Ok(((), 19)),
        "!" => Ok(((), 19)),
        _ => Err(ExpressionError::new_complex(
            ExpressionCategory::Parse,
            format!("Invalid prefix operator in expression: '{}'", op),
            source,
            SpanSet::from_span(index),
        )),
    }
}

fn postfix_binding_power(op: &str) -> Option<(u8, ())> {
    let res = match op {
        "[" => (21, ()),
        "(" => (21, ()),
        _ => return None,
    };
    Some(res)
}

fn infix_binding_power(op: &str) -> Option<(u8, u8)> {
    let res = match op {
        "=" => (2, 1),
        "||" => (5, 6),
        "&&" => (7, 8),
        "==" | "!=" => (11, 12),
        "<" | "<=" | ">" | ">=" => (13, 14),
        "+" | "-" => (15, 16),
        "*" | "/" | "%" => (17, 18),
        "^" => (20, 19),
        _ => return None,
    };
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expr(s: &str) -> Result<Parser, ExpressionError> {
        let lexer = Lexer::new(s)?;
        parse(&lexer)
    }

    #[test]
    fn basic_test() {
        let s = expr("1").unwrap();
        assert_eq!(s.get_token().to_string(), "{0:1}1");

        let s = expr("1 + 2 * 3").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({2:3}+ {0:1}1 ({6:7}* {4:5}2 {8:9}3))"
        );

        let s = expr("1.5 + 2.5 * 32").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({4:5}+ {0:3}1.5 ({10:11}* {6:9}2.5 {12:14}32))"
        );

        let s = expr("a + b * c * d + e").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({14:15}+ ({2:3}+ {0:1}a ({10:11}* ({6:7}* {4:5}b {8:9}c) {12:13}d)) {16:17}e)"
        );

        let s = expr("--1 * 2").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({4:5}* ({0:1}- ({1:2}- {2:3}1)) {6:7}2)"
        );

        let s = expr("!true").unwrap();
        assert_eq!(s.get_token().to_string(), "({0:1}! {1:5}true)");

        let s = expr("!!true").unwrap();
        assert_eq!(s.get_token().to_string(), "({0:1}! ({1:2}! {2:6}true))");

        let s = expr("(((0)))").unwrap();
        assert_eq!(s.get_token().to_string(), "{3:4}0");

        let s = expr("x[0][1]").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({4:5}[ ({1:2}[ {0:1}x {2:3}0) {5:6}1)"
        );

        let s = expr("x[0+ 1][1]").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({7:8}[ ({1:2}[ {0:1}x ({3:4}+ {2:3}0 {5:6}1)) {8:9}1)"
        );

        let s = expr("x(1,2,3)").unwrap();
        assert_eq!(s.get_token().to_string(), "({0:1}x {2:3}1 {4:5}2 {6:7}3)");
    }

    #[test]
    fn function_call_operators() {
        let s = expr("x()").unwrap();
        assert_eq!(s.get_token().to_string(), "({0:1}x)");

        let s = expr("x(1)").unwrap();
        assert_eq!(s.get_token().to_string(), "({0:1}x {2:3}1)");

        let s = expr("x(1,2,3)").unwrap();
        assert_eq!(s.get_token().to_string(), "({0:1}x {2:3}1 {4:5}2 {6:7}3)");

        // arguments can be arbitrary expressions
        let s = expr("f(a + b, c * d)").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({0:1}f ({4:5}+ {2:3}a {6:7}b) ({11:12}* {9:10}c {13:14}d))"
        );

        // calls can be nested and combined with other operators
        let s = expr("f(g(a), h())").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({0:1}f ({2:3}g {4:5}a) ({8:9}h))"
        );

        let s = expr("f(a) + g(b)").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({5:6}+ ({0:1}f {2:3}a) ({7:8}g {9:10}b))"
        );
    }

    #[test]
    fn comparison_operators() {
        let s = expr("a == b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:4}== {0:1}a {5:6}b)");

        let s = expr("a != b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:4}!= {0:1}a {5:6}b)");

        let s = expr("a < b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:3}< {0:1}a {4:5}b)");

        let s = expr("a <= b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:4}<= {0:1}a {5:6}b)");

        let s = expr("a > b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:3}> {0:1}a {4:5}b)");

        let s = expr("a >= b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:4}>= {0:1}a {5:6}b)");

        // comparisons are left-associative and lower precedence than arithmetic
        let s = expr("a + 1 >= b - 1").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({6:8}>= ({2:3}+ {0:1}a {4:5}1) ({11:12}- {9:10}b {13:14}1))"
        );

        let s = expr("a < b < c").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({6:7}< ({2:3}< {0:1}a {4:5}b) {8:9}c)"
        );
    }

    #[test]
    fn logical_operators() {
        let s = expr("a && b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:4}&& {0:1}a {5:6}b)");

        let s = expr("a || b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:4}|| {0:1}a {5:6}b)");

        // && binds tighter than ||
        let s = expr("a || b && c").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({2:4}|| {0:1}a ({7:9}&& {5:6}b {10:11}c))"
        );

        // comparisons bind tighter than && / ||
        let s = expr("a == b && c != d").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({7:9}&& ({2:4}== {0:1}a {5:6}b) ({12:14}!= {10:11}c {15:16}d))"
        );

        let s = expr(
            "p_value1 >= p_value2 && p_value3 != p_value4 || p_value1 <= p_value2 || p_value3 == p_value4",
        )
        .unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({69:71}|| ({45:47}|| ({21:23}&& ({9:11}>= {0:8}p_value1 {12:20}p_value2) ({33:35}!= {24:32}p_value3 {36:44}p_value4)) ({57:59}<= {48:56}p_value1 {60:68}p_value2)) ({81:83}== {72:80}p_value3 {84:92}p_value4))"
        );
    }

    #[test]
    fn modulo_and_power_operators() {
        let s = expr("a % b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:3}% {0:1}a {4:5}b)");

        // % has same precedence as * and /, higher than +
        let s = expr("a + b % c").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({2:3}+ {0:1}a ({6:7}% {4:5}b {8:9}c))"
        );

        let s = expr("a ^ b").unwrap();
        assert_eq!(s.get_token().to_string(), "({2:3}^ {0:1}a {4:5}b)");

        // ^ (power) binds tighter than *, /, %, and +, -
        let s = expr("a + b ^ c * d").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({2:3}+ {0:1}a ({10:11}* ({6:7}^ {4:5}b {8:9}c) {12:13}d))"
        );

        let s = expr("2 * 3 ^ 2").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({2:3}* {0:1}2 ({6:7}^ {4:5}3 {8:9}2))"
        );

        // ^ is right-associative
        let s = expr("a ^ b ^ c").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({2:3}^ {0:1}a ({6:7}^ {4:5}b {8:9}c))"
        );

        // ^ binds tighter than comparison and &&/||
        let s = expr("a == b ^ c && d").unwrap();
        assert_eq!(
            s.get_token().to_string(),
            "({11:13}&& ({2:4}== {0:1}a ({7:8}^ {5:6}b {9:10}c)) {14:15}d)"
        );
    }

    /// Asserts that parsing `s` fails with an `ExpressionCategory::Parse` error whose message
    /// contains `expected_message`.
    fn assert_parse_error(s: &str, expected_message: &str) {
        let err = expr(s).unwrap_err();
        let err = err.to_string();
        assert!(
            err.starts_with("[Parse]"),
            "expected a Parse error for input {:?}, got: {}",
            s,
            err
        );
        assert!(
            err.contains(expected_message),
            "expected error message for input {:?} to contain {:?}, got: {}",
            s,
            expected_message,
            err
        );
    }

    #[test]
    fn missing_operand_at_start_of_expression() {
        // Empty input: an operand is expected but only EndOfInput is available.
        assert_parse_error(
            "",
            "expected an atom or a prefix operator, found end of input",
        );
    }

    #[test]
    fn missing_operand_after_infix_operator() {
        // After consuming `1` and `+`, the right-hand side is missing.
        assert_parse_error(
            "1+",
            "expected an atom or a prefix operator, found end of input",
        );
    }

    #[test]
    fn missing_operator_between_atoms() {
        // Two atoms in a row with no operator between them.
        assert_parse_error("1 2", "expected an operator, found atom '2'");
    }

    #[test]
    fn unclosed_parenthesis() {
        // Missing closing `)`, so `expect_operator` finds EndOfInput instead.
        assert_parse_error("(1", "expected operator ')', found end of input");
    }

    #[test]
    fn mismatched_closing_bracket() {
        // Closing token doesn't match the expected `)`.
        assert_parse_error("(1]", "expected operator ')', found operator ']'");
    }

    #[test]
    fn bad_prefix_operator() {
        // `*`, `/`, and `^` are not valid prefix operators.
        assert_parse_error("*2", "Invalid prefix operator in expression: '*'");
        assert_parse_error("/2", "Invalid prefix operator in expression: '/'");
        assert_parse_error("^2", "Invalid prefix operator in expression: '^'");
    }

    #[test]
    fn not_operator_is_prefix_only() {
        // `!` is a valid prefix (logical not) operator.
        let s = expr("!a").unwrap();
        assert_eq!(s.get_token().to_string(), "({0:1}! {1:2}a)");

        // `!` is not a valid postfix operator, so trailing `!` is leftover, invalid input.
        assert_parse_error(
            "a!",
            "Invalid expression: expected end of input, found operator '!'",
        );
        assert_parse_error(
            "9!",
            "Invalid expression: expected end of input, found operator '!'",
        );
    }
}
