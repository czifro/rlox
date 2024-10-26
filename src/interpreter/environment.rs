use std::{collections::BTreeMap, rc::Rc, sync::RwLock};

use super::values::LoxValue;

#[derive(Default)]
pub struct Environment {
    inner: Rc<BTreeMap<String, LoxValue>>,
    enclosing: Option<Rc<Environment>>,
	enclosed: Option<Rc<RwLock<>
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

    pub fn drop_enclusure(self) -> Self {
        *Rc::as_ref(&self.enclosing.unwrap())
    }

    pub fn define(&mut self, name: String, value: LoxValue) {
        let env = Rc::get_mut(&mut self.inner).unwrap();
        env.insert(name, value);
    }

    pub fn get(&self, name: &String) -> Option<&LoxValue> {
        let env = Rc::as_ref(&self.inner);
        if let Some(val) = env.get(name) {
            return Some(val);
        }

        match &self.enclosing {
            None => None,
            Some(env) => {
                let env = Rc::as_ref(env);

                match env.get(name) {
                    None => self.inner.get(name),
                    val => val,
                }
            }
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
}
