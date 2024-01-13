use std::{collections::HashMap, rc::Rc};

pub trait LoxAny: DebugTrait {
	fn def(&self) -> LoxTypeDef;
}

#[derive(Debug, Default, Clone)]
pub enum LoxValue<V> {
	#[default]
	Nil,
	Value(V),
}

#[derive(Debug, Clone)]
pub struct LoxTypeDef {
	name: String,
	fields: HashMap<String, Rc<LoxTypeDef>>,
	functions: HashMap<String, Rc<LoxFunctionDef>>,
	// interfaces: HashMap<String, Box<LoxClassDef>>,
}

#[derive(Debug, Clone)]
pub struct LoxTypeInstance<V> {
	def: LoxTypeDef,
	value: LoxValue<V>,
	fields: HashMap<String, Rc<dyn LoxAny>>,
}

impl<V> LoxAny for LoxTypeInstance<V>
where
	V: DebugTrait,
{
	fn def(&self) -> LoxTypeDef {
		self.def
	}
}

impl<V> Into<Rc<dyn LoxAny>> for LoxTypeInstance<V>
where
	V: DebugTrait,
{
	fn into(self) -> Rc<dyn LoxAny> {
		Rc::new(self)
	}
}

// pub struct LoxInterface {
// 	name: String,
// 	funcs: HashMap<String, ()>
// }

pub trait FunctionInvoker {
	fn invoke(&self, def: LoxFunctionDef) -> Rc<dyn LoxAny>;
}

#[derive(Debug, Clone)]
pub struct LoxFunctionDef {
	name: String,
	args: HashMap<String, LoxTypeDef>,
	result: LoxTypeDef,
}

pub struct Class;
pub type LoxClass = LoxTypeInstance<Class>;
impl LoxClass {
	pub fn new(fields: HashMap<String, Rc<dyn LoxAny>>) -> Self {
		let field_defs = fields
			.clone()
			.iter()
			.map(|(k, v)| (k.clone(), Rc::new(v.def())))
			.collect();

		Self {
			def: LoxTypeDef {
				name: "Class".to_string(),
				fields: field_defs,
				functions: HashMap::default(),
			},
			value: LoxValue::default(),
			fields,
		}
	}
}

pub type LoxBool = LoxTypeInstance<bool>;
impl LoxBool {
	pub fn new(value: bool) -> Self {
		Self {
			def: Self::def(),
			value: LoxValue::Value(value),
			fields: HashMap::default(),
		}
	}

	pub fn class() -> LoxClass {
		let def = Self::def();

		let fields: HashMap<String, Rc<dyn LoxAny>> = HashMap::from(
			[("name".to_string(), LoxString::new(def.name).into())],
		);

		LoxClass::new(fields)
	}

	pub fn def() -> LoxTypeDef {
		LoxTypeDef {
			name: "Bool".to_string(),
			fields: HashMap::default(),
			functions: HashMap::default(),
		}
	}
}

pub type LoxString = LoxTypeInstance<String>;
impl LoxString {
	pub fn new(value: String) -> Self {
		Self {
			def: Self::def(),
			value: LoxValue::Value(value),
			fields: HashMap::default(),
		}
	}

	pub fn class() -> LoxClass {
		let def = Self::def();

		let fields: HashMap<String, Rc<dyn LoxAny>> = HashMap::from(
			[("name".to_string(), LoxString::new(def.name).into())],
		);

		LoxClass::new(fields)
	}

	pub fn def() -> LoxTypeDef {
		LoxTypeDef {
			name: "String".to_string(),
			fields: HashMap::default(),
			functions: HashMap::default(),
		}
	}
}

pub type LoxNumber = LoxTypeInstance<f64>;
impl LoxNumber {
	pub fn new(value: f64) -> Self {
		Self {
			def: Self::def(),
			value: LoxValue::Value(value),
			fields: HashMap::default(),
		}
	}

	pub fn class() -> LoxClass {
		let def = Self::def();

		let fields: HashMap<String, Rc<dyn LoxAny>> = HashMap::from(
			[("name".to_string(), LoxString::new(def.name).into())],
		);

		LoxClass::new(fields)
	}

	pub fn def() -> LoxTypeDef {
		LoxTypeDef {
			name: "Number".to_string(),
			fields: HashMap::default(),
			functions: HashMap::default(),
		}
	}
}

