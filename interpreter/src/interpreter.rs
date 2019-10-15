use crate::parser::{Expression, Operator, UnaryOperator};
use crate::runtime_value::{RuntimeError, Value};
use crate::token::{Literal, TokenType};

type InterpreterResult = Result<Value, RuntimeError>;

pub fn interpret(expr: Box<Expression>) -> InterpreterResult {
    match *expr {
        Expression::Literal(literal) => Ok(Value::new(literal)),
        Expression::Binary(left, operator, right) => {
            let a = interpret(left)?;
            let b = interpret(right)?;
            match operator {
                Operator::Plus => a + b,
                Operator::Minus => a - b,
                Operator::Multiply => a * b,
                Operator::Divide => a / b,
            }
        }
        Expression::Grouping(expr) => interpret(expr),
        Expression::Unary(operator, expr) => {
            let val = interpret(expr)?;
            match operator {
                UnaryOperator::Minus => match val {
                    Value::Number(val) => Ok(Value::Number(val * -1.0)),
                    _ => Err(RuntimeError::UnaryTypeMismatch),
                },
                UnaryOperator::Bang => Ok(Value::Boolean(!val.to_bool())),
            }
        }
        _ => Ok(Value::Null),
    }
}
