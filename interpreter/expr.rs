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
    fn visit_get(&mut self, name: &String, token: &Token, expr: &Expr) -> Result<R, Error>;
    fn visit_set(
        &mut self,
        token: &Token,
        name: &String,
        value: &Expr,
        obj: &Expr,
    ) -> Result<R, Error>;
    fn visit_this(&mut self, token: &Token) -> Result<R, Error>;
    fn visit_super(&mut self, token: &Token, method_name: &String) -> Result<R, Error>;
}

#[derive(Debug, Clone, EnumAsInner)]
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
    Get {
        name: String,
        token: Token,
        expr: Box<Expr>,
    },
    Set {
        name: String,
        token: Token,
        value: Box<Expr>,
        obj: Box<Expr>,
    },
    This {
        token: Token,
    },
    Super {
        token: Token,
        method_name: String,
    },
}

impl Expr {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> Result<R, Error> {
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
            Expr::Get { name, token, expr } => visitor.visit_get(name, token, expr),
            Expr::Set {
                token,
                name,
                value,
                obj,
            } => visitor.visit_set(token, name, value, obj),
            Expr::This { token } => visitor.visit_this(token),
            Expr::Super { token, method_name } => visitor.visit_super(token, method_name),
        }
    }
}
