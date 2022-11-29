//! Structs, enums, and traits for representing the Lox grammar for value expressions
//!
//! The Lox grammar has expressions that produce values. These expressions are represented
//! independently from other types of expressions (i.e. statement expressions). This is
//! is so operator are logically mapped to the correct type of operand. Without this, the
//! following expressions would be legal:
//!
//! ```
//! (print "hello") + 5
//! -(print "there")
//! ```
//!
//! This is something that could be handled at runtime. However, Rust has such an expressive
//! type system that we should be able to catch things like this when we parse the token stream.


use crate::interpreter::types::*;

pub trait LoxValueExpression {
  type Value: LoxValue;
}

impl<E> super::LoxExpression for E
where
  E: LoxValueExpression,
{}

pub trait BoxedLoxValueExpression {
  type Expression: LoxValueExpression;
  
  fn unbox(self) -> Self::Expression;
}

impl<E> BoxedLoxValueExpression for Box<E>
where
  E: LoxValueExpression,
{
  type Expression = E;
  
  fn unbox(self) -> Self::Expression {
    *self
  }
}

pub trait IntoBoxedLoxValueExpression: LoxValueExpression {
  fn into_boxed(self) -> Box<Self>;
}

impl<E> IntoBoxedLoxValueExpression for E
where
  E: LoxValueExpression,
{
  fn into_boxed(self) -> Box<Self> {
    Box::from(self)
  }
}


// fn test() {
//   let e0 = Expression::from(LoxInteger::from(5));
//   let e1 = Expression::from(LoxInteger::from(2));
//   let me = Expression::multiply(e0, e1.clone());
//   let ge = Expression::group(me);
//   let de = Expression::divide(ge, e1);
//   let _ = de;
// }
