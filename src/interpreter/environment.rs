use std::{collections::BTreeMap, rc::Rc};

use super::values::LoxValue;

#[derive(Default)]
pub struct Environment {
	inner: Rc<BTreeMap<String, LoxValue>>,
	enclosing: Option<Rc<Environment>>,
}

impl Clone for Environment {
	fn clone(&self) -> Self {
		Self {
			inner: Rc::clone(&self.inner),
			enclosing: self.enclosing.clone(),
		}
	}
}

impl Environment {
	pub fn define(&mut self, name: String, value: LoxValue) {
		match &mut self.enclosing {
			None => {
				let env = Rc::get_mut(&mut self.inner).unwrap();
				env.insert(name, value);
			}
			Some(env) => {
				let env = Rc::get_mut(env).unwrap();
				env.define(name, value);
			}
		};
	}

	pub fn get(&self, name: &String) -> Option<&LoxValue> {
		match &self.enclosing {
			None => self.inner.get(name),
			Some(env) => {
				let env = Rc::as_ref(env);

				match env.get(name) {
					None => self.inner.get(name),
					val => val,
				}
			}
		}
	}

	pub fn update(
		&mut self,
		name: String,
		value: LoxValue,
	) -> Option<LoxValue> {
		match &mut self.enclosing {
			None => self.update_inner(name, value),
			Some(env) => {
				let env = Rc::get_mut(env).unwrap();
				env.update(name, value)
			}
		}
	}

	fn update_inner(
		&mut self,
		name: String,
		value: LoxValue,
	) -> Option<LoxValue> {
		if let None = self.inner.get(&name) {
			return None;
		}
		let env = Rc::get_mut(&mut self.inner).unwrap();
		env.insert(name, value)
	}

	pub fn create_enclosing(&mut self) {
		match &mut self.enclosing {
			None => {
				self.enclosing = Some(Rc::default());
			}
			Some(env) => {
				let env = Rc::get_mut(env).unwrap();
				env.create_enclosing();
			}
		};
	}

	pub fn drop_enclosing(&mut self) -> Option<Environment> {
		let env = match &mut self.enclosing {
			None => None,
			Some(env) => {
				let env = Rc::get_mut(env).unwrap();
				match env.drop_enclosing() {
					Some(env) => return Some(env),
					None => Some(env.to_owned()),
				}
			}
		};

		if env.is_some() {
			self.enclosing = None;
		}

		env
	}
}
