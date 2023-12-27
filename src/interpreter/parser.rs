use super::{
	error::Error,
	expression::{AstPrinter, Decl, Expr, Stmt},
	token::*,
};

pub struct Parser {
	cursor: i32,
	tokens: Vec<Token>,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Self { cursor: 0, tokens }
	}

	pub fn parse(&mut self) -> Vec<Result<Decl, Error>> {
		let mut exprs: Vec<Result<Decl, Error>> = Vec::default();

		loop {
			if self.is_eof() {
				break;
			}

			let expr = self.declaration();
			if expr.is_err() {
				self.synchronize();
			}

			exprs.push(expr);
		}

		exprs
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

	fn declaration(&mut self) -> Result<Decl, Error> {
		match self.peek().token_type {
			TokenType::Var => {
				self.advance();
				match self.peek().token_type.clone() {
					TokenType::Identifier => {
						let ident = self.advance().clone();
						match self.peek().token_type.clone() {
							TokenType::SemiColon => {
								let res = Ok(Decl::Declaration(ident.to_owned(), None));
								self.advance();
								res
							}
							TokenType::Equal => {
								self.advance();
								let expr = self.expression()?;
								match self.peek().token_type.clone() {
									TokenType::SemiColon | TokenType::Eof => {
										self.advance();
										Ok(Decl::Declaration(ident.to_owned(), Some(expr)))
									}
									token_type => {
										let res = Err(Error::UnexpectedToken(
											self.peek().line,
											format!(
												"Found {:?}, expected {:?}",
												token_type,
												TokenType::Identifier
											),
										));
										res
									}
								}
							}
							token_type => {
								let res = Err(Error::UnexpectedToken(
									self.peek_offset(1).line,
									format!(
										"Found {:?}, expected {:?}",
										token_type,
										TokenType::Identifier
									),
								));
								self.advance();
								res
							}
						}
					}
					token_type => {
						let res = Err(Error::UnexpectedToken(
							self.peek().line,
							format!(
								"Found {:?}, expected {:?}",
								token_type,
								TokenType::Identifier
							),
						));
						res
					}
				}
			}
			_ => {
				let stmt = self.statement();
				if self.peek().token_type == TokenType::SemiColon {
					self.advance();
				}
				stmt.map(Decl::Statement)
			}
		}
	}

	fn statement(&mut self) -> Result<Stmt, Error> {
		match self.peek().token_type {
			TokenType::Print => {
				self.advance();
				let expr = self.expression()?;
				if self.peek().token_type == TokenType::SemiColon {
					self.advance();
				}
				Ok(Stmt::Print(expr))
			}
			TokenType::LeftBrace => {
				self.advance();
				match self.peek().token_type {
					TokenType::RightBrace => Ok(Stmt::Block(Vec::default())),
					_ => {
						let mut decls = vec![];
						while self.peek().token_type != TokenType::RightBrace {
							if self.is_eof() {
								return Err(Error::UnexpectedEof(self.peek().line));
							}
							let decl = self.declaration()?;
							decls.push(decl);
						}
						self.advance();
						Ok(Stmt::Block(decls))
					}
				}
			}
			TokenType::If => {
				let tok = self.peek().clone();
				self.advance();
				if self.peek().token_type != TokenType::LeftParen {
					return Err(Error::WrongTokenType(
						self.peek().line,
						self.peek().lexeme.clone(),
						"(".to_string(),
					));
				}
				self.advance();
				let expr = self.expression()?;
				if self.peek().token_type != TokenType::RightParen {
					return Err(Error::WrongTokenType(
						self.peek().line,
						self.peek().lexeme.clone(),
						")".to_string(),
					));
				}
				self.advance();
				let stmt = self.statement()?;
				if self.peek().token_type != TokenType::Else {
					return Ok(Stmt::If(tok, expr, Box::from(stmt), None));
				}
				self.advance();
				let else_stmt = self.statement()?;

				Ok(Stmt::If(
					tok,
					expr,
					Box::from(stmt),
					Some(Box::from(else_stmt)),
				))
			}
			_ => {
				let expr = self.expression()?;
				if self.peek().token_type == TokenType::SemiColon {
					self.advance();
				}
				Ok(Stmt::Expression(expr))
			}
		}
	}

	fn expression(&mut self) -> Result<Expr, Error> {
		self.assignment()
	}

	fn assignment(&mut self) -> Result<Expr, Error> {
		let mut expr = self.equality()?;

		if self.advance().token_type == TokenType::Equal {
			let prev_cursor = self.cursor - 1;
			let value = self.assignment()?;

			match expr {
				Expr::Identifier(tok) => expr = Expr::Assign(tok, Box::from(value)),
				_ => {
					let tok = self.peek_offset(prev_cursor - self.cursor); // Peeking back to equals token
					let mut printer = AstPrinter::default();
					let expr = expr.accept(&mut printer).unwrap();
					return Err(Error::InvalidAssignmentTarget(tok.line, expr));
				}
			}
		} else {
			self.shift_cursor(-1);
		}

		Ok(expr)
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
			TokenType::Identifier => Ok(Expr::Identifier(self.advance().clone())),
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

	fn peek_offset(&self, offset: i32) -> &Token {
		self.tokens.get((self.cursor + offset) as usize).unwrap()
	}

	fn peek(&self) -> &Token {
		self.peek_offset(0)
	}

	fn previous(&self) -> &Token {
		self.peek_offset(-1)
	}

	fn is_eof(&self) -> bool {
		self.peek().token_type == TokenType::Eof
	}

	fn shift_cursor(&mut self, offset: i32) {
		self.cursor = self.cursor + offset;
	}

	fn advance(&mut self) -> &Token {
		if !self.is_eof() {
			self.shift_cursor(1);
		}

		self.previous()
	}
}
