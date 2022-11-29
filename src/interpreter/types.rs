use crate::prelude::*;

pub trait LoxValue: Sized + DebugTrait + Clone {}

pub trait LoxPrimitive: LoxValue {
  type Prim;
}

macro_rules! lox_type {
  ($lt:ident) => {
    #[derive(Debug, Clone, PartialEq)]
    pub struct $lt;
    
    impl LoxValue for $lt {}
  };

  ($lt:ident, $prim:ty) => {
    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub struct $lt($prim);
    
    impl LoxValue for $lt {}
    
    impl LoxPrimitive for $lt {
      type Prim = $prim;
    }
    
    impl From<$prim> for $lt {
      fn from(v: $prim) -> Self {
        Self(v)
      }
    }
    
    impl Into<$prim> for $lt {
      fn into(self) -> $prim {
        let $lt(prim) = self;
        prim
      }
    }
  }
}

lox_type!(LoxInteger, i32);
lox_type!(LoxFloat, f32);
lox_type!(LoxBool, bool);
lox_type!(LoxString, String);
lox_type!(LoxNil);

pub trait LoxEq: LoxValue + PartialEq {
  fn equals(&self, v: Self) -> LoxBool;
}

impl<V: LoxValue + PartialEq> LoxEq for V {
  fn equals(&self, v: Self) -> LoxBool {
    LoxBool::from(*self == v)
  }
}

pub trait LoxOrd: LoxValue + PartialOrd {
  fn compare(&self, v: Self) -> LoxInteger;
}

impl<V: LoxValue + PartialOrd> LoxOrd for V {
  fn compare(&self, v: Self) -> LoxInteger {
    if *self == v {
      return LoxInteger::from(0)
    }
    if *self > v {
      return LoxInteger::from(1)
    }
    LoxInteger::from(-1)
  }
}

pub trait LoxAdd: Sized {
  fn add(self, v: Self) -> Self;
}

impl<P, V> LoxAdd for V
where
  P: std::ops::Add<Output = P>,
  V: LoxPrimitive<Prim = P> + Into<P> + From<P>,
{
  fn add(self, v: Self) -> Self {
    Self::from(self.into() + v.into())
  }
}

pub trait LoxSub: Sized {
  fn subtract(self, v: Self) -> Self;
}

impl<P, V> LoxSub for V
where
  P: std::ops::Sub<Output = P>,
  V: LoxPrimitive<Prim = P> + Into<P> + From<P>,
{
  fn subtract(self, v: Self) -> Self {
    Self::from(self.into() - v.into())
  }
}

pub trait LoxMul: Sized {
  fn multiply(self, v: Self) -> Self;
}

impl<P, V> LoxMul for V
where
  P: std::ops::Mul<Output = P>,
  V: LoxPrimitive<Prim = P> + Into<P> + From<P>,
{
  fn multiply(self, v: Self) -> Self {
    Self::from(self.into() * v.into())
  }
}

pub trait LoxZero: Sized {
  fn zero() -> Self;
  fn is_zero(&self) -> bool;
}

impl LoxZero for i32 {
  fn zero() -> Self {
    0
  }
  
  fn is_zero(&self) -> bool {
    *self == Self::zero()
  }
}

impl LoxZero for f32 {
  fn zero() -> Self {
    0.0
  }
  
  fn is_zero(&self) -> bool {
    *self == Self::zero()
  }
}

impl LoxZero for LoxInteger {
  fn zero() -> Self {
    LoxInteger::from(i32::zero())
  }
  
  fn is_zero(&self) -> bool {
    *self == Self::zero()
  }
}

impl LoxZero for LoxFloat {
  fn zero() -> Self {
    LoxFloat::from(f32::zero())
  }
  
  fn is_zero(&self) -> bool {
    *self == Self::zero()
  }
}

pub trait LoxDiv: Sized {
  fn divide(self, v: Self) -> Result<Self, ()>;
}

impl<P, V> LoxDiv for V
where
  P: std::ops::Div<Output = P>,
  V: LoxPrimitive<Prim = P> + LoxZero + Into<P> + From<P>,
{
  fn divide(self, v: Self) -> Result<Self, ()> {
    if v.is_zero() {
      return Err(())
    }
    Ok(Self::from(self.into() / v.into()))
  }
}

pub trait LoxNeg: Sized {
  fn negate(self) -> Self;
}

impl<P, V> LoxNeg for V
where
  P: std::ops::Neg<Output = P>,
  V: LoxPrimitive<Prim = P> + Into<P> + From<P>,
{
  fn negate(self) -> Self {
    Self::from(self.into().neg())
  }
}

pub trait LoxNot: Sized {
  fn not(self) -> Self;
}

impl LoxNot for LoxBool {
  fn not(self) -> Self {
    let inner_self: bool = self.into();
    Self::from(!inner_self)
  }
}
