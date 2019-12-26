use crate::error::Error;
use crate::expr::Expr;
use crate::token::Token;

pub trait Visitor<R> {
    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<R, Error>;
    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<R, Error>;
    fn visit_var(&mut self, name: &String, value: &Expr) -> Result<R, Error>;
    fn visit_block_stmt(&mut self, stms: &Vec<Stmt>) -> Result<R, Error>;
    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_body: &Stmt,
        else_body: &Stmt,
    ) -> Result<R, Error>;
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<R, Error>;
    fn visit_break_stmt(&mut self) -> Result<R, Error>;
    fn visit_continue_stmt(&mut self) -> Result<R, Error>;
}

#[derive(Debug)]
pub enum Stmt {
    Print {
        expr: Expr,
    },
    Expr {
        expr: Expr,
    },
    Var {
        name: String,
        value: Expr,
    },
    Block {
        stmts: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_body: Box<Stmt>,
        else_body: Box<Stmt>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Break,
    Continue,
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
            Stmt::Continue => visitor.visit_continue_stmt(),
            Stmt::Break => visitor.visit_break_stmt(),
        }
    }
}
