use crate::runtime_value::Value;
use crate::token::Token;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define_or_update(&mut self, name: &str, value: &Value) -> Option<Value> {
        if let Some(env) = &mut self.enclosing {
            return env.define_or_update(name, value);
        }
        self.values.insert(name.to_owned(), value.clone())
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        if let Some(env) = &self.enclosing {
            return env.get(name);
        }
        self.values.get(name)
    }
}
