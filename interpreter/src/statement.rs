use crate::parser::Expression;

pub enum StmtType {
    Print,
    Expression,
}

pub struct Stmt {
    expr: Box<Expression>,
    stmt_type: StmtType,
}
