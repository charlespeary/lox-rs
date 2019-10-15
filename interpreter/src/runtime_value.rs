use crate::token::Literal;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug)]
pub enum RuntimeError {
    AdditionTypeMismatch,
    SubtractionTypeMismatch,
    MultiplicationTypeMismatch,
    DivisionTypeMismatch,
    UnaryTypeMismatch,
}

#[derive(Clone, Debug)]
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
            // TODO: not sure what to do with identifier yet
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
        // TODO: these errors should be more robust, but for now it's just fine
        match self {
            Value::Number(a) => match value {
                Value::Number(b) => Ok(Value::Number(a + b)),
                _ => Err(RuntimeError::AdditionTypeMismatch),
            },
            Value::Boolean(a) => match value {
                // TODO: not sure how to handle boolean logic yet
                Value::Boolean(b) => Ok(Value::Boolean(a && b)),
                _ => Err(RuntimeError::AdditionTypeMismatch),
            },
            Value::String(a) => match value {
                Value::String(b) => Ok(Value::String([a, b].concat())),
                _ => Err(RuntimeError::AdditionTypeMismatch),
            },
            // never add null to anything
            Value::Null => Err(RuntimeError::AdditionTypeMismatch),
        }
    }
}

impl Sub for Value {
    type Output = InterpreterResult;

    fn sub(self, value: Value) -> InterpreterResult {
        // TODO: these errors should be more robust, but for now it's just fine
        match self {
            Value::Number(a) => match value {
                Value::Number(b) => Ok(Value::Number(a - b)),
                _ => Err(RuntimeError::SubtractionTypeMismatch),
            },
            // never add null to anything
            _ => Err(RuntimeError::SubtractionTypeMismatch),
        }
    }
}

impl Mul for Value {
    type Output = InterpreterResult;

    fn mul(self, value: Value) -> InterpreterResult {
        // TODO: these errors should be more robust, but for now it's just fine
        match self {
            Value::Number(a) => match value {
                Value::Number(b) => Ok(Value::Number(a * b)),
                _ => Err(RuntimeError::MultiplicationTypeMismatch),
            },
            // never add null to anything
            _ => Err(RuntimeError::MultiplicationTypeMismatch),
        }
    }
}

impl Div for Value {
    type Output = InterpreterResult;

    fn div(self, value: Value) -> InterpreterResult {
        // TODO: these errors should be more robust, but for now it's just fine
        match self {
            Value::Number(a) => match value {
                Value::Number(b) => Ok(Value::Number(a / b)),
                _ => Err(RuntimeError::DivisionTypeMismatch),
            },
            // never add null to anything
            _ => Err(RuntimeError::DivisionTypeMismatch),
        }
    }
}
