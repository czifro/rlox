//! Traits for handling operator precedence
//!
//! Precedence traits allow for operator precedence to be described in
//! the Rust type system.

use crate::prelude::*;

/// Trait for representing a precedence level.
pub trait PrecedenceLevel: DebugTrait {}

/// Trait for adding a precedence to an expression.
pub trait PrecedenceExpression {
  type Level: PrecedenceLevel;
}

pub trait PrecedenceLimit<P: PrecedenceLevel + ?Sized> {}

macro_rules! limit_impl {
  ($prec:ty, limits $($lim:ty), *) => {
    $(
      impl PrecedenceLimit<$lim> for $prec {}
    )*
  }
}

/// P0 precedence has the highest precedence.
#[derive(Debug, Clone)]
pub struct Zero;

impl PrecedenceLevel for Zero {}

limit_impl!(Zero, limits Zero, One, Two, Three, Four, Five, Six, Seven);

/// P1 precedence.
#[derive(Debug, Clone)]
pub struct One;

impl PrecedenceLevel for One {}

limit_impl!(One, limits One, Two, Three, Four, Five, Six, Seven);

/// P2 precedence.
#[derive(Debug, Clone)]
pub struct Two;

impl PrecedenceLevel for Two {}

limit_impl!(Two, limits Two, Three, Four, Five, Six, Seven);
  
/// P3 precedence.
#[derive(Debug, Clone)]
pub struct Three;

impl PrecedenceLevel for Three {}

limit_impl!(Three, limits Three, Four, Five, Six, Seven);

/// P4 precedence.
#[derive(Debug, Clone)]
pub struct Four;

impl PrecedenceLevel for Four {}

limit_impl!(Four, limits Four, Five, Six, Seven);

/// P5 precedence.
#[derive(Debug, Clone)]
pub struct Five;

impl PrecedenceLevel for Five {}

limit_impl!(Five, limits Five, Six, Seven);

/// P6 precedence.
#[derive(Debug, Clone)]
pub struct Six;

impl PrecedenceLevel for Six {}

limit_impl!(Six, limits Six, Seven);

/// P7 precedence.
#[derive(Debug, Clone)]
pub struct Seven;

impl PrecedenceLevel for Seven {}

limit_impl!(Seven, limits Seven);

