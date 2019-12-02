use crate::error::Error;
use crate::expr::Expr;

pub trait Visitor<R> {
    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<R, Error>;
    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<R, Error>;
}

#[derive(Debug)]
pub enum Stmt {
    Print { expr: Expr },
    Expr { expr: Expr },
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> Result<R, Error> {
        match self {
            Stmt::Print { expr } => visitor.visit_print_stmt(expr),
            Stmt::Expr { expr } => visitor.visit_expr_stmt(expr),
        }
    }
}
