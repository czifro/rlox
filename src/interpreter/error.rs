use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Error {
	UnexpectedToken(i32, String),
	UnterminatedString(i32),
	UnparsableNumber(i32, String),
	WrongTokenType(i32, String, String),
	UnexpectedEof(i32),
    Eof,
}

impl Display for Error {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
		match self {
			Self::UnexpectedToken(line, token) => {
				write!(fmt, "[line {line}] Error: Unexpected token {token}.")
			}
			Self::UnterminatedString(line) => {
				write!(fmt, "[line {line}] Error: Unterminated string literal.")
			}
			Self::UnparsableNumber(line, message) => write!(fmt, "[line {line}] Error: {message}."),
			Self::WrongTokenType(line, actual, expected) => write!(
				fmt,
				"[line {line}] Error: Expected token type {expected}, found {actual}"
			),
			Self::UnexpectedEof(line) => write!(fmt, "[line {line}] Error: Unexpected EOF."),
            Self::Eof => write!(fmt, "Error: EOF"),
		}
	}
}

impl std::error::Error for Error {}

