use std::collections::HashMap;

#[derive(Debug, Clone, Display, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Identifier(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    EOF,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Coma,
    Dot,
    Minus,
    Plus,
    Star,
    Divide,
    Semicolon,
    Bang,
    BangEquals,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,
    Compare,
    Equals,
    Comment,
    If,
    Else,
    False,
    True,
    Var,
    While,
    For,
    And,
    Or,
    Function,
    Return,
    Class,
    Super,
    This,
    Null,
    Print,
    Literal(Literal),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub line_offset: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, line_offset: usize) -> Self {
        Token {
            token_type,
            line,
            line_offset,
        }
    }
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut map: HashMap<&'static str, TokenType> = HashMap::new();
        map.insert("if", TokenType::If);
        map.insert("else", TokenType::Else);
        map.insert("false", TokenType::False);
        map.insert("true", TokenType::True);
        map.insert("var", TokenType::Var);
        map.insert("while", TokenType::While);
        map.insert("for", TokenType::For);
        map.insert("&&", TokenType::And);
        map.insert("||", TokenType::Or);
        map.insert("==", TokenType::Equals);
        map.insert("!=", TokenType::BangEquals);
        map.insert("<=", TokenType::LessEquals);
        map.insert(">=", TokenType::GreaterEquals);
        map.insert("fn", TokenType::Function);
        map.insert("return", TokenType::Return);
        map.insert("class", TokenType::Class);
        map.insert("super", TokenType::Super);
        map.insert("this", TokenType::This);
        map.insert("null", TokenType::Null);
        map.insert("print", TokenType::Print);
        map
    };
}
