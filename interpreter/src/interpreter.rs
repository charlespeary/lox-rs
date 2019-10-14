use crate::parser::{Expression, Operator};
use crate::token::{Literal, TokenType};


fn add(left: Literal, right: Literal) -> Literal {
    match left {
         Literal::Number(left_val) => {
                match right {
                    Literal::Number(right_val) => {
                        return Literal::Number(left_val + right_val)
                    }
                    _ => left
                }
        }
        _ => Literal::Number(0.0)
    }
}

pub fn visit_expr(expr: Box<Expression>) -> Literal {
    match *expr {
        Expression::Literal(literal) => literal,
        Expression::Binary(left, operator, right) => {
            let left_val = visit_expr(left);
            let right_val = visit_expr(right);
            match operator {
                 Operator::Plus => {
                        add(left_val, right_val)
                }
                Operator::Minus => {
                    subtract(left_val, right_val)
                }
                Operator::Multiply => {
                    multiply(left_val, right_val)
                }
                Operator::Divide => {
                    divide(left_val, right_val)
                }
                _ => Literal::Number(0.0)
            }
        }
        _ => Literal::Number(0.0)
    }
}
