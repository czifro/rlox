use std::{collections::HashMap, marker::PhantomData, rc::Rc};

use rlox_ast::{Block as FunctionBlock, Expr::Logical, Identifier, Visitor};
use rlox_lexer::{Token, TokenType, TokenLiteral};

use crate::{environment::Environment, prelude::*};

pub trait LoxObject: 'static {
}

pub trait LoxType: 'static {
	fn name(&self) -> String;

	fn is(&self, other: Box<dyn LoxType>) -> bool {
		self.name() == other.name()
	}
}

pub struct LoxInstance {
	environment: Environment,
	value: Box<dyn LoxObject>,
	r#type: Box<dyn LoxType>,
}

impl LoxInstance {
	pub fn new(value: Box<dyn LoxObject>, r#type: Box<dyn LoxType>) -> Self {
		let mut environment = Environment::default();

		Self { environment, value, r#type }
	}
}

#[derive(Debug, Clone)]
pub struct LoxClass {
	name: String,
	fields: HashMap<String, Rc<LoxClass>>,
	functions: HashMap<String, Rc<LoxFunction>>,
}

impl LoxClass {
	pub fn bool_def() -> Self {
		let mut bool_class = Self {
			name: "bool".to_string(),
			fields: HashMap::new(),
			functions: HashMap::new(),
		};

		// fields.insert("value".to_string(), Rc::new(LoxClass::bool_def()));

		let other_ident = Token {
			token_type: TokenType::Identifier,
			lexeme: "other".to_string(),
			literal: Some(TokenLiteral::String("other".to_string())),
			line: 1,
		};
		let other_ident = Identifier(other_ident);

		bool_class.functions.insert(
			"and".to_string(),
			Rc::new(LoxFunction {
				name: "and".to_string(),
				associated_type: LoxClass::bool_def(),
				params: vec![(other_ident.clone(), LoxClass::bool_def())],
				block: FunctionBlock(vec![]),
			}),
		);

		bool_class.functions.insert(
			"or".to_string(),
			Rc::new(LoxFunction {
				name: "or".to_string(),
				associated_type: LoxClass::bool_def(),
				params: vec![(other_ident.clone(), LoxClass::bool_def())],
				block: FunctionBlock(vec![]),
			}),
		);

		bool_class.functions.insert(
			"not".to_string(),
			Rc::new(LoxFunction {
				name: "not".to_string(),
				associated_type: LoxClass::bool_def(),
				params: vec![],
				block: FunctionBlock(vec![]),
			}),
		);

		let bool_class = bool_class;

		bool_class
	}
}

impl LoxType for LoxClass {
	fn name(&self) -> String {
		self.name.clone()
	}
}

#[derive(Debug, Clone)]
pub struct LoxFunction {
	name: String,
	associated_type: LoxClass,
	params: Vec<(Identifier, LoxClass)>,
	block: FunctionBlock,
}

impl LoxType for LoxFunction {
	fn name(&self) -> String {
		self.name.clone()
	}
}

impl LoxFunction {
	pub fn param_idents(&self) -> Vec<Identifier> {
		self.params.iter().map(|(i, _)| i.clone()).collect()
	}
}

pub struct LoxBuiltin<T> {
	value: T,
	token: Token,
}

impl LoxObject for LoxBuiltin<bool> {}
impl LoxObject for LoxBuiltin<f64> {}
impl LoxObject for LoxBuiltin<String> {}

impl LoxBuiltin<bool> {
	pub fn new(value: bool, token: Token) -> Self {
		Self { value, token }
	}

	pub fn and(&self, other: &Self) -> Self {
		Self::new(self.value && other.value, self.token.clone())
	}

	pub fn and_expr(&self) -> FunctionBlock {
		FunctionBlock(vec![])
	}
}

// pub trait LoxObjectInvoker<L>: Visitor<L>
// where
// 	L: LoxInvocable,
// {
// 	fn set_environment(values: Vec<(Identifier, Rc<dyn LoxObject>)>);
//
// 	fn invoke(&self, lox_invocable: L) -> Result<Self::Output, Self::Error>;
// }
//
// pub trait LoxInvocable: LoxObject + Sized {
// 	fn call<Invoker>(&self, args: Vec<Rc<dyn LoxObject>>, invoker: Invoker)
// 	where
// 		Invoker: LoxObjectInvoker<Self>;
// }
