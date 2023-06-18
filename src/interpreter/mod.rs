use std::fs;
use std::io;

use dialoguer::theme::ColorfulTheme;
use dialoguer::*;

use crate::prelude::*;

mod error;
mod expression;
mod parser;
mod token;
mod types;

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
					Err(e) => return Err(e),
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

		if tokens
			.clone()
			.iter()
			.filter(|r| r.clone().is_err())
			.map(|r| r.clone())
			.collect::<Vec<Result<Token, error::Error>>>()
			.len() > 0
		{
			return Err(io::Error::new(
				io::ErrorKind::Other,
				"Errors occurred while tokenizing source",
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
			Ok(expr) => println!("Expression: {:}", printer.visit(&expr)),
			Err(e) => eprintln!("{e}"),
		};

		Ok(())
	}
}
