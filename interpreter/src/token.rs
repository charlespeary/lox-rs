use std::collections::HashMap;

#[derive(Debug, Clone, Display, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Identifier(String),
}
#[derive(Debug, Clone, PartialEq, Display)]
pub enum Keyword {
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
    Keyword(Keyword),
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
    pub static ref KEYWORDS: HashMap<&'static str, Keyword> = {
        let mut map: HashMap<&'static str, Keyword> = HashMap::new();
        map.insert("if", Keyword::If);
        map.insert("else", Keyword::Else);
        map.insert("false", Keyword::False);
        map.insert("true", Keyword::True);
        map.insert("var", Keyword::Var);
        map.insert("while", Keyword::While);
        map.insert("for", Keyword::For);
        map.insert("and", Keyword::And);
        map.insert("or", Keyword::Or);
        map.insert("fn", Keyword::Function);
        map.insert("return", Keyword::Return);
        map.insert("class", Keyword::Class);
        map.insert("super", Keyword::Super);
        map.insert("this", Keyword::This);
        map.insert("null", Keyword::Null);
        map.insert("print", Keyword::Print);
        map
    };
}
