//! The book, [Crafting Interpreters](https://craftinginterpreters.com/), builds
//! a separate tool for generating ASTs that represent the Lox grammar
//! (for more info, [see chapter](https://craftinginterpreters.com/representing-code.html#implementing-syntax-trees)).



/// This macro generates the necessary structs, traits, and impls to represent the
/// Lox grammar using [`Expression`].
macro_rules! generate_ast {
  [$($def:tt),*] => {
    $(
      generate_ast! $def
    )*
  };
  {
    Literal
    
    Struct: $expr:ident,
    
    Output: $val:ty
  } => {
    #[derive(Debug, Clone)]
    pub struct $expr($val);
    
    impl LoxValueExpression for $expr {
      type Value = $val;
    }
    
    impl From<$val> for $expr {
      fn from(v: $val) -> Self {
        Self(v)
      }
    }
    
    impl Into<$val> for $expr {
      fn into(self) -> $val {
        let Self(v) = self;
        v
      }
    }
    
    impl From<$val> for Expression<$expr> {
      fn from(v: $val) -> Self {
        Self {
          expression: $expr::from(v),
        }
      }
    }
    
    impl Into<$val> for Expression<$expr> {
      fn into(self) -> $val {
        self.inner().into()
      }
    }
    
    impl PrecedenceExpression for $expr {
      type Level = Zero;
    }
    
    impl LoxEvaluator<$expr> for Evaluator {
      type Ok = $val;
      
      type Error = crate::interpreter::error::Error;
      
      fn evaluate(self, expr: $expr) -> Result<Self::Ok, Self::Error> {
        Ok(expr.into())
      }
    }
  };
  {
    Binary
    
    Struct: $expr:ident,
    
    Trait: $expr_trait:ident,
    
    Constraint: $con:ident,
    
    Func: $func:ident,
    
    Precedence: $prec:ty,
    
    PrecedenceLimit: $lim:ty,
    
    Output: $val:ty
  } => {
    #[derive(Debug, Clone)]
    pub struct $expr<PL, PR, V, L, R>
    where
      PL: PrecedenceLevel + PrecedenceLimit<$lim>,
      PR: PrecedenceLevel + PrecedenceLimit<$lim>,
      V: LoxValue + $con,
      L: LoxValueExpression<Value = V> + PrecedenceExpression<Level = PL>,
      R: LoxValueExpression<Value = V> + PrecedenceExpression<Level = PR>,
    {
      left: Box<L>,
      right: Box<R>,
      _data: PhantomData<(PL, PR, V)>,
    }

    pub trait $expr_trait {
      type Value: LoxValue + $con;
  
      type PrecedenceLeft: PrecedenceLevel + PrecedenceLimit<$lim>;
  
      type PrecedenceRight: PrecedenceLevel + PrecedenceLimit<$lim>;

      type Left: LoxValueExpression<Value = Self::Value> + PrecedenceExpression<Level = Self::PrecedenceLeft>;
  
      type Right: LoxValueExpression<Value = Self::Value> + PrecedenceExpression<Level = Self::PrecedenceRight>;

      fn $func(left: Self::Left, right: Self::Right) -> Self;
    }

    impl<PL, PR, V, L, R> LoxValueExpression for $expr<PL, PR, V, L, R>
    where
      PL: PrecedenceLevel + PrecedenceLimit<$lim>,
      PR: PrecedenceLevel + PrecedenceLimit<$lim>,
      V: LoxValue + $con,
      L: LoxValueExpression<Value = V> + PrecedenceExpression<Level = PL>,
      R: LoxValueExpression<Value = V> + PrecedenceExpression<Level = PR>,
    {
      type Value = $val;
    }

    impl<PL, PR, V, L, R> PrecedenceExpression for $expr<PL, PR, V, L, R>
    where
      PL: PrecedenceLevel + PrecedenceLimit<$lim>,
      PR: PrecedenceLevel + PrecedenceLimit<$lim>,
      V: LoxValue + $con,
      L: LoxValueExpression<Value = V> + PrecedenceExpression<Level = PL>,
      R: LoxValueExpression<Value = V> + PrecedenceExpression<Level = PR>,
    {
      type Level = $prec;
    }

    impl<PL, PR, V, L, R> $expr_trait for $expr<PL, PR, V, L, R>
    where
      PL: PrecedenceLevel + PrecedenceLimit<$lim>,
      PR: PrecedenceLevel + PrecedenceLimit<$lim>,
      V: LoxValue + $con,
      L: LoxValueExpression<Value = V> + PrecedenceExpression<Level = PL>,
      R: LoxValueExpression<Value = V> + PrecedenceExpression<Level = PR>,
    {
      type PrecedenceLeft = PL;
  
      type PrecedenceRight = PR;

      type Value = V;
  
      type Left = L;
  
      type Right = R;

      fn $func(left: Self::Left, right: Self::Right) -> Self {
        Self {
          left: left.into_boxed(),
          right: right.into_boxed(),
          _data: PhantomData::default(),
        }
      }
    }

    impl<PL, PR, V, L, R, E> Expression<E>
    where
      PL: PrecedenceLevel + PrecedenceLimit<$lim>,
      PR: PrecedenceLevel + PrecedenceLimit<$lim>,
      V: LoxValue + $con,
      L: LoxValueExpression<Value = V> + PrecedenceExpression<Level = PL>,
      R: LoxValueExpression<Value = V> + PrecedenceExpression<Level = PR>,
      E: LoxValueExpression + $expr_trait<PrecedenceLeft = PL, PrecedenceRight = PR, Value = V, Left = L, Right = R>,
    {
      fn $func(left: Expression<E::Left>, right: Expression<E::Right>) -> Self {
        Self {
          expression: E::$func(left.inner(), right.inner()),
        }
      }
    }
  };
  {
    Unary
    
    Struct: $expr:ident,
    
    Trait: $expr_trait:ident,
    
    Constraint: $con:ident,
    
    Func: $func:ident
  } => {
    #[derive(Debug, Clone)]
    pub struct $expr<P, V, E>
    where
      P: PrecedenceLevel + PrecedenceLimit<One>,
      V: LoxValue + $con,
      E: LoxValueExpression<Value = V> + PrecedenceExpression<Level = P>,
    {
      expr: Box<E>,
      _data: PhantomData<(P, V)>,
    }

    pub trait $expr_trait {
      type Value: LoxValue + $con;
  
      type Precedence: PrecedenceLevel + PrecedenceLimit<One>;
  
      type Expression: LoxValueExpression<Value = Self::Value> + PrecedenceExpression<Level = Self::Precedence>;
  
      fn $func(expr: Self::Expression) -> Self;
    }

    impl<P, V, E> LoxValueExpression for $expr<P, V, E>
    where
      P: PrecedenceLevel + PrecedenceLimit<One>,
      V: LoxValue + $con,
      E: LoxValueExpression<Value = V> + PrecedenceExpression<Level = P>,
    {
      type Value = E::Value;
    }

    impl<P, V, E> PrecedenceExpression for $expr<P, V, E>
    where
      P: PrecedenceLevel + PrecedenceLimit<One>,
      V: LoxValue + $con,
      E: LoxValueExpression<Value = V> + PrecedenceExpression<Level = P>,
    {
      type Level = One;
    }

    impl<P, V, E> $expr_trait for $expr<P, V, E>
    where
      P: PrecedenceLevel + PrecedenceLimit<One>,
      V: LoxValue + $con,
      E: LoxValueExpression<Value = V> + PrecedenceExpression<Level = P>,
    {
      type Precedence = P;

      type Value = V;
  
      type Expression = E;

      fn $func(expr: Self::Expression) -> Self {
        Self {
          expr: expr.into_boxed(),
          _data: PhantomData::default(),
        }
      }
    }

    impl<P, V, IE, E> Expression<E>
    where
      P: PrecedenceLevel + PrecedenceLimit<One>,
      V: LoxValue + $con,
      IE: LoxValueExpression<Value = V> + PrecedenceExpression<Level = P>,
      E: LoxValueExpression + $expr_trait<Precedence = P, Value = V, Expression = IE>,
    {
      fn $func(expr: Expression<E::Expression>) -> Self {
        Self {
          expression: E::$func(expr.inner()),
        }
      }
    }
  };
  {
    Group
    
    Struct: $expr:ident,
    
    Trait: $expr_trait:ident
  } => {
    #[derive(Debug, Clone)]
    pub struct $expr<E: LoxValueExpression>(Box<E>);

    pub trait $expr_trait {
      type Expression: LoxValueExpression;
  
      fn group(expr: Self::Expression) -> Self;
    }
    
    impl<E> LoxValueExpression for $expr<E>
    where
      E: LoxValueExpression,
    {
      type Value = E::Value;
    }
    
    impl<E> PrecedenceExpression for $expr<E>
    where
      E: LoxValueExpression,
    {
      type Level = Zero;
    }
    
    impl<V, E> $expr_trait for $expr<E>
    where
      V: LoxValue,
      E: LoxValueExpression<Value = V>,
    {
      type Expression = E;
      
      fn group(expr: Self::Expression) -> Self {
        Self(expr.into_boxed())
      }
    }
    
    impl<V, IE, E> Expression<E>
    where
      V: LoxValue,
      IE: LoxValueExpression<Value = V>,
      E: LoxValueExpression<Value = V> + $expr_trait<Expression = IE>,
    {
      fn group(expr: Expression<E::Expression>) -> Self {
        Self {
          expression: E::group(expr.inner()),
        }
      }
    }
  };
}

pub(crate) use generate_ast;