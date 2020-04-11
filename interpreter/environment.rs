use crate::runtime_value::Value;
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

    pub fn has_enclosing(&self) -> bool {
        self.enclosing.is_some()
    }

    pub fn define_or_update(&mut self, name: &str, value: &Value) -> Option<Value> {
        self.values.insert(name.to_owned(), value.clone())
    }

    pub fn assign_at(&mut self, name: &str, value: &Value, distance: usize) -> Option<Value> {
        if distance > 0 {
            self.ancestor(distance)
                .borrow_mut()
                .define_or_update(name, value)
        } else {
            self.define_or_update(name, value)
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.values.get(name).map(|val| val.clone())
    }

    pub fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment>> {
        // this clone here is probably very expensive one
        let mut env = self
            .enclosing
            .clone()
            .expect("Trying to access environment that doesn't exist");

        for _ in 1..distance {
            let enclosing = env.borrow().enclosing.clone();
            if let Some(e) = &enclosing {
                env = Rc::clone(e)
            }
        }
        env
    }

    pub fn get_at(&self, name: &str, distance: usize) -> Option<Value> {
        if distance == 0 {
            self.get(name)
        } else {
            self.ancestor(distance).borrow().get(name)
        }
    }

    pub fn get_deep(&self, name: &str) -> Option<Value> {
        match self.values.get(name) {
            Some(this) => Some(this.clone()),
            None => {
                if let Some(e) = &self.enclosing {
                    e.borrow().get_deep(name)
                } else {
                    None
                }
            }
        }
    }
}
