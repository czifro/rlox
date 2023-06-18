use super::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
	Binary(Box<Expr>, Token, Box<Expr>),
	Grouping(Box<Expr>),
	Literal(Token),
	Unary(Token, Box<Expr>),
}

pub trait Visitor<R> {
	fn visit(&self, expr: &Expr) -> R;
}

impl Expr {
	pub fn accept<R, V>(&self, visitor: &V) -> R
	where
		V: Visitor<R>,
	{
		return visitor.visit(self);
	}
}

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
	fn visit(&self, expr: &Expr) -> String {
		match expr {
			Expr::Binary(left, op, right) => {
				let left = left.accept(self);
				let right = right.accept(self);
				format!("{left} {:} {right}", op.lexeme)
			}
			Expr::Grouping(sub_expr) => {
				let sub_expr = sub_expr.accept(self);
				format!("({sub_expr})")
			}
			Expr::Literal(lit) => lit.lexeme.clone(),
			Expr::Unary(op, sub_expr) => {
				let sub_expr = sub_expr.accept(self);
				format!("{:}{sub_expr}", op.lexeme)
			}
		}
	}
}

