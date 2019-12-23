use crate::runtime_value::Value;
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn from(env: &Rc<RefCell<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(Rc::clone(env)),
        }
    }

    pub fn define_or_update(&mut self, name: &str, value: &Value) -> Option<Value> {
        if let Some(env) = &self.enclosing {
            return env.borrow_mut().define_or_update(name, value);
        }
        self.values.insert(name.to_owned(), value.clone())
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(env) = &self.enclosing {
            return env.borrow().get(name);
        }
        self.values.get(name).map(|val| val.clone())
    }
}
