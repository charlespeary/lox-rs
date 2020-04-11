use crate::runtime_value::Value;
use crate::token::{Token, TokenType};

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
    #[display(fmt = "Expected close bar")]
    ExpectedCloseBar,
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
    #[display(fmt = "Expected open brace at the start of the block")]
    ExpectedBlockStart,
    #[display(fmt = "Cannot be used outside loops")]
    NotAllowedOutsideLoop,
    #[display(fmt = "Maximum number of the arguments is 255")]
    MaximumArguments,
    #[display(fmt = "This value is not callable")]
    ValueNotCallable,
    #[display(fmt = "Expected arrow after closure declaration")]
    ExpectedArrow,
    #[display(fmt = "Invalid number of arguments")]
    InvalidNumberOfArguments,
    #[display(fmt = "Can't use variable in it's own initializer")]
    CantUseVariableInItsInitializer,
    #[display(fmt = "Return")]
    Return(Value),
    #[display(fmt = "Value is not an instance, therefore you can't access its properties")]
    ValueNotInstance,
    #[display(fmt = "This instance doesn't have this property")]
    PropertyDoesntExist,
    #[display(fmt = "Class can't inherit from itself")]
    CantInheritFromItself,
    #[display(fmt = "Can only inherit from class")]
    CanOnlyInheritFromClass,
    #[display(fmt = "Expected dot after super")]
    DotAfterSuper,
    #[display(fmt = "Method not found in the superclass instance")]
    MethodNotFound,
    #[display(fmt = "Can't use super outside class or inside a class without superclass")]
    CantUseSuper,
    #[display(fmt = "Can't use this outside class")]
    CantUseThis,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub token: Token,
    pub error_type: ErrorType,
}

impl From<Vec<Error>> for Error {
    // This is used in Resolver, during static analysis and this is not a good way, because I discard errors
    // Maybe it will be fixed in the future
    fn from(errors: Vec<Error>) -> Self {
        errors.get(0).unwrap().clone()
    }
}

pub fn error(token: &Token, error_type: ErrorType) -> Result<Value, Error> {
    Err(Error {
        token: token.clone(),
        error_type,
    })
}

pub fn return_stmt(val: Value) -> Result<(), Error> {
    Err(Error {
        token: Token {
            token_type: TokenType::Return,
            line: 0,
            start: 0,
            end: 0,
        },
        error_type: ErrorType::Return(val),
    })
}
