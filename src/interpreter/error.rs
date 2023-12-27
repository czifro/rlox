use std::fmt::{Display, Formatter, Result};

use super::{expression::*, values::LoxType, token::Token};

#[derive(Debug, Clone)]
pub enum Error {
	UnexpectedToken(i32, String),
	UnterminatedString(i32),
	UnparsableNumber(i32, String),
	WrongTokenType(i32, String, String),
	InvalidAssignmentTarget(i32, String),
	UnexpectedEof(i32),
	WrongType(i32, Expr, LoxType, LoxType),
	IncompatibleTypes(i32, Expr, LoxType, LoxType),
	InoperableTypes(Token, Expr, Vec<LoxType>, LoxType, LoxType),
	RuntimeError(i32, Expr, String),
	UndefinedVariable(i32, String)
	// Eof,
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
			Self::InvalidAssignmentTarget(line, target) => write!(fmt, "[line {line}] Error: Invalid Assignment Target: {target}."),
			Self::WrongType(line, expr, actual, expected) => write!(
				fmt,
				"[line {line}] Error in expression: {:}:\nExpected type {expected}, found {actual}",
				expr.accept(&mut ast_printer).unwrap(),
			),
			Self::IncompatibleTypes(line, expr, left, right) => write!(
				fmt,
				"[line {line}] Incompatible types in expression: {:}: {left}, {right}",
				expr.accept(&mut ast_printer).unwrap(),
			),
			Self::InoperableTypes(op, expr, supported_types, left, right) => write!(
				fmt,
				"[line {:}] Operator cannot be used on types in expression: {:}: {left}, {right}. Supported types: {:}",
				op.line,
				expr.accept(&mut ast_printer).unwrap(),
				supported_types.iter().map(ToOwned::to_owned).map(String::from).collect::<Vec<_>>().join(", "),
			),
			Self::RuntimeError(line, expr, err) => write!(
				fmt,
				"[line {line}] Error in expression: {:}:\n{err}",
				expr.accept(&mut ast_printer).unwrap(),
			),
			Self::UndefinedVariable(line, var) => write!(
				fmt,
				"[line {line}] Undefined variable: {var}",
			),
			Self::UnexpectedEof(line) => write!(fmt, "[line {line}] Error: Unexpected EOF."),
			// Self::Eof => write!(fmt, "Error: EOF"),
		}
	}
}

impl std::error::Error for Error {}
