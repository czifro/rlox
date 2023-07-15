use super::{error::Error, token::Token};

generate_ast! [
		{
				Expr {
						Binary(Box<Expr>, Token, Box<Expr>),
						Grouping(Box<Expr>),
						Literal(Token),
						Unary(Token, Box<Expr>),
						Identifier(Token),
				}
		},
		{
				Stmt {
						Expression(Expr),
						Print(Expr),
				}
		},
		{
				Decl {
						Declaration(Token, Option<Expr>),
						Statement(Stmt),
				}
		}
];

pub trait Visitor<R, E> {
	fn visit(&mut self, expr: &E) -> Result<R, Error>;
}

pub struct AstPrinter;

impl Visitor<String, Decl> for AstPrinter {
	fn visit(&mut self, expr: &Decl) -> Result<String, Error> {
		match expr {
			Decl::Declaration(token, Some(e)) => {
				let e = e.accept(self).unwrap();
				let ident = token.clone().lexeme;
				Ok(format!("var {ident} = {e};"))
			}
			Decl::Declaration(token, None) => {
				let ident = token.clone().lexeme;
				Ok(format!("var {ident};"))
			}
			Decl::Statement(s) => {
				let s = s.accept(self)?;
				Ok(format!("print {s}"))
			}
		}
	}
}

impl Visitor<String, Stmt> for AstPrinter {
	fn visit(&mut self, expr: &Stmt) -> Result<String, Error> {
		match expr {
			Stmt::Expression(e) => e.accept(self),
			Stmt::Print(e) => {
				let e = e.accept(self)?;
				Ok(format!("print {e}"))
			}
		}
	}
}

impl Visitor<String, Expr> for AstPrinter {
	fn visit(&mut self, expr: &Expr) -> Result<String, Error> {
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
			Expr::Identifier(ident) => ident.lexeme.clone(),
		};

		Ok(expr)
	}
}

macro_rules! generate_ast {
  [$($def:tt),*] => {
    $(
      generate_ast! $def
    )*
  };
    {
        $expr:ident {$($case:tt)*}
    } => {
        ast! {
            pub enum $expr { $($case)* }
        }

        impl $expr {
            pub fn accept<R, V>(&self, visitor: &mut V) -> Result<R, Error>
            where
                V: Visitor<R, $expr>,
            {
                return visitor.visit(self);
            }
        }
    };
}

macro_rules! ast {
	($ast:item) => {
		#[derive(Clone, Debug, PartialEq)]
		$ast
	};
}

pub(crate) use ast;
pub(crate) use generate_ast;
