//! Configuration-time compiler for the signal Expression primitive.

/// A compiled scalar expression over the current input and simulation time.
#[derive(Debug)]
pub(crate) struct CompiledSignalExpression {
    /// Root of the immutable compiled syntax tree.
    root: Node,
}

/// Stable expression compilation failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CompileError {
    /// The source was empty or ended before an operand was complete.
    MissingOperand,
    /// The source contained an unsupported token or identifier.
    UnexpectedToken,
    /// A parenthesized expression was not closed.
    MissingClosingParenthesis,
    /// A numeric literal was malformed or nonfinite.
    InvalidNumber,
}

/// Immutable expression syntax tree allocated only during configuration.
#[derive(Debug)]
enum Node {
    /// Numeric literal.
    Constant(f64),
    /// Current input value (`x`).
    Input,
    /// Current simulation time.
    Time,
    /// Unary negation.
    Negate(Box<Self>),
    /// Scalar addition.
    Add(Box<Self>, Box<Self>),
    /// Scalar subtraction.
    Subtract(Box<Self>, Box<Self>),
    /// Scalar multiplication.
    Multiply(Box<Self>, Box<Self>),
    /// Scalar division.
    Divide(Box<Self>, Box<Self>),
}

impl CompiledSignalExpression {
    /// Compiles one complete expression source.
    pub(crate) fn compile(source: &str) -> Result<Self, CompileError> {
        let mut parser = Parser::new(source);
        let root = parser.expression()?;
        parser.skip_whitespace();
        if parser.current().is_some() {
            return Err(CompileError::UnexpectedToken);
        }
        Ok(Self { root })
    }

    /// Evaluates without allocating or mutating compiled state.
    pub(crate) fn evaluate(&self, input: f64, time: f64) -> Option<f64> {
        self.root
            .evaluate(input, time)
            .filter(|value| value.is_finite())
    }
}

impl Node {
    /// Recursively evaluates one immutable syntax node.
    #[allow(
        clippy::float_arithmetic,
        reason = "Compiled signal expressions implement scalar arithmetic."
    )]
    fn evaluate(&self, input: f64, time: f64) -> Option<f64> {
        match self {
            Self::Constant(value) => Some(*value),
            Self::Input => Some(input),
            Self::Time => Some(time),
            Self::Negate(value) => Some(-value.evaluate(input, time)?),
            Self::Add(left, right) => {
                Some(left.evaluate(input, time)? + right.evaluate(input, time)?)
            }
            Self::Subtract(left, right) => {
                Some(left.evaluate(input, time)? - right.evaluate(input, time)?)
            }
            Self::Multiply(left, right) => {
                Some(left.evaluate(input, time)? * right.evaluate(input, time)?)
            }
            Self::Divide(left, right) => {
                Some(left.evaluate(input, time)? / right.evaluate(input, time)?)
            }
        }
    }
}

/// Recursive-descent parser over an ASCII expression source.
struct Parser<'a> {
    /// Complete ASCII source bytes.
    source: &'a [u8],
    /// Current byte position.
    position: usize,
}

impl<'a> Parser<'a> {
    /// Creates a parser at the start of one expression.
    const fn new(source: &'a str) -> Self {
        Self {
            source: source.as_bytes(),
            position: 0,
        }
    }

    /// Parses addition and subtraction.
    fn expression(&mut self) -> Result<Node, CompileError> {
        let mut node = self.term()?;
        loop {
            self.skip_whitespace();
            node = match self.current() {
                Some(b'+') => {
                    self.advance();
                    Node::Add(Box::new(node), Box::new(self.term()?))
                }
                Some(b'-') => {
                    self.advance();
                    Node::Subtract(Box::new(node), Box::new(self.term()?))
                }
                Some(_) | None => return Ok(node),
            };
        }
    }

    /// Parses multiplication and division.
    fn term(&mut self) -> Result<Node, CompileError> {
        let mut node = self.unary()?;
        loop {
            self.skip_whitespace();
            node = match self.current() {
                Some(b'*') => {
                    self.advance();
                    Node::Multiply(Box::new(node), Box::new(self.unary()?))
                }
                Some(b'/') => {
                    self.advance();
                    Node::Divide(Box::new(node), Box::new(self.unary()?))
                }
                Some(_) | None => return Ok(node),
            };
        }
    }

