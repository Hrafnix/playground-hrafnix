use crate::{ExpressionCategory, ExpressionError};

/// A simple lexer for tokenizing expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LexarToken {
    /// Represents an atomic value (e.g., a number or identifier).
    Atom(String),
    /// Represents an operator (e.g., `+`, `-`, `*`, `/`).
    Operator(String),
    /// Represents the end of the input.
    EndOfInput,
}

/// A simple lexer for tokenizing expressions.
///
/// Valid Tokens for Atoms:
/// - Alphanumeric characters (a-z, 0-9)
/// - Underscore (_)
///
/// Valid Tokens for Operators:
/// - Operators: +, -, *, /, (, ), \[, \], ==, <, >, <=, >=, !=, &&, ||, %, ^, !, ,
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Lexer {
    tokens: Vec<LexarToken>,
}

impl Lexer {
    pub(crate) fn new(input: &str) -> Result<Self, ExpressionError> {
        let mut lexer = Self { tokens: Vec::new() };
        lexer.tokenize(input)?;
        Ok(lexer)
    }

    fn tokenize(&mut self, input: &str) -> Result<(), ExpressionError> {
        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            // Skip whitespace characters
            if c.is_whitespace() {
                continue;
            }

            // Check for invalid characters (non-ASCII)
            if !c.is_ascii() {
                return Err(ExpressionError::new(
                    ExpressionCategory::Lexer,
                    format!("Invalid character in expression: '{}'", c),
                ));
            }

            // Check for invalid tokens
            if !c.is_numeric() && !c.is_lowercase() && !"+_-*/()[]<>=!&|%^.,".contains(c) {
                return Err(ExpressionError::new(
                    ExpressionCategory::Lexer,
                    format!("Invalid character in expression: '{}'", c),
                ));
            }

            if c == '.' {
                let mut s = String::new();
                s.push(c);
                while let Some(&c) = chars.peek() {
                    if c.is_numeric() || c == '.' {
                        s.push(chars.next().expect("peeked value must be present"));
                    } else {
                        break;
                    }
                }

                if s == "." {
                    self.tokens.push(LexarToken::Operator(s));
                } else {
                    self.tokens.push(LexarToken::Atom(s));
                }
            } else if c.is_numeric() {
                let mut s = String::new();
                s.push(c);
                while let Some(&c) = chars.peek() {
                    if c.is_numeric() || c == '.' {
                        s.push(chars.next().expect("peeked value must be present"));
                    } else {
                        break;
                    }
                }
                self.tokens.push(LexarToken::Atom(s));
            } else if c.is_alphanumeric() || c == '_' {
                let mut s = String::new();
                s.push(c);
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' || c == '.' {
                        s.push(chars.next().expect("peeked value must be present"));
                    } else {
                        break;
                    }
                }
                self.tokens.push(LexarToken::Atom(s));
            } else {
                let mut s = String::new();
                s.push(c);
                if let Some(&next_c) = chars.peek() {
                    match (c, next_c) {
                        ('=', '=')
                        | ('!', '=')
                        | ('<', '=')
                        | ('>', '=')
                        | ('&', '&')
                        | ('|', '|') => {
                            s.push(chars.next().expect("peeked value must be present"));
                        }
                        _ => {}
                    }
                }

                self.tokens.push(LexarToken::Operator(s));
            }
        }

        for token in self.tokens.iter() {
            match token {
                LexarToken::Atom(s) => {
                    if s.starts_with("_") {
                        return Err(ExpressionError::new(
                            ExpressionCategory::Lexer,
                            format!("Invalid string in expression: '{}'", s),
                        ));
                    }

                    if s.starts_with(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
                        && s.matches('.').count() > 1
                    {
                        return Err(ExpressionError::new(
                            ExpressionCategory::Lexer,
                            format!("Invalid number in expression: '{}'", s),
                        ));
                    }
                }
                LexarToken::Operator(s) => {
                    if s == "&" || s == "|" || s == "=" || s == "." {
                        return Err(ExpressionError::new(
                            ExpressionCategory::Lexer,
                            format!("Invalid operator in expression: '{}'", s),
                        ));
                    }
                }
                LexarToken::EndOfInput => unreachable!(),
            }
        }

        self.tokens.reverse();

        Ok(())
    }

    pub(crate) fn next(&mut self) -> LexarToken {
        self.tokens.pop().unwrap_or(LexarToken::EndOfInput)
    }

    pub(crate) fn peek(&mut self) -> LexarToken {
        self.tokens
            .last()
            .cloned()
            .unwrap_or(LexarToken::EndOfInput)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let input = "a + b * (c - d)";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexarToken::Atom("a".to_string()),
            LexarToken::Operator("+".to_string()),
            LexarToken::Atom("b".to_string()),
            LexarToken::Operator("*".to_string()),
            LexarToken::Operator("(".to_string()),
            LexarToken::Atom("c".to_string()),
            LexarToken::Operator("-".to_string()),
            LexarToken::Atom("d".to_string()),
            LexarToken::Operator(")".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexarToken::EndOfInput);
    }

    #[test]
    fn basic_test_no_spaces() {
        let input = "a+b*(c-d)";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexarToken::Atom("a".to_string()),
            LexarToken::Operator("+".to_string()),
            LexarToken::Atom("b".to_string()),
            LexarToken::Operator("*".to_string()),
            LexarToken::Operator("(".to_string()),
            LexarToken::Atom("c".to_string()),
            LexarToken::Operator("-".to_string()),
            LexarToken::Atom("d".to_string()),
            LexarToken::Operator(")".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexarToken::EndOfInput);
    }

    #[test]
    fn test_practical_example_1() {
        let input = "g_test + p_apple * (v_one - v_two)";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexarToken::Atom("g_test".to_string()),
            LexarToken::Operator("+".to_string()),
            LexarToken::Atom("p_apple".to_string()),
            LexarToken::Operator("*".to_string()),
            LexarToken::Operator("(".to_string()),
            LexarToken::Atom("v_one".to_string()),
            LexarToken::Operator("-".to_string()),
            LexarToken::Atom("v_two".to_string()),
            LexarToken::Operator(")".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexarToken::EndOfInput);
    }

    #[test]
    fn test_practical_example_2() {
        let input = "sin(p_angle)/(v_table[1][1]^2) + 43.5!";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexarToken::Atom("sin".to_string()),
            LexarToken::Operator("(".to_string()),
            LexarToken::Atom("p_angle".to_string()),
            LexarToken::Operator(")".to_string()),
            LexarToken::Operator("/".to_string()),
            LexarToken::Operator("(".to_string()),
            LexarToken::Atom("v_table".to_string()),
            LexarToken::Operator("[".to_string()),
            LexarToken::Atom("1".to_string()),
            LexarToken::Operator("]".to_string()),
            LexarToken::Operator("[".to_string()),
            LexarToken::Atom("1".to_string()),
            LexarToken::Operator("]".to_string()),
            LexarToken::Operator("^".to_string()),
            LexarToken::Atom("2".to_string()),
            LexarToken::Operator(")".to_string()),
            LexarToken::Operator("+".to_string()),
            LexarToken::Atom("43.5".to_string()),
            LexarToken::Operator("!".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexarToken::EndOfInput);
    }

    #[test]
    fn test_practical_example_3() {
        let input = "p_value1 >= p_value2 && p_value3 != p_value4 || p_value1 <= p_value2 || p_value3 == p_value4";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexarToken::Atom("p_value1".to_string()),
            LexarToken::Operator(">=".to_string()),
            LexarToken::Atom("p_value2".to_string()),
            LexarToken::Operator("&&".to_string()),
            LexarToken::Atom("p_value3".to_string()),
            LexarToken::Operator("!=".to_string()),
            LexarToken::Atom("p_value4".to_string()),
            LexarToken::Operator("||".to_string()),
            LexarToken::Atom("p_value1".to_string()),
            LexarToken::Operator("<=".to_string()),
            LexarToken::Atom("p_value2".to_string()),
            LexarToken::Operator("||".to_string()),
            LexarToken::Atom("p_value3".to_string()),
            LexarToken::Operator("==".to_string()),
            LexarToken::Atom("p_value4".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexarToken::EndOfInput);
    }

    #[test]
    fn test_practical_example_4() {
        let input = "p_map[key1][item1] + p_map[key2][item2]";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexarToken::Atom("p_map".to_string()),
            LexarToken::Operator("[".to_string()),
            LexarToken::Atom("key1".to_string()),
            LexarToken::Operator("]".to_string()),
            LexarToken::Operator("[".to_string()),
            LexarToken::Atom("item1".to_string()),
            LexarToken::Operator("]".to_string()),
            LexarToken::Operator("+".to_string()),
            LexarToken::Atom("p_map".to_string()),
            LexarToken::Operator("[".to_string()),
            LexarToken::Atom("key2".to_string()),
            LexarToken::Operator("]".to_string()),
            LexarToken::Operator("[".to_string()),
            LexarToken::Atom("item2".to_string()),
            LexarToken::Operator("]".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexarToken::EndOfInput);
    }

    #[test]
    fn test_practical_example_5() {
        let input = "function(p_map[key1][item1], p_map[key2][item2])";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexarToken::Atom("function".to_string()),
            LexarToken::Operator("(".to_string()),
            LexarToken::Atom("p_map".to_string()),
            LexarToken::Operator("[".to_string()),
            LexarToken::Atom("key1".to_string()),
            LexarToken::Operator("]".to_string()),
            LexarToken::Operator("[".to_string()),
            LexarToken::Atom("item1".to_string()),
            LexarToken::Operator("]".to_string()),
            LexarToken::Operator(",".to_string()),
            LexarToken::Atom("p_map".to_string()),
            LexarToken::Operator("[".to_string()),
            LexarToken::Atom("key2".to_string()),
            LexarToken::Operator("]".to_string()),
            LexarToken::Operator("[".to_string()),
            LexarToken::Atom("item2".to_string()),
            LexarToken::Operator("]".to_string()),
            LexarToken::Operator(")".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexarToken::EndOfInput);
    }

    #[test]
    fn test_practical_example_6() {
        let input = "2.0p_value1 + 5.0p_value2 * 6.0(.87p_value3 - 77p_value4) / p_value5";
        let mut lexer = Lexer::new(input).unwrap();

        let expected_tokens = vec![
            LexarToken::Atom("2.0".to_string()),
            LexarToken::Atom("p_value1".to_string()),
            LexarToken::Operator("+".to_string()),
            LexarToken::Atom("5.0".to_string()),
            LexarToken::Atom("p_value2".to_string()),
            LexarToken::Operator("*".to_string()),
            LexarToken::Atom("6.0".to_string()),
            LexarToken::Operator("(".to_string()),
            LexarToken::Atom(".87".to_string()),
            LexarToken::Atom("p_value3".to_string()),
            LexarToken::Operator("-".to_string()),
            LexarToken::Atom("77".to_string()),
            LexarToken::Atom("p_value4".to_string()),
            LexarToken::Operator(")".to_string()),
            LexarToken::Operator("/".to_string()),
            LexarToken::Atom("p_value5".to_string()),
        ];

        for expected in expected_tokens {
            let token = lexer.next();
            assert_eq!(token, expected);
        }

        // Ensure that the lexer returns EndOfInput after all tokens are consumed
        assert_eq!(lexer.next(), LexarToken::EndOfInput);
    }

    #[test]
    fn test_peak_and_next_index() {
        let input = "a + b * (c - d)";
        let mut lexer = Lexer::new(input).unwrap();

        // Peek at the first token
        let token = lexer.peek();
        assert_eq!(token, LexarToken::Atom("a".to_string()));

        // Consume the first token
        let token = lexer.next();
        assert_eq!(token, LexarToken::Atom("a".to_string()));

        // Peek at the next token
        let token = lexer.peek();
        assert_eq!(token, LexarToken::Operator("+".to_string()));
    }

    #[test]
    fn test_invalid_characters_1() {
        for c in 0..=255 {
            let ch = c as u8 as char;
            if !ch.is_numeric()
                && !ch.is_lowercase()
                && !"+_-*/()[]<>=!&|%^.,".contains(ch)
                && !ch.is_whitespace()
            {
                let input = format!("a + b * (c - d) {} e", ch);
                let result = Lexer::new(&input);
                assert!(result.is_err());
                let error = result.err().unwrap();
                assert_eq!(error.category, ExpressionCategory::Lexer);
                assert_eq!(
                    error.message,
                    format!("Invalid character in expression: '{}'", ch)
                );
            }
        }
    }

    #[test]
    fn test_invalid_characters_2() {
        let input = "a + b * (c - d) \u{1F600} e"; // Includes a non-ASCII character (😀)
        let result = Lexer::new(input);
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.category, ExpressionCategory::Lexer);
        assert_eq!(
            error.message,
            format!("Invalid character in expression: '{}'", '\u{1F600}')
        );
    }

    #[test]
    fn test_invalid_characters_3() {
        let input = "5..0";
        let result = Lexer::new(input);
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.category, ExpressionCategory::Lexer);
        assert_eq!(error.message, "Invalid number in expression: '5..0'");
    }

    #[test]
    fn test_invalid_characters_4() {
        for c in "=&|.".chars() {
            let ch = c as u8 as char;
            let input = format!("a + b * (c - d) {} e", ch);
            let result = Lexer::new(&input);
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert_eq!(error.category, ExpressionCategory::Lexer);
            assert_eq!(
                error.message,
                format!("Invalid operator in expression: '{}'", ch)
            );
        }
    }

    #[test]
    fn test_invalid_characters_5() {
        for c in "_".chars() {
            let ch = c as u8 as char;
            let input = format!("a + b * (c - d) {} e", ch);
            let result = Lexer::new(&input);
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert_eq!(error.category, ExpressionCategory::Lexer);
            assert_eq!(
                error.message,
                format!("Invalid string in expression: '{}'", ch)
            );
        }
    }
}
