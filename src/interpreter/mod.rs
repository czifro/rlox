// use std::fs;
// use std::io;
//
// use dialoguer::theme::ColorfulTheme;
// use dialoguer::*;
//
// use crate::prelude::*;
//
// mod environment;
// mod error;
// mod expression;
// mod parser;
mod token;
mod values;

// use expression::{Decl, /*AstPrinter,*/ Expr, Stmt, Visitor,};
// use parser::Parser;
// use token::{Token, TokenType};
//
// use self::environment::Environment;
//
// pub struct LoxInterpreter {
// 	mode: InterpreterMode,
// 	environment: Environment,
// }
//
// #[derive(PartialEq)]
// enum InterpreterMode {
// 	File,
// 	Repl,
// }
//
// impl Default for InterpreterMode {
// 	fn default() -> Self {
// 		Self::Repl
// 	}
// }
//
// impl LoxInterpreter {
// 	pub fn new() -> Self {
// 		Self {
// 			mode: InterpreterMode::Repl,
// 			environment: Environment::default(),
// 		}
// 	}
//
// 	pub fn launch(&mut self) -> io::Result<()> {
// 		println!("Welcome to Lox interpreter!");
// 		let run_modes = &["Run File", "Run REPL"];
//
// 		let run_mode = Select::with_theme(&ColorfulTheme::default())
// 			.with_prompt("Choose run mode")
// 			.default(0)
// 			.items(&run_modes[..])
// 			.interact()
// 			.unwrap();
//
// 		match run_modes[run_mode] {
// 			"Run File" => self.run_file(),
// 			"Run REPL" => self.run_repl(),
// 			x => unreachable!("Unknown option: {x}"),
// 		}
// 	}
//
// 	fn run_file(&mut self) -> io::Result<()> {
// 		self.mode = InterpreterMode::File;
//
// 		let paths = fs::read_dir("./examples")?
// 			.map(|p| p.unwrap().path().to_string_lossy().to_string())
// 			.collect::<Vec<String>>();
//
// 		let path =
// 			FuzzySelect::with_theme(&ColorfulTheme::default())
// 				.with_prompt("Choose a Lox file to interpret")
// 				.default(0)
// 				.items(&paths[..])
// 				.interact()
// 				.unwrap();
//
// 		let lox_source: String =
// 			String::from_utf8_lossy(&fs::read(paths[path].as_str())?)
// 				.to_string();
//
// 		let now = std::time::Instant::now();
// 		self.run(lox_source)?;
// 		let elapsed_time = now.elapsed();
//
// 		println!("");
// 		println!(
// 			"Executed in {:} microseconds",
// 			elapsed_time.as_micros()
// 		);
//
// 		Ok(())
// 	}
//
// 	fn run_repl(&mut self) -> io::Result<()> {
// 		println!("Lox REPL (enter `exit` to quit)");
//
// 		loop {
// 			let input: String =
// 				Input::with_theme(&ColorfulTheme::default())
// 					.with_prompt("> ")
// 					.interact_text()
// 					.unwrap();
//
// 			match input.trim().to_lowercase().as_str() {
// 				"exit" => break,
// 				_ => match self.run(input) {
// 					Err(e) => eprintln!("{e}"),
// 					_ => {}
// 				},
// 			};
// 		}
//
// 		Ok(())
// 	}
//
// 	fn run(&mut self, lox_source: String) -> io::Result<()> {
// 		let tokens = Token::tokenize(lox_source);
//
// 		let errs = tokens
// 			.clone()
// 			.iter()
// 			.filter(|r| r.is_err())
// 			.map(|r| r.clone().err().unwrap())
// 			.map(|e| format!("{e}"))
// 			.fold(String::default(), |l, r| l + r.as_str());
//
// 		if errs.len() > 0 {
// 			return Err(io::Error::new(
// 				io::ErrorKind::Other,
// 				format!(
// 					"Errors occurred while tokenizing source:\n{errs}"
// 				),
// 			));
// 		}
//
// 		let tokens = tokens
// 			.iter()
// 			.map(|r| r.clone().unwrap())
// 			.filter(|t| t.token_type != TokenType::Whitespace)
// 			.collect::<Vec<Token>>();
//
// 		// let printer = AstPrinter;
// 		let mut parser = Parser::new(tokens);
// 		let exprs = parser.parse();
//
// 		for expr in exprs.iter() {
// 			match expr {
// 				Ok(expr) => {
// 					// println!("Expression: {:}", expr.accept(&printer).unwrap());
// 					match expr.accept(self) {
// 						Ok(output) => match self.mode {
// 							InterpreterMode::Repl => println!("{output}"),
// 							_ => {}
// 						},
// 						Err(e) => eprintln!("{e}"),
// 					};
// 				}
// 				Err(e) => eprintln!("{e}"),
// 			};
// 		}
//
// 		Ok(())
// 	}
// }
//
// impl Visitor<values::LoxValue, Decl> for LoxInterpreter {
// 	fn visit(
// 		&mut self,
// 		decl: &Decl,
// 	) -> Result<values::LoxValue, error::Error> {
// 		use values::LoxValue;
// 		match decl {
// 			Decl::Declaration(tok, expr) => {
// 				let value = match expr {
// 					Some(expr) => expr.accept(self)?,
// 					None => LoxValue::Nil,
// 				};
// 				self
// 					.environment
// 					.define(tok.lexeme.to_owned(), value.clone());
// 				Ok(value)
// 			}
// 			Decl::Statement(expr) => expr.accept(self),
// 		}
// 	}
// }
//
// impl Visitor<values::LoxValue, Stmt> for LoxInterpreter {
// 	fn visit(
// 		&mut self,
// 		stmt: &Stmt,
// 	) -> Result<values::LoxValue, error::Error> {
// 		use values::{LoxClass, LoxValue};
// 		match stmt {
// 			Stmt::Expression(e) => e.accept(self),
// 			Stmt::If(tok, ie, s, ee) => {
// 				let cond = ie.accept(self)?;
// 				if cond.lox_type() != LoxClass::Bool {
// 					return error::Error::RuntimeError(
// 						tok.line,
// 						ie.to_owned(),
// 						"Expected condition to resolve to boolean value"
// 							.to_string(),
// 					)
// 					.to_result();
// 				}
// 				if cond == LoxValue::Bool(true) {
// 					return s.accept(self);
// 				}
//
// 				match ee {
// 					None => Ok(LoxValue::Nil),
// 					Some(ee) => ee.accept(self),
// 				}
// 			}
// 			Stmt::Print(e) => {
// 				let e = e.accept(self)?;
// 				println!("{e}");
// 				Ok(LoxValue::Nil)
// 			}
// 			Stmt::Block(decls) => {
// 				self.environment.create_enclosing();
// 				for d in decls.iter() {
// 					let output = match d.accept(self) {
// 						Ok(output) => output,
// 						err => {
// 							self.environment.drop_enclosing();
// 							return err;
// 						}
// 					};
// 					if self.mode == InterpreterMode::Repl {
// 						println!("{output}");
// 					}
// 				}
// 				self.environment.drop_enclosing();
// 				Ok(LoxValue::Nil)
// 			}
// 		}
// 	}
// }
//
// impl Visitor<values::LoxValue, Expr> for LoxInterpreter {
// 	fn visit(
// 		&mut self,
// 		expr: &Expr,
// 	) -> Result<values::LoxValue, error::Error> {
// 		use values::{LoxClass, LoxValue};
// 		match expr {
// 			Expr::Literal(tok) => Ok(LoxValue::from(tok.clone())),
// 			Expr::Identifier(tok) => {
// 				match self.environment.get(&tok.lexeme) {
// 					Some(v) => Ok(v.clone()),
// 					_ => error::Error::RuntimeError(
// 						tok.line,
// 						expr.to_owned(),
// 						format!("Undefined variable: {:}", tok.lexeme),
// 					)
// 					.to_result(),
// 				}
// 			}
// 			Expr::Assign(ident, sub_expr) => {
// 				let value = sub_expr.accept(self)?;
// 				let _ = self
// 					.environment
// 					.update(ident.lexeme.to_owned(), value)
// 					.ok_or(error::Error::UndefinedVariable(
// 						ident.line,
// 						ident.lexeme.clone(),
// 					))?;
// 				Ok(LoxValue::Nil)
// 			}
// 			Expr::Grouping(sub_expr) => sub_expr.accept(self),
// 			Expr::Unary(op, sub_expr) => match op.token_type.clone() {
// 				TokenType::Minus => {
// 					let output = sub_expr.accept(self)?;
// 					match output {
// 						LoxValue::Number(f) => {
// 							Ok(LoxValue::Number(-1.0_f64 * f))
// 						}
// 						_ => Err(error::Error::WrongType(
// 							op.line,
// 							expr.to_owned(),
// 							output.lox_type(),
// 							LoxClass::Number,
// 						)),
// 					}
// 				}
// 				TokenType::Bang => {
// 					let output = sub_expr.accept(self)?;
// 					match output {
// 						LoxValue::Bool(b) => Ok(LoxValue::Bool(!b)),
// 						_ => error::Error::WrongType(
// 							op.line,
// 							expr.to_owned(),
// 							output.lox_type(),
// 							LoxClass::Bool,
// 						)
// 						.to_result(),
// 					}
// 				}
// 				_ => unreachable!("Unary operator: {:?}", op.token_type),
// 			},
// 			Expr::Binary(left, op, right) => {
// 				expression::visit_binary_expression(
// 					expr,
// 					left.clone(),
// 					op.clone(),
// 					right.clone(),
// 					self,
// 				)
// 			}
// 			Expr::Logical(left, op, right) => {
// 				let left_output = left.accept(self)?;
// 				if left_output.lox_type() != LoxClass::Bool {
// 					return error::Error::WrongType(
// 						op.line,
// 						left.to_owned(),
// 						left_output.lox_type(),
// 						LoxClass::Bool,
// 					)
// 					.to_result();
// 				}
// 				let right_output = right.accept(self)?;
// 				if right_output.lox_type() != LoxClass::Bool {
// 					return error::Error::WrongType(
// 						op.line,
// 						right.to_owned(),
// 						right_output.lox_type(),
// 						LoxClass::Bool,
// 					)
// 					.to_result();
// 				}
//
// 				Ok(LoxValue::Bool(left_output || right_output))
// 			}
// 		}
// 	}
// }
