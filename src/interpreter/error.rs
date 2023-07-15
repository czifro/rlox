use std::fmt::{Display, Formatter, Result};

use super::{expression::*, values::LoxType};

#[derive(Debug, Clone)]
pub enum Error {
	UnexpectedToken(i32, String),
	UnterminatedString(i32),
	UnparsableNumber(i32, String),
    WrongTokenType(i32, String, String),
	UnexpectedEof(i32),
	WrongType(i32, Expr, LoxType, LoxType),
	IncompatibleTypes(i32, Expr, LoxType, LoxType),
	RuntimeError(i32, Expr, String),
    Eof,
}

impl Display for Error {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        let mut ast_printer = AstPrinter;
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
			Self::WrongType(line, expr, actual, expected) => write!(
				fmt,
				"[line {line}] Error in expression: {:}:\nExpected type {expected}, found {actual}",
                expr.accept(&mut ast_printer).unwrap(),
			),
			Self::IncompatibleTypes(line, expr, actual, expected) => write!(
				fmt,
				"[line {line}] Incompatible types in expression: {:}:\nExpected type {expected}, found {actual}",
                expr.accept(&mut ast_printer).unwrap(),
			),
			Self::RuntimeError(line, expr, err) => write!(
				fmt,
				"[line {line}] Error in expression: {:}:\n{err}",
                expr.accept(&mut ast_printer).unwrap(),
			),
			Self::UnexpectedEof(line) => write!(fmt, "[line {line}] Error: Unexpected EOF."),
            Self::Eof => write!(fmt, "Error: EOF"),
		}
	}
}

impl std::error::Error for Error {}

