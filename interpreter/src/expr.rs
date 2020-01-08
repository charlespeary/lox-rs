use crate::error::Error;
use crate::statement::Stmt;
use crate::token::{Literal, Token};

pub trait Visitor<R> {
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<R, Error>;
    fn visit_literal(&mut self, literal: &Literal) -> Result<R, Error>;
    fn visit_unary(&mut self, operator: &Token, expr: &Expr) -> Result<R, Error>;
    fn visit_grouping(&mut self, expr: &Expr) -> Result<R, Error>;
    fn visit_var(&mut self, name: &String, token: &Token) -> Result<R, Error>;
    fn visit_assignment(&mut self, name: &String, expr: &Expr, token: &Token) -> Result<R, Error>;
    fn visit_logical(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<R, Error>;
    fn visit_call(
        &mut self,
        callee: &Expr,
        token: &Token,
        arguments: &Vec<Expr>,
    ) -> Result<R, Error>;
    fn visit_closure(
        &mut self,
        params: &Vec<String>,
        body: &Vec<Stmt>,
        name: &String,
        token: &Token,
    ) -> Result<R, Error>;
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
    Var {
        name: String,
        token: Token,
    },
    Assign {
        name: String,
        expr: Box<Expr>,
        token: Token,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        token: Token,
        arguments: Vec<Expr>,
    },
    Closure {
        params: Vec<String>,
        body: Vec<Stmt>,
        name: String,
        token: Token,
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
            Expr::Var { name, token } => visitor.visit_var(name, token),
            Expr::Assign { name, expr, token } => visitor.visit_assignment(name, expr, token),
            Expr::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logical(left, operator, right),

            Expr::Call {
                callee,
                token,
                arguments,
            } => visitor.visit_call(callee, token, arguments),
            Expr::Closure {
                params,
                body,
                token,
                name,
            } => visitor.visit_closure(params, body, name, token),
        }
    }
}
