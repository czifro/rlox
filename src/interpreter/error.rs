use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub enum Error {
  UnexpectedToken(i32, String),
  UnterminatedString(i32),
  UnparsableNumber(i32, String),
}

impl Display for Error {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
    match self {
      Self::UnexpectedToken(line, token) => write!(fmt, "[line {line}] Error: Unexpected token {token}."),
      Self::UnterminatedString(line) => write!(fmt, "[line {line}] Error: Unterminated string literal."),
      Self::UnparsableNumber(line, message) => write!(fmt, "[line {line}] Error: {message}."),
    }
  }
}

