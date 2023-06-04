pub mod macros;
pub mod parser;
pub mod precedence;
// pub mod value;

use std::marker::PhantomData;

use combine::*;
use combine::parser::function;
use combine::{
  parser::{
    choice,
    token::{
      satisfy,
      satisfy_map,
    },
  },
  stream::*,
};

use crate::interpreter::types::*;
use crate::interpreter::token::*;
use macros::generate_ast;
// use parser::*;
use precedence::*;
// use value::*;

/// Top-level trait for all expressions that represent
/// the Lox grammar.
pub trait LoxExpression {}

pub trait LoxValueExpression {
  type Value: LoxValue;
}

impl<E> LoxValueExpression for Box<E>
where
  E: LoxValueExpression,
{
  type Value = E::Value;
}

impl<E> LoxExpression for E
where
  E: LoxValueExpression,
{}

pub trait BoxedLoxExpression {
  type Expression: LoxExpression;
  
  fn unbox(self) -> Self::Expression;
}

impl<E> BoxedLoxExpression for Box<E>
where
  E: LoxExpression,
{
  type Expression = E;
  
  fn unbox(self) -> Self::Expression {
    *self
  }
}

pub trait IntoBoxedLoxExpression: LoxExpression {
  fn into_boxed(self) -> Box<Self>;
}

impl<E> IntoBoxedLoxExpression for E
where
  E: LoxExpression,
{
  fn into_boxed(self) -> Box<Self> {
    Box::from(self)
  }
}

/// Top-level struct for handling any expression
/// within the Lox grammar.
#[derive(Debug, Clone)]
pub struct Expression<E: LoxExpression> {
  expression: Box<E>,
}

impl<E> Expression<E>
where
  E: LoxExpression,
{
  pub fn new(expression: E) -> Self {
    Self { expression: expression.into_boxed() }
  }
  pub fn inner(self) -> E {
    self.expression.unbox()
  }
}

impl<E> LoxExpression for Expression<E>
where
  E: LoxExpression
{
}

pub trait LoxEvaluator<E>
where
  E: LoxExpression
{
  type Ok;
  
  type Error: std::error::Error;
  
  fn evaluate(self, expr: E) -> Result<Self::Ok, Self::Error>;
}

#[derive(Debug, Clone)]
pub struct Evaluator;

// pub trait Evaluate: LoxExpression {
//   fn evaluate<E>(self, evaluator: E) -> Result<E::Ok, E::Error>
//   where
//     E: LoxEvaluator<Self>;
// }

// impl<E> Evaluate for E
// where
//   E: LoxExpression,
// {
//   fn evaluate<EV>(self, evaluator: EV) -> Result<EV::Ok, EV::Error>
//   where
//     EV: LoxEvaluator<Self>
//   {
//     evaluator.evaluate(self)
//   }
// }

pub type TokenStream<'a> = SliceStream<'a, Token>;

generate_ast! [
  {
    Literal
  
    Struct: BooleanExpression,
  
    Output: LoxBool
  },
  {
    Literal
  
    Struct: IntegerExpression,
  
    Output: LoxInteger
  },
  {
    Literal
  
    Struct: FloatExpression,
  
    Output: LoxFloat
  },
  {
    Literal
  
    Struct: StringExpression,
  
    Output: LoxString
  },
  {
    Literal
  
    Struct: NilExpression,
  
    Output: LoxNil
  },
  {
    Binary
  
    Struct: EqualEqualExpression,
  
    Trait: LoxEqualEqualExpression,
  
    Constraint: LoxEq,
  
    Func: equal_equal,
  
    Precedence: Seven,
  
    PrecedenceLimit: Six,
  
    Output: LoxBool
  },
  {
    Binary
  
    Struct: BangEqualExpression,
  
    Trait: LoxBangEqualExpression,
  
    Constraint: LoxEq,
  
    Func: bang_equal,
  
    Precedence: Seven,
  
    PrecedenceLimit: Six,
  
    Output: LoxBool
  },
  {
    Binary
  
    Struct: GreaterExpression,
  
    Trait: LoxGreaterExpression,
  
    Constraint: LoxOrd,
  
    Func: greater,
  
    Precedence: Six,
  
    PrecedenceLimit: Five,
  
    Output: LoxBool
  },
  {
    Binary
  
    Struct: GreaterEqualExpression,
  
    Trait: LoxGreaterEqualExpression,
  
    Constraint: LoxOrd,
  
    Func: greater_equal,
  
    Precedence: Six,
  
    PrecedenceLimit: Five,
  
    Output: LoxBool
  },
  {
    Binary
  
    Struct: LesserExpression,
  
    Trait: LoxLesserExpression,
  
    Constraint: LoxOrd,
  
    Func: lesser,
  
    Precedence: Six,
  
    PrecedenceLimit: Five,
  
    Output: LoxBool
  },
  {
    Binary
  
    Struct: LesserEqualExpression,
  
    Trait: LoxLesserEqualExpression,
  
    Constraint: LoxOrd,
  
    Func: lesser_equal,
  
    Precedence: Six,
  
    PrecedenceLimit: Five,
  
    Output: LoxBool
  },
  {
    Binary
  
    Struct: SubtractExpression,
  
    Trait: LoxSubtractExpression,
  
    Constraint: LoxSub,
  
    Func: subtract,
  
    Precedence: Five,
  
    PrecedenceLimit: Four,
  
    Output: <Self as LoxSubtractExpression>::Value
  },
  {
    Binary
  
    Struct: AddExpression,
  
    Trait: LoxAddExpression,
  
    Constraint: LoxAdd,
  
    Func: add,
  
    Precedence: Four,
  
    PrecedenceLimit: Four,
  
    Output: <Self as LoxAddExpression>::Value
  },
  {
    Binary
  
    Struct: DivideExpression,
  
    Trait: LoxDivideExpression,
  
    Constraint: LoxDiv,
  
    Func: divide,
  
    Precedence: Three,
  
    PrecedenceLimit: Two,
  
    Output: <Self as LoxDivideExpression>::Value
  },
  {
    Binary
  
    Struct: MultiplyExpression,
  
    Trait: LoxMultiplyExpression,
  
    Constraint: LoxMul,
  
    Func: multiply,
  
    Precedence: Two,
  
    PrecedenceLimit: Two,
  
    Output: <Self as LoxMultiplyExpression>::Value
  },
  {
    Unary
  
    Struct: NegateExpression,
  
    Trait: LoxNegateExpression,
  
    Constraint: LoxNeg,
  
    Func: negate
  },
  {
    Unary
  
    Struct: NotExpression,
  
    Trait: LoxNotExpression,
  
    Constraint: LoxNot,
  
    Func: not
  },
  {
    Group
  
    Struct: GroupExpression,
  
    Trait: LoxGroupExpression
  }
];