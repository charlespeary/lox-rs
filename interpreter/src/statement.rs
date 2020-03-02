use crate::error::Error;
use crate::expr::Expr;
use crate::runtime_value::Value;
use crate::token::Token;

pub trait Visitor<R> {
    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<R, Error>;
    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<R, Error>;
    fn visit_var(&mut self, name: &String, value: &Option<Expr>) -> Result<R, Error>;
    fn visit_block_stmt(&mut self, stms: &Vec<Stmt>) -> Result<R, Error>;
    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_body: &Stmt,
        else_body: &Option<Box<Stmt>>,
    ) -> Result<R, Error>;
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<R, Error>;
    fn visit_break_stmt(&mut self, token: &Token) -> Result<R, Error>;
    fn visit_continue_stmt(&mut self, token: &Token) -> Result<R, Error>;
    fn visit_function_stmt(
        &mut self,
        name: &String,
        params: &Vec<String>,
        body: &Vec<Stmt>,
        token: &Token,
    ) -> Result<R, Error>;
    fn visit_class_stmt(
        &mut self,
        name: &String,
        token: &Token,
        members: &Vec<Stmt>,
        superclass: &Option<Expr>,
    ) -> Result<R, Error>;
    fn visit_return_stmt(&mut self, value: &Option<Expr>, token: &Token) -> Result<R, Error>;
}

#[derive(Debug, Clone, EnumAsInner)]
pub enum Stmt {
    Print {
        expr: Expr,
    },
    Expr {
        expr: Expr,
    },
    Var {
        name: String,
        value: Option<Expr>,
    },
    Block {
        stmts: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_body: Box<Stmt>,
        else_body: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Break {
        token: Token,
    },
    Continue {
        token: Token,
    },
    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
        name: String,
        token: Token,
    },
    Class {
        name: String,
        token: Token,
        members: Vec<Stmt>,
        superclass: Option<Expr>,
    },
    Return {
        token: Token,
        value: Option<Expr>,
    },
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> Result<R, Error> {
        match self {
            Stmt::Print { expr } => visitor.visit_print_stmt(expr),
            Stmt::Expr { expr } => visitor.visit_expr_stmt(expr),
            Stmt::Var { name, value } => visitor.visit_var(name, value),
            Stmt::Block { stmts } => visitor.visit_block_stmt(stmts),
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => visitor.visit_if_stmt(condition, then_body, else_body),
            Stmt::While { condition, body } => visitor.visit_while_stmt(condition, body),
            Stmt::Continue { token } => visitor.visit_continue_stmt(token),
            Stmt::Break { token } => visitor.visit_break_stmt(token),
            Stmt::Function {
                name,
                params,
                body,
                token,
            } => visitor.visit_function_stmt(name, params, body, token),
            Stmt::Class {
                name,
                token,
                members,
                superclass,
            } => visitor.visit_class_stmt(name, token, members, superclass),
            Stmt::Return { value, token } => visitor.visit_return_stmt(value, token),
        }
    }
}
