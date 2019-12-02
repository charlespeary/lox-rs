use crate::error::Error;
use crate::token::{Literal, Token};

pub trait Visitor<R> {
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<R, Error>;
    fn visit_literal(&mut self, literal: &Literal) -> Result<R, Error>;
    fn visit_unary(&mut self, operator: &Token, expr: &Expr) -> Result<R, Error>;
    fn visit_grouping(&mut self, expr: &Expr) -> Result<R, Error>;
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        operator: Token,
        expr: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
}

impl Expr {
    pub fn accept<R>(&self, visitor: &mut Visitor<R>) -> Result<R, Error> {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Expr::Literal { value } => visitor.visit_literal(value),
            Expr::Grouping { expr } => visitor.visit_grouping(expr),
            Expr::Unary { operator, expr } => visitor.visit_unary(operator, expr),
        }
    }
}
