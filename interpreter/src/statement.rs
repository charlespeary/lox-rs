use crate::error::Error;
use crate::expr::Expr;
use crate::token::Token;

pub trait Visitor<R> {
    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<R, Error>;
    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<R, Error>;
    fn visit_var(&mut self, name: &String, value: &Expr) -> Result<R, Error>;
    fn visit_block_stmt(&mut self, stms: &Vec<Stmt>) -> Result<R, Error>;
}

#[derive(Debug)]
pub enum Stmt {
    Print { expr: Expr },
    Expr { expr: Expr },
    Var { name: String, value: Expr },
    Block { stmts: Vec<Stmt> },
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> Result<R, Error> {
        match self {
            Stmt::Print { expr } => visitor.visit_print_stmt(expr),
            Stmt::Expr { expr } => visitor.visit_expr_stmt(expr),
            Stmt::Var { name, value } => visitor.visit_var(name, value),
            Stmt::Block { stmts } => visitor.visit_block_stmt(stmts),
        }
    }
}
