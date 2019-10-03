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

#[derive(Debug, Clone)]
pub struct LexerError {
    pub error: Error,
    pub literal: char,
}

#[derive(Debug, Clone)]
pub struct ParserError {
    pub error: Error,
}

pub fn print_lexer_errors(errors: &Vec<LexerError>) {
    //    println!("{:#?}", errors);
}

pub fn print_parser_errors(errors: &Vec<ParserError>) {
    //    println!("{:#?}", errors);
}