// pub trait LoxType: Sized + DebugTrait + Clone + Default {
// 	fn instance_type_info(&self) -> LoxTypeInfo<Self> {
// 		Self::type_info()
// 	}
//
// 	fn type_info() -> LoxTypeInfo<Self> {
// 		LoxTypeInfo::default()
// 	}
//
// 	fn is_type<L>(&self) -> bool
// 	where
// 		L: LoxType,
// 		String: From<LoxTypeInfo<L>>,
// 		String: From<LoxTypeInfo<Self>>,
// 	{
// 		Self::type_info().is::<L>()
// 	}
// }
//
// #[derive(Debug, Default, Clone)]
// pub struct LoxTypeInfo<L>(L)
// where
// 	L: LoxType;
//
// impl<L> LoxTypeInfo<L>
// where
// 	L: LoxType,
// 	String: From<Self>,
// {
// 	pub fn is<Other>(&self) -> bool
// 	where
// 		Other: LoxType,
// 		String: From<LoxTypeInfo<Other>>,
// 	{
// 		let other: String = Other::type_info().name();
//
// 		self.name() == other
// 	}
//
// 	pub fn name(&self) -> String {
// 		String::from(self.clone())
// 	}
// }
//
// impl<L> From<L> for LoxTypeInfo<L>
// where
// 	L: LoxType,
// {
// 	fn from(value: L) -> Self {
// 		Self(value)
// 	}
// }
//
// impl<V> Display for LoxTypeInfo<V>
// where
// 	V: LoxType + PartialEq,
// {
// 	fn fmt(
// 		&self,
// 		f: &mut std::fmt::Formatter<'_>,
// 	) -> std::fmt::Result {
// 		write!(f, "{}", self.to_string())
// 	}
// }
//
// pub trait LoxPrimitive:
// 	LoxType + Into<Self::Prim> + From<Self::Prim>
// {
// 	type Prim;
//
// 	fn to_prim(self) -> Self::Prim {
// 		self.into()
// 	}
// }
//
// macro_rules! lox_prim {
// 	($lt:ident, $prim:ty, $name:expr) => {
// 		#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
// 		pub struct $lt($prim);
//
// 		impl LoxType for $lt {}
//
// 		impl From<LoxTypeInfo<$lt>> for String {
// 			fn from(value: LoxTypeInfo<$lt>) -> Self {
// 				$name.to_string()
// 			}
// 		}
//
// 		impl LoxPrimitive for $lt {
// 			type Prim = $prim;
// 		}
//
// 		impl From<$prim> for $lt {
// 			fn from(v: $prim) -> Self {
// 				Self(v)
// 			}
// 		}
//
// 		impl Into<$prim> for $lt {
// 			fn into(self) -> $prim {
// 				let $lt(prim) = self;
// 				prim
// 			}
// 		}
// 	};
// }
//
// lox_prim!(LoxNumber, f64, "number");
// lox_prim!(LoxBool, bool, "bool");
// lox_prim!(LoxString, String, "string");
//
// impl LoxBool {
// 	pub fn r#true() -> Self {
// 		Self::from(true)
// 	}
// 	pub fn r#false() -> Self {
// 		// !Self::true()
// 		Self::from(false)
// 	}
// }
//
// #[derive(Debug, Default, Clone)]
// pub enum LoxValue {
// 	#[default]
// 	Nil,
// 	Number(LoxNumber),
// 	Bool(LoxBool),
// 	String(LoxString),
// }
//
// impl LoxValue {
// 	pub fn is_nil(&self) -> bool {
// 		*self == Self::Nil
// 	}
// }
//
// impl LoxType for LoxValue {}
//
// impl PartialEq for LoxValue {
// 	fn eq(&self, other: &Self) -> bool {
// 		match (self, other) {
// 			(Self::Number(lhs), Self::Number(rhs)) => lhs == rhs,
// 			(Self::String(lhs), Self::String(rhs)) => lhs == rhs,
// 			(Self::Bool(lhs), Self::Bool(rhs)) => lhs == rhs,
// 			(Self::Nil, Self::Nil) => true,
// 			_ => false,
// 		}
// 	}
// }
//
// impl Into<String> for LoxTypeInfo<LoxValue> {
// 	fn into(self) -> String {
// 		match self.0 {
// 			LoxValue::Nil => panic!("Nil reference"),
// 			LoxValue::Number(_) => LoxNumber::type_info().into(),
// 			LoxValue::Bool(_) => LoxBool::type_info().into(),
// 			LoxValue::String(_) => LoxString::type_info().into(),
// 		}
// 	}
// }
//
// impl From<Token> for LoxValue {
// 	fn from(token: Token) -> Self {
// 		match token.token_type.clone() {
// 			TokenType::String
// 			| TokenType::True
// 			| TokenType::False
// 			| TokenType::Integer
// 			| TokenType::Float => Self::from(token.literal.unwrap()),
// 			TokenType::Nil => Self::Nil,
// 			_ => unreachable!(
// 				"Unexpected token type: {:?}",
// 				token.token_type
// 			),
// 		}
// 	}
// }
//
// impl From<TokenLiteral> for LoxValue {
// 	fn from(lit: TokenLiteral) -> Self {
// 		match lit {
// 			TokenLiteral::Nil(_) => Self::Nil,
// 			TokenLiteral::Bool(b) => Self::Bool(LoxBool::from(b)),
// 			TokenLiteral::String(s) => {
// 				Self::String(LoxString::from(s))
// 			}
// 			TokenLiteral::Float(f) => {
// 				Self::Number(LoxNumber::from(f64::from(f)))
// 			}
// 			TokenLiteral::Integer(i) => {
// 				Self::Number(LoxNumber::from(f64::from(i)))
// 			}
// 		}
// 	}
// }
//
// impl From<LoxBool> for LoxValue {
// 	fn from(value: LoxBool) -> Self {
// 		Self::Bool(value)
// 	}
// }
//
// impl From<LoxNumber> for LoxValue {
// 	fn from(value: LoxNumber) -> Self {
// 		Self::Number(value)
// 	}
// }
//
// impl From<LoxString> for LoxValue {
// 	fn from(value: LoxString) -> Self {
// 		Self::String(value)
// 	}
// }
//
// #[derive(Debug, Clone)]
// pub struct LoxEq<L>(L)
// where
// 	L: LoxType + PartialEq;
//
// impl<L> LoxEq<L>
// where
// 	L: LoxType + PartialEq,
// 	LoxValue: From<L>,
// 	String: From<LoxTypeInfo<L>>,
// {
// 	pub fn eq<Other>(&self, other: &LoxEq<Other>) -> LoxBool
// 	where
// 		Other: LoxType + PartialEq,
// 		LoxValue: From<Other>,
// 		String: From<LoxTypeInfo<Other>>,
// 	{
// 		if !L::type_info().is::<Other>() {
// 			return LoxBool::r#true();
// 		}
//
// 		LoxBool::from(
// 			LoxValue::from(self.0) == LoxValue::from(other.0),
// 		)
// 	}
// }

