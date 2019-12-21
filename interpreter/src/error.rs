use crate::token::Token;

#[derive(Debug, Clone, Display)]
pub enum ErrorType {
    #[display(fmt = "String not closed")]
    StringNotClosed,
    #[display(fmt = "Unexpected character")]
    UnexpectedCharacter,
    #[display(fmt = "Unparsable expression")]
    UnparsableExpression,
    #[display(fmt = "Unclosed parenthesis")]
    UnclosedParenthesis,
    #[display(fmt = "Expected operator")]
    ExpectedOperator,
    #[display(fmt = "Expected unary operator")]
    ExpectedUnaryOperator,
    #[display(fmt = "Expected semicolon")]
    ExpectedSemicolon,
    #[display(fmt = "Unexpected type mismatch")]
    WrongType,
    #[display(fmt = "Expected variable to have an identifier")]
    ExpectedIdentifier,
    #[display(fmt = "Expected assign after identifier")]
    ExpectedAssign,
    #[display(fmt = "Variable is undefined")]
    UndefinedVariable,
    #[display(fmt = "Invalid Assignment")]
    InvalidAssignment,
    #[display(fmt = "Expected close brace at the end of the block")]
    ExpectedBlockEnd,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub token: Token,
    pub error_type: ErrorType,
}
