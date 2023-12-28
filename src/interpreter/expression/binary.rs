use super::{Expr, Visitor};
use crate::interpreter::error::Error;
use crate::interpreter::values::{LoxType, LoxValue};
use crate::interpreter::{Token, TokenType};

pub fn visit_binary_expression<V>(
	expr: &Expr,
	left: Box<Expr>,
	op: Token,
	right: Box<Expr>,
	visitor: &mut V,
) -> Result<LoxValue, Error>
where
	V: Visitor<LoxValue, Expr>,
{
	let left = left.accept(visitor)?;
	let right = right.accept(visitor)?;
	match op.token_type.clone() {
		TokenType::BangEqual | TokenType::EqualEqual => {
			visit_equality_expression(expr, left, op, right)
		}
		TokenType::Greater
		| TokenType::GreaterEqual
		| TokenType::Less
		| TokenType::LessEqual => {
			visit_comparison_expression(expr, left, op, right)
		}
		TokenType::Star | TokenType::Slash => {
			visit_factor_expression(expr, left, op, right)
		}
		TokenType::Minus | TokenType::Plus => {
			visit_term_expression(expr, left, op, right)
		}
		_ => unreachable!("Binary operator: {:?}", op.token_type),
	}
}

fn visit_equality_expression(
	expr: &Expr,
	left: LoxValue,
	op: Token,
	right: LoxValue,
) -> Result<LoxValue, Error> {
	if !left.is_nil()
		&& !right.is_nil()
		&& !left.is_same_type_as(&right)
	{
		return Err(Error::IncompatibleTypes(
			op.line,
			expr.to_owned(),
			left.lox_type(),
			right.lox_type(),
		));
	}
	match op.token_type.clone() {
		TokenType::BangEqual => Ok(LoxValue::Bool(left != right)),
		TokenType::EqualEqual => Ok(LoxValue::Bool(left == right)),
		_ => {
			unreachable!("Comparison operator: {:?}", op.token_type)
		}
	}
}

fn visit_comparison_expression(
	expr: &Expr,
	left: LoxValue,
	op: Token,
	right: LoxValue,
) -> Result<LoxValue, Error> {
	if !left.is_nil()
		&& !right.is_nil()
		&& !left.is_same_type_as(&right)
	{
		return Err(Error::IncompatibleTypes(
			op.line,
			expr.to_owned(),
			left.lox_type(),
			right.lox_type(),
		));
	}
	match op.token_type.clone() {
		TokenType::Greater => {
			if left.is_nil()
				|| right.is_nil()
				|| !left.is_same_type_as(&right)
			{
				return Err(Error::IncompatibleTypes(
					op.line,
					expr.to_owned(),
					left.lox_type(),
					right.lox_type(),
				));
			}
			Ok(LoxValue::Bool(left > right))
		}
		TokenType::GreaterEqual => {
			if left.is_nil()
				|| right.is_nil()
				|| !left.is_same_type_as(&right)
			{
				return Err(Error::IncompatibleTypes(
					op.line,
					expr.to_owned(),
					left.lox_type(),
					right.lox_type(),
				));
			}
			Ok(LoxValue::Bool(left >= right))
		}
		TokenType::Less => {
			if left.is_nil()
				|| right.is_nil()
				|| !left.is_same_type_as(&right)
			{
				return Err(Error::IncompatibleTypes(
					op.line,
					expr.to_owned(),
					left.lox_type(),
					right.lox_type(),
				));
			}
			Ok(LoxValue::Bool(left < right))
		}
		TokenType::LessEqual => {
			if left.is_nil()
				|| right.is_nil()
				|| !left.is_same_type_as(&right)
			{
				return Err(Error::IncompatibleTypes(
					op.line,
					expr.to_owned(),
					left.lox_type(),
					right.lox_type(),
				));
			}
			Ok(LoxValue::Bool(left <= right))
		}
		_ => {
			unreachable!("Comparison operator: {:?}", op.token_type)
		}
	}
}