// pub trait LoxEq: LoxType + PartialEq {
// 	fn equals(&self, v: Self) -> LoxBool;
// }
//
// impl<V: LoxType + PartialEq> LoxEq for V {
// 	fn equals(&self, v: Self) -> LoxBool {
// 		LoxBool::from(*self == v)
// 	}
// }
//
// pub trait LoxOrd: LoxType {
// 	fn compare(&self, v: &Self) -> LoxNumber;
// }
//
// impl<P, V> LoxOrd for V
// where
// 	P: PartialOrd,
// 	V: LoxType + LoxPrimitive<Prim = P>,
// {
// 	fn compare(&self, v: &Self) -> LoxNumber {
// 		match self.to_prim().partial_cmp(&v.to_prim()) {
// 			Some(Ordering::Less) => LoxNumber::from(-1.0),
// 			Some(Ordering::Equal) => LoxNumber::from(0.0),
// 			Some(Ordering::Greater) => LoxNumber::from(1.0),
// 			None => LoxNumber::from(2.0),
// 		}
// 	}
// }
//
// pub trait LoxAdd: Sized {
// 	fn add(self, v: Self) -> Self;
// }
//
// impl<P, V> LoxAdd for V
// where
// 	P: std::ops::Add<Output = P>,
// 	V: LoxPrimitive<Prim = P>,
// {
// 	fn add(self, v: Self) -> Self {
// 		Self::from(self.to_prim() + v.to_prim())
// 	}
// }
//
// macro_rules! impl_add {
// 	($lt:ident) => {
// 		impl std::ops::Add for $lt {
// 			type Output = Self;
//
// 			fn add(self, rhs: Self) -> Self::Output {
// 				<Self as LoxAdd>::add(self, rhs)
// 			}
// 		}
// 	};
// }
//
// impl_add!(LoxNumber);
// impl_add!(LoxString);
//
// pub trait LoxSub: Sized {
// 	fn subtract(self, v: Self) -> Self;
// }
//
// impl<P, V> LoxSub for V
// where
// 	P: std::ops::Sub<Output = P>,
// 	V: LoxPrimitive<Prim = P>,
// {
// 	fn subtract(self, v: Self) -> Self {
// 		Self::from(self.into() - v.into())
// 	}
// }
//
// macro_rules! impl_sub {
// 	($lt:ident) => {
// 		impl std::ops::Sub for $lt {
// 			type Output = Self;
//
// 			fn sub(self, rhs: Self) -> Self::Output {
// 				self.sub(rhs)
// 			}
// 		}
// 	};
// }
//
// impl_sub!(LoxNumber);
//
// pub trait LoxMul: Sized {
// 	fn multiply(self, v: Self) -> Self;
// }
//
// impl<P, V> LoxMul for V
// where
// 	P: std::ops::Mul<Output = P>,
// 	V: LoxPrimitive<Prim = P> + Into<P> + From<P>,
// {
// 	fn multiply(self, v: Self) -> Self {
// 		Self::from(self.into() * v.into())
// 	}
// }
//
// macro_rules! impl_mul {
// 	($lt:ident) => {
// 		impl std::ops::Mul for $lt {
// 			type Output = Self;
//
// 			fn mul(self, rhs: Self) -> Self::Output {
// 				self.mul(rhs)
// 			}
// 		}
// 	};
// }
//
// impl_mul!(LoxNumber);
//
// pub trait LoxZero: Sized + LoxEq {
// 	fn zeroed() -> Self;
//
// 	fn is_zero(&self) -> bool {
// 		*self == Self::zeroed()
// 	}
// }
//
// impl LoxZero for LoxNumber {
// 	fn zeroed() -> Self {
// 		Self::from(f64::default())
// 	}
// }
// pub trait LoxDiv: Sized {
// 	fn div(self, v: Self) -> Result<Self, String>;
// }
//
// impl<P, V> LoxDiv for V
// where
// 	P: std::ops::Div<Output = P>,
// 	V: LoxPrimitive<Prim = P> + LoxZero + Into<P> + From<P>,
// {
// 	fn div(self, v: Self) -> Result<Self, String> {
// 		if v.is_zero() {
// 			return Err("Divide by 0".to_string());
// 		}
// 		Ok(Self::from(self.into() / v.into()))
// 	}
// }

