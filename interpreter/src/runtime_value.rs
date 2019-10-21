use crate::token::Literal;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug)]
pub enum RuntimeError {
    WrongType,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

type InterpreterResult = Result<Value, RuntimeError>;

impl Value {
    pub fn new(literal: Literal) -> Value {
        match literal {
            Literal::Number(val) => Value::Number(val),
            Literal::String(val) => Value::String(val),
            Literal::Identifier(val) => Value::String(val),
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Value::String(val) => val.len() > 0,
            Value::Number(val) => true,
            Value::Boolean(val) => *val,
            Null => false,
        }
    }
}

impl Add for Value {
    type Output = InterpreterResult;

    fn add(self, value: Value) -> InterpreterResult {
        match (self, value) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String([a, b].concat())),
            _ => Err(RuntimeError::WrongType),
        }
    }
}

impl Sub for Value {
    type Output = InterpreterResult;

    fn sub(self, value: Value) -> InterpreterResult {
        match (self, value) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            _ => Err(RuntimeError::WrongType),
        }
    }
}

impl Mul for Value {
    type Output = InterpreterResult;

    fn mul(self, value: Value) -> InterpreterResult {
        match (self, value) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            _ => Err(RuntimeError::WrongType),
        }
    }
}

impl Div for Value {
    type Output = InterpreterResult;

    fn div(self, value: Value) -> InterpreterResult {
        match (self, value) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
            _ => Err(RuntimeError::WrongType),
        }
    }
}

pub fn equals(first: Value, second: Value) -> InterpreterResult {
    match (first, second) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a == b)),
        (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a == b)),
        (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a == b)),
        (Value::Null, Value::Null) => Ok(Value::Boolean(true)),
        _ => Err(RuntimeError::WrongType),
    }
}

pub fn bang_equals(first: Value, second: Value) -> InterpreterResult {
    match (first, second) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a != b)),
        (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a != b)),
        (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a != b)),
        (Value::Null, Value::Null) => Ok(Value::Boolean(false)),
        _ => Err(RuntimeError::WrongType),
    }
}

pub fn greater_equals(first: Value, second: Value) -> InterpreterResult {
    match (first, second) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a >= b)),
        (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a.len() >= b.len())),
        _ => Err(RuntimeError::WrongType),
    }
}

pub fn greater(first: Value, second: Value) -> InterpreterResult {
    match (first, second) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)),
        (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a.len() > b.len())),
        _ => Err(RuntimeError::WrongType),
    }
}

pub fn less_equals(first: Value, second: Value) -> InterpreterResult {
    match (first, second) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a <= b)),
        (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a.len() <= b.len())),
        (Value::Null, Value::Null) => Ok(Value::Boolean(false)),
        _ => Err(RuntimeError::WrongType),
    }
}

pub fn less(first: Value, second: Value) -> InterpreterResult {
    match (first, second) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)),
        (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a.len() < b.len())),
        (Value::Null, Value::Null) => Ok(Value::Boolean(false)),
        _ => Err(RuntimeError::WrongType),
    }
}
