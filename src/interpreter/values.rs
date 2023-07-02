use std::cmp::Ordering;

use crate::interpreter::token::*;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum LoxValue {
	Nil,
	Number(f64),
	Bool(bool),
	String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoxType {
    Nil,
    Number,
    Bool,
    String,
}

impl LoxValue {
    pub fn lox_type(&self) -> LoxType {
        match self {
            Self::Number(_) => LoxType::Number,
            Self::Bool(_) => LoxType::Bool,
            Self::String(_) => LoxType::String,
            Self::Nil => LoxType::Nil,
        }
    }

    pub fn is_same_type(&self, other: &Self) -> bool {
        self.lox_type() == other.lox_type()
    }

    pub fn is_nil(&self) -> bool {
        self.lox_type() == LoxType::Nil
    }
}

impl PartialEq for LoxValue {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Number(lhs), Self::Number(rhs)) => lhs == rhs,
			(Self::String(lhs), Self::String(rhs)) => lhs == rhs,
			(Self::Bool(lhs), Self::Bool(rhs)) => lhs == rhs,
			(Self::Nil, Self::Nil) => true,
			_ => false,
		}
	}
}

impl PartialOrd for LoxValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Number(l), Self::Number(r)) => l.partial_cmp(r),
            (Self::String(l), Self::String(r)) => l.partial_cmp(r),
            _ => None,
        }
    }
}

impl Display for LoxValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Number(n) => write!(f, "{}", n),
			Self::String(s) => write!(f, "{}", s),
			Self::Bool(b) => write!(f, "{}", b),
			Self::Nil => write!(f, "nil"),
		}
	}
}

impl Display for LoxType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
			Self::Number => write!(f, "number"),
			Self::String => write!(f, "string"),
			Self::Bool => write!(f, "bool"),
			Self::Nil => write!(f, "nil"),
        }
    }
}

impl From<Token> for LoxValue {
	fn from(token: Token) -> Self {
		match token.token_type.clone() {
			TokenType::String
			| TokenType::True
			| TokenType::False
			| TokenType::Integer
			| TokenType::Float => Self::from(token.literal.unwrap()),
			TokenType::Nil => Self::Nil,
			_ => unreachable!("Unexpected token type: {:?}", token.token_type),
		}
	}
}

impl From<TokenLiteral> for LoxValue {
	fn from(lit: TokenLiteral) -> Self {
		match lit {
			TokenLiteral::Nil(_) => Self::Nil,
			TokenLiteral::Bool(b) => Self::Bool(b),
			TokenLiteral::String(s) => Self::String(s),
			TokenLiteral::Float(f) => Self::Number(f64::from(f)),
			TokenLiteral::Integer(i) => Self::Number(f64::from(i)),
		}
	}
}
