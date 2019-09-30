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
}

#[derive(Debug, Clone)]
pub struct Error {
    pub line: usize,
    pub line_offset: usize,
    pub error_type: ErrorType,
}