//
// #[derive(Debug, Clone, PartialEq)]
// pub enum LoxType {
// 	Nil = 0,
// 	Number,
// 	Bool,
// 	String,
// }
//
// impl From<LoxType> for String {
// 	fn from(value: LoxType) -> Self {
// 		use LoxType::*;
// 		match value {
// 			Nil => "nil".to_string(),
// 			Number => "number".to_string(),
// 			Bool => "bool".to_string(),
// 			String => "string".to_string(),
// 		}
// 	}
// }
//
// impl LoxValue {
// 	pub fn lox_type(&self) -> LoxType {
// 		match self {
// 			Self::Number(_) => LoxType::Number,
// 			Self::Bool(_) => LoxType::Bool,
// 			Self::String(_) => LoxType::String,
// 			Self::Nil => LoxType::Nil,
// 		}
// 	}
//
// 	pub fn is_same_type_as(&self, other: &Self) -> bool {
// 		self.lox_type() == other.lox_type()
// 	}
//
// 	pub fn is_nil(&self) -> bool {
// 		self.lox_type() == LoxType::Nil
// 	}
// }
//
//
// impl PartialOrd for LoxValue {
// 	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
// 		match (self, other) {
// 			(Self::Number(l), Self::Number(r)) => l.partial_cmp(r),
// 			(Self::String(l), Self::String(r)) => l.partial_cmp(r),
// 			_ => None,
// 		}
// 	}
// }
//
// impl Display for LoxValue {
// 	fn fmt(
// 		&self,
// 		f: &mut std::fmt::Formatter<'_>,
// 	) -> std::fmt::Result {
// 		match self {
// 			Self::Number(n) => write!(f, "{}", n),
// 			Self::String(s) => write!(f, "{}", s),
// 			Self::Bool(b) => write!(f, "{}", b),
// 			Self::Nil => write!(f, "nil"),
// 		}
// 	}
// }
