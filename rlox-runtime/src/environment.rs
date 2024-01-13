use std::{any::Any, cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::types::*;

#[derive(Default)]
pub struct Environment {
    inner: Rc<BTreeMap<String, Rc<dyn LoxObject>>>,
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
    pub fn enclose(env: Rc<Environment>) -> Self {
        Self {
            inner: Rc::default(),
            enclosing: Some(env),
        }
    }

    pub fn define<L: LoxObject + LoxType>(&mut self, name: String, value: L) {
        match &mut self.enclosing {
            None => {
                let value: Rc<dyn LoxObject> = Rc::new(value);
                let env = Rc::get_mut(&mut self.inner).unwrap();
                env.insert(name, value);
            }
            Some(env) => {
                let env = Rc::get_mut(env).unwrap();
                env.define(name, value);
            }
        };
    }

    pub fn get<L: LoxObject + LoxType>(&self, name: &String) -> Option<&L> {
        let value = match &self.enclosing {
            None => self.inner.get(name),
            Some(env) => {
                let env = Rc::as_ref(env);

                match env.get(name) {
                    None => self.inner.get(name),
                    val => val,
                }
            }
        };

        match value {
            None => None,
            Some(v) => v.as_ref().as_type::<L>(),
        }
    }

    pub fn update(&mut self, name: String, value: LoxValue) -> Option<LoxValue> {
        match &mut self.enclosing {
            None => self.update_inner(name, value),
            Some(env) => {
                let env = Rc::get_mut(env).unwrap();
                env.update(name, value)
            }
        }
    }

    fn update_inner(&mut self, name: String, value: LoxValue) -> Option<LoxValue> {
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
