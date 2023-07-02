use std::fs;
use std::io;

use dialoguer::theme::ColorfulTheme;
use dialoguer::*;

use crate::prelude::*;

mod error;
mod expression;
mod parser;
mod token;
mod values;

use expression::{AstPrinter, Expr, Visitor};
use parser::Parser;
use token::{Token, TokenType};

pub struct LoxInterpreter {}

impl LoxInterpreter {
	pub fn new() -> Self {
		Self {}
	}

	pub fn launch(&self) -> io::Result<()> {
		println!("Welcome to Lox interpreter!");
		let run_modes = &["Run File", "Run REPL"];

		let run_mode = Select::with_theme(&ColorfulTheme::default())
			.with_prompt("Choose run mode")
			.default(0)
			.items(&run_modes[..])
			.interact()
			.unwrap();

		match run_modes[run_mode] {
			"Run File" => self.run_file(),
			"Run REPL" => self.run_repl(),
			x => unreachable!("Unknown option: {x}"),
		}
	}

	fn run_file(&self) -> io::Result<()> {
		let paths = fs::read_dir("./examples")?
			.map(|p| p.unwrap().path().to_string_lossy().to_string())
			.collect::<Vec<String>>();

		let path = FuzzySelect::with_theme(&ColorfulTheme::default())
			.with_prompt("Choose a Lox file to interpret")
			.default(0)
			.items(&paths[..])
			.interact()
			.unwrap();

		let lox_source: String = String::from_utf8_lossy(&fs::read(paths[path].as_str())?).to_string();

		self.run(lox_source)
	}

	fn run_repl(&self) -> io::Result<()> {
		println!("Lox REPL (enter `exit` to quit)");

		loop {
			let input: String = Input::with_theme(&ColorfulTheme::default())
				.with_prompt("> ")
				.interact_text()
				.unwrap();

			match input.trim().to_lowercase().as_str() {
				"exit" => break,
				_ => match self.run(input) {
				    Err(e) => eprintln!("{e}"),
					_ => {}
				},
			};
		}

		Ok(())
	}

	fn run(&self, lox_source: String) -> io::Result<()> {
		let tokens = Token::tokenize(lox_source);

		for token in tokens.clone().iter() {
			match token {
				Ok(t) => println!("{:?}", t),
				Err(e) => eprintln!("{e}"),
			}
		}

		let errs = tokens
			.clone()
			.iter()
			.filter(|r| r.clone().is_err())
			.map(|r| r.clone().err().unwrap())
            .map(|e| format!("{e}"))
            .fold(String::default(), |l, r| l + r.as_str());

		if errs
			.len() > 0
		{
			return Err(io::Error::new(
				io::ErrorKind::Other,
				format!("Errors occurred while tokenizing source:\n{errs}"),
			));
		}

		let tokens = tokens
			.iter()
			.map(|r| r.clone().unwrap())
			.filter(|t| t.token_type != TokenType::Whitespace)
			.collect::<Vec<Token>>();

		let mut parser = Parser::new(tokens);
		let printer = AstPrinter;

		match parser.parse() {
			Ok(expr) => {
				println!("Expression: {:}", printer.visit(&expr).unwrap());
				match self.visit(&expr) {
					Ok(output) => println!("Output: {:}", output),
					Err(e) => eprintln!("{e}"),
				};
			}
			Err(e) => eprintln!("{e}"),
		};

		Ok(())
	}
}

