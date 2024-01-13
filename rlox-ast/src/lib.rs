use rlox_lexer::Token;

generate_ast! [
		{
				Expr {
						Assign(Token, Box<Expr>),
						Binary(Box<Expr>, Token, Box<Expr>),
						Grouping(Box<Expr>),
						Literal(Token),
						Logical(Box<Expr>, Token, Box<Expr>),
						Unary(Token, Box<Expr>),
						Identifier(Identifier),
				}
		},
		{
		  Identifier( Token )
	  },
		{
				Stmt {
						Expression(Expr),
						If(Token, Expr, Box<Stmt>, Option<Box<Stmt>>),
						Print(Expr),
						Block(Block),
				}
		},
		{
				Block ( Vec<Decl> )
		},
		{
				Decl {
						Declaration(Token, Option<Expr>),
						Statement(Stmt),
				}
		}
];

pub trait Visitor<E> {
	type Output;
	type Error;

	fn visit(&mut self, expr: &E) -> Result<Self::Output, Self::Error>;
}

#[derive(Default)]
pub struct AstPrinter;

impl Visitor<Decl> for AstPrinter {
	type Output = String;
	type Error = Infallible;

	fn visit(&mut self, expr: &Decl) -> Result<Self::Output, Self::Error> {
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

impl Visitor<Stmt> for AstPrinter {
	type Output = String;
	type Error = Infallible;

	fn visit(&mut self, expr: &Stmt) -> Result<Self::Output, Self::Error> {
		match expr {
			Stmt::Expression(e) => e.accept(self),
			Stmt::If(_, ie, s, ee) => {
				let ie = ie.accept(self)?;
				let s = s.accept(self)?;
				let ee = match ee {
					Some(ee) => format!("\nelse {:}", ee.accept(self)?),
					_ => String::default(),
				};
				Ok(format!("if ({ie}) {s}{ee}"))
			}
			Stmt::Print(e) => {
				let e = e.accept(self)?;
				Ok(format!("print {e}"))
			}
			Stmt::Block(block) => block.accept(self),
		}
	}
}

impl Visitor<Block> for AstPrinter {
	type Output = String;
	type Error = Infallible;

	fn visit(&mut self, expr: &Block) -> Result<Self::Output, Self::Error> {
		let Block(decls) = expr;
		let block = decls
			.iter()
			.map(|decl| {
				let decl = decl.accept(self).unwrap();
				let decl = decl
					.lines()
					.map(|l| format!("  {l}"))
					.collect::<Vec<String>>();
				let decl = decl.join("\n");
				decl
			})
			.collect::<Vec<String>>()
			.join("\n");
		Ok(format!("{{\n{block}\n}}"))
	}
}

impl Visitor<Expr> for AstPrinter {
	type Output = String;
	type Error = Infallible;

	fn visit(&mut self, expr: &Expr) -> Result<Self::Output, Self::Error> {
		let expr = match expr {
			Expr::Assign(ident, sub_expr) => {
				let ident = &ident.lexeme;
				let sub_expr = sub_expr.accept(self).unwrap();
				format!("{ident} = {sub_expr}")
			}
			Expr::Binary(left, op, right) => {
				let op = &op.lexeme;
				let left = left.accept(self).unwrap();
				let right = right.accept(self).unwrap();
				format!("{left} {op} {right}")
			}
			Expr::Grouping(sub_expr) => {
				let sub_expr = sub_expr.accept(self).unwrap();
				format!("({sub_expr})")
			}
			Expr::Literal(lit) => lit.lexeme.clone(),
			Expr::Logical(left, op, right) => {
				let op = &op.lexeme;
				let left = left.accept(self).unwrap();
				let right = right.accept(self).unwrap();
				format!("{left} {op} {right}")
			}
			Expr::Unary(op, sub_expr) => {
				let op = &op.lexeme;
				let sub_expr = sub_expr.accept(self).unwrap();
				format!("{op}{sub_expr}")
			}
			Expr::Identifier(ident) => ident.accept(self).unwrap(),
		};

		Ok(expr)
	}
}

impl Visitor<Identifier> for AstPrinter {
	type Output = String;
	type Error = Infallible;

	fn visit(&mut self, expr: &Identifier) -> Result<Self::Output, Self::Error> {
		let Identifier(ident) = expr;
		Ok(ident.lexeme.clone())
	}
}

macro_rules! impl_accept {
	($expr:ident) => {
		impl $expr {
			pub fn accept<R, E, V>(&self, visitor: &mut V) -> Result<R, E>
			where
				V: Visitor<$expr, Output = R, Error = E>,
			{
				return visitor.visit(self);
			}
		}
	};
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

        impl_accept!($expr);
    };
    {
        $expr:ident ( $val:ty )
    } => {
        ast! {
            pub struct $expr(pub $val);
        }

        impl_accept!($expr);
    };
}

macro_rules! ast {
    ($ast:item) => {
        #[derive(Clone, Debug, PartialEq)]
        $ast
    };
}

use std::convert::Infallible;

pub(crate) use ast;
pub(crate) use generate_ast;
pub(crate) use impl_accept;