fn visit_factor_expression(
	expr: &Expr,
	left: LoxValue,
	op: Token,
	right: LoxValue,
) -> Result<LoxValue, Error> {
	match op.token_type.clone() {
		TokenType::Star => match (&left, &right) {
			(LoxValue::Number(l), LoxValue::Number(r)) => {
				Ok(LoxValue::Number(l * r))
			}
			(_, LoxValue::Number(_)) => Err(Error::IncompatibleTypes(
				op.line,
				expr.to_owned(),
				right.lox_type(),
				left.lox_type(),
			)),
			(LoxValue::Number(_), _) => Err(Error::IncompatibleTypes(
				op.line,
				expr.to_owned(),
				left.lox_type(),
				right.lox_type(),
			)),
			_ => Err(Error::InoperableTypes(
				op.to_owned(),
				expr.to_owned(),
				vec![LoxType::Number],
				left.lox_type(),
				right.lox_type(),
			)),
		},
		TokenType::Slash => match (&left, &right) {
			(LoxValue::Number(l), LoxValue::Number(r)) => {
				if *r == 0_f64 {
					return Err(Error::RuntimeError(
						op.line,
						expr.to_owned(),
						"Divide by zero".to_string(),
					));
				}
				Ok(LoxValue::Number(l / r))
			}
			(_, LoxValue::Number(_)) => Err(Error::IncompatibleTypes(
				op.line,
				expr.to_owned(),
				right.lox_type(),
				left.lox_type(),
			)),
			(LoxValue::Number(_), _) => Err(Error::IncompatibleTypes(
				op.line,
				expr.to_owned(),
				left.lox_type(),
				right.lox_type(),
			)),
			_ => Err(Error::InoperableTypes(
				op.to_owned(),
				expr.to_owned(),
				vec![LoxType::Number],
				left.lox_type(),
				right.lox_type(),
			)),
		},
		_ => unreachable!("Factor operator: {:?}", op.token_type),
	}
}

fn visit_term_expression(
	expr: &Expr,
	left: LoxValue,
	op: Token,
	right: LoxValue,
) -> Result<LoxValue, Error> {
	match op.token_type.clone() {
		TokenType::Minus => match (&left, &right) {
			(LoxValue::Number(l), LoxValue::Number(r)) => {
				Ok(LoxValue::Number(l - r))
			}
			(_, LoxValue::Number(_)) => Err(Error::IncompatibleTypes(
				op.line,
				expr.to_owned(),
				right.lox_type(),
				left.lox_type(),
			)),
			(LoxValue::Number(_), _) => Err(Error::IncompatibleTypes(
				op.line,
				expr.to_owned(),
				left.lox_type(),
				right.lox_type(),
			)),
			_ => Err(Error::InoperableTypes(
				op.to_owned(),
				expr.to_owned(),
				vec![LoxType::Number],
				left.lox_type(),
				right.lox_type(),
			)),
		},
		TokenType::Plus => match (&left, &right) {
			(LoxValue::Number(l), LoxValue::Number(r)) => {
				Ok(LoxValue::Number(l + r))
			}
			(LoxValue::String(l), LoxValue::String(r)) => {
				Ok(LoxValue::String(l.to_owned() + r.as_str()))
			}
			(LoxValue::Number(_), _) => Err(Error::IncompatibleTypes(
				op.line,
				expr.to_owned(),
				left.lox_type(),
				right.lox_type(),
			)),
			(_, LoxValue::Number(_)) => Err(Error::IncompatibleTypes(
				op.line,
				expr.to_owned(),
				right.lox_type(),
				left.lox_type(),
			)),
			(LoxValue::String(_), _) => Err(Error::IncompatibleTypes(
				op.line,
				expr.to_owned(),
				left.lox_type(),
				right.lox_type(),
			)),
			(_, LoxValue::String(_)) => Err(Error::IncompatibleTypes(
				op.line,
				expr.to_owned(),
				right.lox_type(),
				left.lox_type(),
			)),
			_ => Err(Error::InoperableTypes(
				op.to_owned(),
				expr.to_owned(),
				vec![LoxType::Number, LoxType::String],
				left.lox_type(),
				right.lox_type(),
			)),
		},
		_ => unreachable!("Term operator: {:?}", op.token_type),
	}
}
