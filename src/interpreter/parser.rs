use super::{error::Error, expression::Expr, token::*};

pub struct Parser {
	cursor: usize,
	tokens: Vec<Token>,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Self { cursor: 0, tokens }
	}

	pub fn parse(&mut self) -> Result<Expr, Error> {
		if self.is_eof() {
			return Err(Error::Eof);
		}

		let expr = self.expression();
		if expr.is_err() {
			self.synchronize();
		}

		expr
	}

	fn synchronize(&mut self) {
		self.advance();

		loop {
			if self.is_eof() {
				return;
			}
			if self.previous().token_type == TokenType::SemiColon {
				return;
			}
			match self.peek().token_type {
				TokenType::Class
				| TokenType::Fun
				| TokenType::Fn
				| TokenType::Var
				| TokenType::For
				| TokenType::If
				| TokenType::While
				| TokenType::Print
				| TokenType::Return => return,
				_ => self.advance(),
			};
		}
	}

	fn expression(&mut self) -> Result<Expr, Error> {
		self.equality()
	}

	fn equality(&mut self) -> Result<Expr, Error> {
		let mut expr = self.comparison()?;

		loop {
			match self.peek().token_type {
				TokenType::BangEqual | TokenType::EqualEqual => {
					let token = self.advance().clone();
					let right = self.comparison()?;
					expr = Expr::Binary(Box::from(expr), token, Box::from(right))
				}
				_ => break,
			};
		}

		Ok(expr)
	}

	fn comparison(&mut self) -> Result<Expr, Error> {
		let mut expr = self.term()?;

		loop {
			match self.peek().token_type {
				TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual => {
					let token = self.advance().clone();
					let right = self.term()?;
					expr = Expr::Binary(Box::from(expr), token.clone(), Box::from(right))
				}
				_ => break,
			};
		}

		Ok(expr)
	}

	fn term(&mut self) -> Result<Expr, Error> {
		let mut expr = self.factor()?;

		loop {
			match self.peek().token_type {
				TokenType::Plus | TokenType::Minus => {
					let token = self.advance().clone();
					let right = self.factor()?;
					expr = Expr::Binary(Box::from(expr), token, Box::from(right))
				}
				_ => break,
			};
		}

		Ok(expr)
	}

	fn factor(&mut self) -> Result<Expr, Error> {
		let mut expr = self.unary()?;

		loop {
			match self.peek().token_type {
				TokenType::Star | TokenType::Slash => {
					let token = self.advance().clone();
					let right = self.unary()?;
					expr = Expr::Binary(Box::from(expr), token, Box::from(right))
				}
				_ => break,
			};
		}

		Ok(expr)
	}

	fn unary(&mut self) -> Result<Expr, Error> {
		match self.peek().token_type {
			TokenType::Minus | TokenType::Bang => {
				let token = self.advance().clone();
				let right = self.unary()?;
				Ok(Expr::Unary(token, Box::from(right)))
			}
			_ => self.primary(),
		}
	}

	fn primary(&mut self) -> Result<Expr, Error> {
		if self.is_eof() {
			return Err(Error::UnexpectedEof(self.peek().line));
		}

		match self.peek().token_type {
			TokenType::True
			| TokenType::False
			| TokenType::Nil
			| TokenType::Integer
			| TokenType::Float
			| TokenType::String => Ok(Expr::Literal(self.advance().clone())),
			TokenType::LeftParen => {
				self.advance();
				let expr = self.expression()?;
				match self.peek().token_type {
					TokenType::RightParen => {
						self.advance();
						Ok(Expr::Grouping(Box::from(expr)))
					}
					_ => {
						let token = self.peek();
						Err(Error::WrongTokenType(
							token.clone().line,
							token.clone().lexeme,
							")".to_string(),
						))
					}
				}
			}
			_ => {
				let token = self.peek();
				Err(Error::WrongTokenType(
					token.clone().line,
					token.clone().lexeme,
					"(".to_string(),
				))
			}
		}
	}

	fn peek(&self) -> &Token {
		self.tokens.get(self.cursor).unwrap()
	}

	fn previous(&self) -> &Token {
		self.tokens.get(self.cursor - 1).unwrap()
	}

	fn is_eof(&self) -> bool {
		self.peek().token_type == TokenType::Eof
	}

	fn advance(&mut self) -> &Token {
		if !self.is_eof() {
			self.cursor = self.cursor + 1;
		}
		self.previous()
	}
}
