use crate::error::Error;
use crate::function::Function;
use crate::token::Literal;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone)]
pub enum Value {
    Function(Function),
    String(String),
    Number(f64),
    Boolean(bool),
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
            Value::Number(val) => true,
            Value::Boolean(val) => *val,
            Value::Function(val) => true,
            Null => false,
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
            Null => "null".to_string(),
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
            Null => "null".to_string(),
        };
        fmt.write_str(&str)?;
        Ok(())
    }
}
