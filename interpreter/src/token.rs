use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Display, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Identifier(String),
    Null,
}

#[derive(Debug, PartialEq, Clone, Display)]
pub enum TokenType {
    EOF,
    Invalid,
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
    Assign,
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
    pub start: usize,
    pub end: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(
            format!(
                "Token type: {} Line: {}:{}-{}",
                self.token_type, self.line, self.start, self.end
            )
            .as_str(),
        )?;
        Ok(())
    }
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, start: usize, end: usize) -> Self {
        Token {
            token_type,
            line,
            start,
            end,
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
        map.insert("fn", TokenType::Function);
        map.insert("return", TokenType::Return);
        map.insert("class", TokenType::Class);
        map.insert("super", TokenType::Super);
        map.insert("this", TokenType::This);
        map.insert("null", TokenType::Null);
        map.insert("print", TokenType::Print);
        map.insert("null", TokenType::Literal(Literal::Null));
        map
    };
}