    /// Parses unary signs.
    fn unary(&mut self) -> Result<Node, CompileError> {
        self.skip_whitespace();
        match self.current() {
            Some(b'+') => {
                self.advance();
                self.unary()
            }
            Some(b'-') => {
                self.advance();
                Ok(Node::Negate(Box::new(self.unary()?)))
            }
            Some(_) => self.primary(),
            None => Err(CompileError::MissingOperand),
        }
    }

    /// Parses literals, identifiers, and parentheses.
    fn primary(&mut self) -> Result<Node, CompileError> {
        self.skip_whitespace();
        match self.current() {
            Some(b'(') => {
                self.advance();
                let node = self.expression()?;
                self.skip_whitespace();
                if self.current() != Some(b')') {
                    return Err(CompileError::MissingClosingParenthesis);
                }
                self.advance();
                Ok(node)
            }
            Some(value) if value.is_ascii_digit() || value == b'.' => self.number(),
            Some(value) if value.is_ascii_alphabetic() => self.identifier(),
            Some(_) => Err(CompileError::UnexpectedToken),
            None => Err(CompileError::MissingOperand),
        }
    }

    /// Parses one finite decimal or scientific numeric literal.
    fn number(&mut self) -> Result<Node, CompileError> {
        let start = self.position;
        while self.current().is_some_and(|value| {
            value.is_ascii_digit() || matches!(value, b'.' | b'e' | b'E' | b'+' | b'-')
        }) {
            if matches!(self.current(), Some(b'+' | b'-'))
                && !matches!(self.previous(), Some(b'e' | b'E'))
            {
                break;
            }
            self.advance();
        }
        let bytes = self
            .source
            .get(start..self.position)
            .ok_or(CompileError::InvalidNumber)?;
        let text = std::str::from_utf8(bytes).map_err(|_| CompileError::InvalidNumber)?;
        let value = text
            .parse::<f64>()
            .map_err(|_| CompileError::InvalidNumber)?;
        value
            .is_finite()
            .then_some(Node::Constant(value))
            .ok_or(CompileError::InvalidNumber)
    }

    /// Parses one supported runtime symbol.
    fn identifier(&mut self) -> Result<Node, CompileError> {
        let start = self.position;
        while self
            .current()
            .is_some_and(|value| value.is_ascii_alphanumeric())
        {
            self.advance();
        }
        match self
            .source
            .get(start..self.position)
            .ok_or(CompileError::UnexpectedToken)?
        {
            b"x" => Ok(Node::Input),
            b"time" => Ok(Node::Time),
            _ => Err(CompileError::UnexpectedToken),
        }
    }

    /// Advances past ASCII whitespace.
    fn skip_whitespace(&mut self) {
        while self
            .current()
            .is_some_and(|value| value.is_ascii_whitespace())
        {
            self.advance();
        }
    }

    /// Returns the current source byte.
    fn current(&self) -> Option<u8> {
        self.source.get(self.position).copied()
    }

    /// Returns the prior source byte.
    fn previous(&self) -> Option<u8> {
        self.position
            .checked_sub(1)
            .and_then(|position| self.source.get(position))
            .copied()
    }

    /// Advances one source byte without wrapping.
    const fn advance(&mut self) {
        self.position = self.position.saturating_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{CompileError, CompiledSignalExpression};

    #[test]
    fn compiles_precedence_parentheses_and_runtime_symbols() {
        let expression = CompiledSignalExpression::compile("2 * (x + time) - 1").unwrap();

        assert_eq!(expression.evaluate(3.0, 0.5), Some(6.0));
    }

    #[test]
    fn rejects_unknown_identifiers_and_nonfinite_results() {
        assert_eq!(
            CompiledSignalExpression::compile("unknown + 1").unwrap_err(),
            CompileError::UnexpectedToken
        );
        let expression = CompiledSignalExpression::compile("1 / (x - x)").unwrap();
        assert_eq!(expression.evaluate(2.0, 0.0), None);
    }
}
