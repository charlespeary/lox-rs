use crate::class::{Class, Instance as ClassInstance};
use crate::function::Function;
use crate::token::Literal;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

type Instance = Rc<RefCell<ClassInstance>>;
#[derive(Clone, EnumAsInner)]
pub enum Value {
    Function(Function),
    String(String),
    Number(f64),
    Boolean(bool),
    Class(Class),
    Instance(Instance),
    Null,
}

impl Value {
    pub fn new(literal: &Literal) -> Value {
        match literal {
            Literal::Number(val) => Value::Number(val.clone()),
            Literal::String(val) => Value::String(val.clone()),
            Literal::Null => Value::Null,
            Literal::Bool(val) => Value::Boolean(val.clone()),
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Value::String(val) => val.len() > 0,
            Value::Boolean(val) => *val,
            Value::Null => false,
            Value::Class(_) | Value::Number(_) | Value::Function(_) | Value::Instance(_) => true,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Value::String(str) => str.to_string(),
            Value::Number(num) => format!("{}", num).to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Function(fun) => fun.to_string(),
            Value::Null => "null".to_string(),
            Value::Class(class) => class.to_string(),
            Value::Instance(instance) => instance.borrow().to_string(),
        };
        fmt.write_str(&str)?;
        Ok(())
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Value::String(str) => str.to_string(),
            Value::Number(num) => format!("{}", num).to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Function(fun) => fun.to_string(),
            Value::Null => "null".to_string(),
            Value::Class(class) => class.to_string(),
            Value::Instance(instance) => instance.borrow().to_string(),
        };
        fmt.write_str(&str)?;
        Ok(())
    }
}
