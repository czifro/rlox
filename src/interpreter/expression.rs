use super::{token::Token, error::Error};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
	Binary(Box<Expr>, Token, Box<Expr>),
	Grouping(Box<Expr>),
	Literal(Token),
	Unary(Token, Box<Expr>),
}

pub trait Visitor<R> {
	fn visit(&self, expr: &Expr) -> Result<R, Error>;
}

impl Expr {
	pub fn accept<R, V>(&self, visitor: &V) -> Result<R, Error>
	where
		V: Visitor<R>,
	{
		return visitor.visit(self);
	}
}

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
	fn visit(&self, expr: &Expr) -> Result<String, Error> {
		let expr = match expr {
			Expr::Binary(left, op, right) => {
				let left = left.accept(self).unwrap();
				let right = right.accept(self).unwrap();
				format!("{left} {:} {right}", op.lexeme)
			}
			Expr::Grouping(sub_expr) => {
				let sub_expr = sub_expr.accept(self).unwrap();
				format!("({sub_expr})")
			}
			Expr::Literal(lit) => lit.lexeme.clone(),
			Expr::Unary(op, sub_expr) => {
				let sub_expr = sub_expr.accept(self).unwrap();
				format!("{:}{sub_expr}", op.lexeme)
			}
		};

        Ok(expr)
	}
}

