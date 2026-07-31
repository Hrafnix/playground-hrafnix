use crate::expression::lexer::{Lexer, LexerToken};
use crate::{ExpressionCategory, ExpressionError};
use std::fmt;

/// The result of parsing an expression: either a single atom (an identifier or literal) or a
/// compound expression consisting of an operator applied to one or more operands.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ParserToken {
    /// An atomic value (e.g., a number or identifier).
    Atom(String),
    /// An operator applied to one or more operand expressions.
    Operator(String, Vec<ParserToken>),
}

impl fmt::Display for ParserToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserToken::Atom(i) => write!(f, "{}", i),
            ParserToken::Operator(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

/// Parses the tokens produced by `lexer` into a fully parenthesized expression tree,
/// respecting operator precedence and associativity.
pub(crate) fn parse(lexer: &Lexer) -> Result<ParserToken, ExpressionError> {
    let mut lexer = lexer.clone();
    let result = expr_bp(&mut lexer, 0)?;

    match lexer.peek() {
        LexerToken::EndOfInput => Ok(result),
        t => Err(ExpressionError::new(
            ExpressionCategory::Parse,
            format!(
                "Invalid expression: expected end of input, found {}",
                describe_token(&t)
            ),
        )),
    }
}

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> Result<ParserToken, ExpressionError> {
    let mut lhs = match lexer.next() {
        LexerToken::Atom(it) => ParserToken::Atom(it),
        LexerToken::Operator(op) if op == "(" => {
            let lhs = expr_bp(lexer, 0)?;
            expect_operator(lexer, ")")?;
            lhs
        }
        LexerToken::Operator(op) => {
            let ((), r_bp) = prefix_binding_power(&op)?;
            let rhs = expr_bp(lexer, r_bp)?;
            ParserToken::Operator(op, vec![rhs])
        }
        t => {
            return Err(ExpressionError::new(
                ExpressionCategory::Parse,
                format!(
                    "Invalid expression: expected an atom or a prefix operator, found {}",
                    describe_token(&t)
                ),
            ));
        }
    };

    loop {
        let op = match lexer.peek() {
            LexerToken::EndOfInput => break,
            LexerToken::Operator(op) => op,
            t => {
                return Err(ExpressionError::new(
                    ExpressionCategory::Parse,
                    format!(
                        "Invalid expression: expected an operator, found {}",
                        describe_token(&t)
                    ),
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
                ParserToken::Operator(op, vec![lhs, rhs])
            } else if op == "(" {
                let name = match lhs {
                    ParserToken::Atom(name) => name,
                    other => {
                        return Err(ExpressionError::new(
                            ExpressionCategory::Parse,
                            format!(
                                "Invalid expression: function calls require a function name, found {}",
                                other
                            ),
                        ));
                    }
                };
                let arguments = parse_call_arguments(lexer)?;
                ParserToken::Operator(name, arguments)
            } else {
                ParserToken::Operator(op, vec![lhs])
            };
            continue;
        }

        if let Some((l_bp, r_bp)) = infix_binding_power(&op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            let rhs = expr_bp(lexer, r_bp)?;
            lhs = ParserToken::Operator(op, vec![lhs, rhs]);
            continue;
        }

        break;
    }

    Ok(lhs)
}

/// Parses a comma-separated list of call arguments, up to (but not including) the closing `)`.
fn parse_call_arguments(lexer: &mut Lexer) -> Result<Vec<ParserToken>, ExpressionError> {
    let mut arguments = Vec::new();

    if lexer.peek() == LexerToken::Operator(")".to_string()) {
        lexer.next();
        return Ok(arguments);
    }

    loop {
        arguments.push(expr_bp(lexer, 0)?);
        match lexer.peek() {
            LexerToken::Operator(comma) if comma == "," => {
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
        LexerToken::Operator(op) if op == expected => Ok(()),
        t => Err(ExpressionError::new(
            ExpressionCategory::Parse,
            format!(
                "Invalid expression: expected operator '{}', found {}",
                expected,
                describe_token(&t)
            ),
        )),
    }
}

/// Returns a human-readable description of `token`, suitable for use in error messages.
fn describe_token(token: &LexerToken) -> String {
    match token {
        LexerToken::Atom(s) => format!("atom '{}'", s),
        LexerToken::Operator(s) => format!("operator '{}'", s),
        LexerToken::EndOfInput => "end of input".to_string(),
    }
}

fn prefix_binding_power(op: &str) -> Result<((), u8), ExpressionError> {
    match op {
        "+" | "-" => Ok(((), 19)),
        "!" => Ok(((), 19)),
        _ => Err(ExpressionError::new(
            ExpressionCategory::Parse,
            format!("Invalid prefix operator in expression: '{}'", op),
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

    fn expr(s: &str) -> Result<ParserToken, ExpressionError> {
        let lexer = Lexer::new(s)?;
        parse(&lexer)
    }

    #[test]
    fn basic_test() {
        let s = expr("1").unwrap();
        assert_eq!(s.to_string(), "1");

        let s = expr("1 + 2 * 3").unwrap();
        assert_eq!(s.to_string(), "(+ 1 (* 2 3))");

        let s = expr("1.5 + 2.5 * 32").unwrap();
        assert_eq!(s.to_string(), "(+ 1.5 (* 2.5 32))");

        let s = expr("a + b * c * d + e").unwrap();
        assert_eq!(s.to_string(), "(+ (+ a (* (* b c) d)) e)");

        let s = expr("--1 * 2").unwrap();
        assert_eq!(s.to_string(), "(* (- (- 1)) 2)");

        let s = expr("!true").unwrap();
        assert_eq!(s.to_string(), "(! true)");

        let s = expr("!!true").unwrap();
        assert_eq!(s.to_string(), "(! (! true))");

        let s = expr("(((0)))").unwrap();
        assert_eq!(s.to_string(), "0");

        let s = expr("x[0][1]").unwrap();
        assert_eq!(s.to_string(), "([ ([ x 0) 1)");

        let s = expr("x[0+ 1][1]").unwrap();
        assert_eq!(s.to_string(), "([ ([ x (+ 0 1)) 1)");

        let s = expr("x(1,2,3)").unwrap();
        assert_eq!(s.to_string(), "(x 1 2 3)");
    }

    #[test]
    fn function_call_operators() {
        let s = expr("x()").unwrap();
        assert_eq!(s.to_string(), "(x)");

        let s = expr("x(1)").unwrap();
        assert_eq!(s.to_string(), "(x 1)");

        let s = expr("x(1,2,3)").unwrap();
        assert_eq!(s.to_string(), "(x 1 2 3)");

        // arguments can be arbitrary expressions
        let s = expr("f(a + b, c * d)").unwrap();
        assert_eq!(s.to_string(), "(f (+ a b) (* c d))");

        // calls can be nested and combined with other operators
        let s = expr("f(g(a), h())").unwrap();
        assert_eq!(s.to_string(), "(f (g a) (h))");

        let s = expr("f(a) + g(b)").unwrap();
        assert_eq!(s.to_string(), "(+ (f a) (g b))");
    }

    #[test]
    fn comparison_operators() {
        let s = expr("a == b").unwrap();
        assert_eq!(s.to_string(), "(== a b)");

        let s = expr("a != b").unwrap();
        assert_eq!(s.to_string(), "(!= a b)");

        let s = expr("a < b").unwrap();
        assert_eq!(s.to_string(), "(< a b)");

        let s = expr("a <= b").unwrap();
        assert_eq!(s.to_string(), "(<= a b)");

        let s = expr("a > b").unwrap();
        assert_eq!(s.to_string(), "(> a b)");

        let s = expr("a >= b").unwrap();
        assert_eq!(s.to_string(), "(>= a b)");

        // comparisons are left-associative and lower precedence than arithmetic
        let s = expr("a + 1 >= b - 1").unwrap();
        assert_eq!(s.to_string(), "(>= (+ a 1) (- b 1))");

        let s = expr("a < b < c").unwrap();
        assert_eq!(s.to_string(), "(< (< a b) c)");
    }

    #[test]
    fn logical_operators() {
        let s = expr("a && b").unwrap();
        assert_eq!(s.to_string(), "(&& a b)");

        let s = expr("a || b").unwrap();
        assert_eq!(s.to_string(), "(|| a b)");

        // && binds tighter than ||
        let s = expr("a || b && c").unwrap();
        assert_eq!(s.to_string(), "(|| a (&& b c))");

        // comparisons bind tighter than && / ||
        let s = expr("a == b && c != d").unwrap();
        assert_eq!(s.to_string(), "(&& (== a b) (!= c d))");

        let s = expr(
            "p_value1 >= p_value2 && p_value3 != p_value4 || p_value1 <= p_value2 || p_value3 == p_value4",
        )
        .unwrap();
        assert_eq!(
            s.to_string(),
            "(|| (|| (&& (>= p_value1 p_value2) (!= p_value3 p_value4)) (<= p_value1 p_value2)) (== p_value3 p_value4))"
        );
    }

    #[test]
    fn modulo_and_power_operators() {
        let s = expr("a % b").unwrap();
        assert_eq!(s.to_string(), "(% a b)");

        // % has same precedence as * and /, higher than +
        let s = expr("a + b % c").unwrap();
        assert_eq!(s.to_string(), "(+ a (% b c))");

        let s = expr("a ^ b").unwrap();
        assert_eq!(s.to_string(), "(^ a b)");

        // ^ (power) binds tighter than *, /, %, and +, -
        let s = expr("a + b ^ c * d").unwrap();
        assert_eq!(s.to_string(), "(+ a (* (^ b c) d))");

        let s = expr("2 * 3 ^ 2").unwrap();
        assert_eq!(s.to_string(), "(* 2 (^ 3 2))");

        // ^ is right-associative
        let s = expr("a ^ b ^ c").unwrap();
        assert_eq!(s.to_string(), "(^ a (^ b c))");

        // ^ binds tighter than comparison and &&/||
        let s = expr("a == b ^ c && d").unwrap();
        assert_eq!(s.to_string(), "(&& (== a (^ b c)) d)");
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
        assert_eq!(s.to_string(), "(! a)");

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
