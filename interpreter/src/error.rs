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
}

#[derive(Debug, Clone)]
pub struct Error {
    pub token: Token,
    pub error_type: ErrorType,
}