impl Visitor<values::LoxValue> for LoxInterpreter {
	fn visit(&self, expr: &Expr) -> Result<values::LoxValue, error::Error> {
		use values::{LoxType, LoxValue};
		match expr {
			Expr::Literal(tok) => Ok(LoxValue::from(tok.clone())),
			Expr::Grouping(sub_expr) => sub_expr.accept(self),
			Expr::Unary(op, sub_expr) => match op.token_type.clone() {
				TokenType::Minus => {
					let output = sub_expr.accept(self)?;
					match output {
						LoxValue::Number(f) => Ok(LoxValue::Number(-1.0_f64 * f)),
						_ => Err(error::Error::WrongType(
							op.line,
							expr.to_owned(),
							output.lox_type(),
							LoxType::Number,
						)),
					}
				}
				TokenType::Bang => {
					let output = sub_expr.accept(self)?;
					match output {
						LoxValue::Bool(b) => Ok(LoxValue::Bool(!b)),
						_ => Err(error::Error::WrongType(
							op.line,
							expr.to_owned(),
							output.lox_type(),
							LoxType::Bool,
						)),
					}
				}
				_ => unreachable!("Unary operator: {:?}", op.token_type),
			},
			Expr::Binary(left, op, right) => {
				let left = left.accept(self)?;
				let right = right.accept(self)?;
				if !left.is_nil() && !right.is_nil() && !left.is_same_type(&right) {
					return Err(error::Error::IncompatibleTypes(
						op.line,
						expr.to_owned(),
						left.lox_type(),
						right.lox_type(),
					));
				}
				match op.token_type.clone() {
					TokenType::BangEqual => Ok(LoxValue::Bool(left != right)),
					TokenType::EqualEqual => Ok(LoxValue::Bool(left == right)),
					TokenType::Greater => {
						if left.is_nil() || right.is_nil() || !left.is_same_type(&right) {
							return Err(error::Error::IncompatibleTypes(
								op.line,
								expr.to_owned(),
								left.lox_type(),
								right.lox_type(),
							));
						}
						Ok(LoxValue::Bool(left > right))
					}
					TokenType::GreaterEqual => {
						if left.is_nil() || right.is_nil() || !left.is_same_type(&right) {
							return Err(error::Error::IncompatibleTypes(
								op.line,
								expr.to_owned(),
								left.lox_type(),
								right.lox_type(),
							));
						}
						Ok(LoxValue::Bool(left >= right))
					}
					TokenType::Less => {
						if left.is_nil() || right.is_nil() || !left.is_same_type(&right) {
							return Err(error::Error::IncompatibleTypes(
								op.line,
								expr.to_owned(),
								left.lox_type(),
								right.lox_type(),
							));
						}
						Ok(LoxValue::Bool(left < right))
					}
					TokenType::LessEqual => {
						if left.is_nil() || right.is_nil() || !left.is_same_type(&right) {
							return Err(error::Error::IncompatibleTypes(
								op.line,
								expr.to_owned(),
								left.lox_type(),
								right.lox_type(),
							));
						}
						Ok(LoxValue::Bool(left <= right))
					}
					TokenType::Star => match (&left, &right) {
						(LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Number(l * r)),
                        (LoxValue::Nil, LoxValue::Nil) => Err(error::Error::IncompatibleTypes(
							op.line,
							expr.to_owned(),
							left.lox_type(),
                            LoxType::Number,
						)),
                        (_, LoxValue::Number(_)) => Err(error::Error::IncompatibleTypes(
							op.line,
							expr.to_owned(),
							right.lox_type(),
							left.lox_type(),
						)),
                        (LoxValue::Number(_), _) => Err(error::Error::IncompatibleTypes(
							op.line,
							expr.to_owned(),
							left.lox_type(),
							right.lox_type(),
						)),
						_ => Err(error::Error::IncompatibleTypes(
							op.line,
							expr.to_owned(),
							left.lox_type(),
                            LoxType::Number
						)),
					},
					TokenType::Slash => match (&left, &right) {
						(LoxValue::Number(l), LoxValue::Number(r)) => {
							if *r == 0_f64 {
								return Err(error::Error::RuntimeError(
									op.line,
									expr.to_owned(),
									"Divide by zero".to_string(),
								));
							}
							Ok(LoxValue::Number(l / r))
						}
                        (LoxValue::Nil, LoxValue::Nil) => Err(error::Error::IncompatibleTypes(
							op.line,
							expr.to_owned(),
							left.lox_type(),
                            LoxType::Number,
						)),
                        (_, LoxValue::Number(_)) => Err(error::Error::IncompatibleTypes(
							op.line,
							expr.to_owned(),
							right.lox_type(),
							left.lox_type(),
						)),
                        (LoxValue::Number(_), _) => Err(error::Error::IncompatibleTypes(
							op.line,
							expr.to_owned(),
							left.lox_type(),
							right.lox_type(),
						)),
						_ => Err(error::Error::IncompatibleTypes(
							op.line,
							expr.to_owned(),
							left.lox_type(),
                            LoxType::Number
						)),
					},
					TokenType::Minus => match (&left, &right) {
						(LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Number(l - r)),
                        (LoxValue::Nil, LoxValue::Nil) => Err(error::Error::IncompatibleTypes(
							op.line,
							expr.to_owned(),
							left.lox_type(),
                            LoxType::Number,
						)),
                        (_, LoxValue::Number(_)) => Err(error::Error::IncompatibleTypes(
							op.line,
							expr.to_owned(),
							right.lox_type(),
							left.lox_type(),
						)),
                        (LoxValue::Number(_), _) => Err(error::Error::IncompatibleTypes(
							op.line,
							expr.to_owned(),
							left.lox_type(),
							right.lox_type(),
						)),
						_ => Err(error::Error::IncompatibleTypes(
							op.line,
							expr.to_owned(),
							left.lox_type(),
                            LoxType::Number
						)),
					},
					TokenType::Plus => match (&left, &right) {
						(LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Number(l + r)),
						(LoxValue::String(l), LoxValue::String(r)) => {
							Ok(LoxValue::String(l.to_owned() + r.as_str()))
						}
                        (LoxValue::Nil, LoxValue::Nil) => Err(error::Error::IncompatibleTypes(
							op.line,
							expr.to_owned(),
							left.lox_type(),
                            LoxType::Number,
						)),
                        (LoxValue::Number(_), _) => Err(error::Error::IncompatibleTypes(
							op.line,
							expr.to_owned(),
							left.lox_type(),
							right.lox_type(),
						)),
                        (_, LoxValue::Number(_)) => Err(error::Error::IncompatibleTypes(
							op.line,
							expr.to_owned(),
							right.lox_type(),
							left.lox_type(),
						)),
                        (LoxValue::String(_), _) => Err(error::Error::IncompatibleTypes(
							op.line,
							expr.to_owned(),
							left.lox_type(),
							right.lox_type(),
						)),
                        (_, LoxValue::String(_)) => Err(error::Error::IncompatibleTypes(
							op.line,
							expr.to_owned(),
							right.lox_type(),
							left.lox_type(),
						)),
						_ => Err(error::Error::IncompatibleTypes(
							op.line,
							expr.to_owned(),
							left.lox_type(),
							right.lox_type(),
						)),
					},
					_ => unreachable!("Binary operator: {:?}", op.token_type),
				}
			}
		}
	}
}
