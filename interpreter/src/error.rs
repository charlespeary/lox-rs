use crate::token::Token;

#[derive(Debug, Clone, Display)]
pub enum ErrorType {
    #[display(fmt = "String not closed")]
    StringNotClosed,
    #[display(fmt = "Unexpected character")]
    UnexpectedCharacter,
    #[display(fmt = "Unparsable expression")]
    UnparsableExpression,
    #[display(fmt = "Expected open parenthesis")]
    ExpectedOpenParenthesis,
    #[display(fmt = "Expected close parenthesis")]
    ExpectedCloseParenthesis,
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
    #[display(fmt = "Expected if statement to have an else block")]
    ExpectedElseStatement,
    #[display(fmt = "This keyword needs can't be used outside of the loops")]
    NotAllowedOutsideLoop,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub token: Token,
    pub error_type: ErrorType,
}
