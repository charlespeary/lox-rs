use super::token::{Literal, Token, TokenType, KEYWORDS};
use crate::errors::{Error, ErrorType};
use crate::utils::is_numeric;
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'a> {
    source_code: Peekable<Chars<'a>>,
    pub tokens: Vec<Token>,
    line: usize,
    line_offset: usize,
    errors: Vec<Error>,
}

impl<'a> Scanner<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Scanner {
            source_code: source_code.chars().peekable(),
            tokens: Vec::new(),
            line: 1,
            line_offset: 1,
            errors: Vec::new(),
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens
            .push(Token::new(token_type, self.line, self.line_offset));
        self.line_offset += 1;
    }

    fn add_token_and_advance(&mut self, token_type: TokenType) {
        self.tokens
            .push(Token::new(token_type, self.line, self.line_offset));
        self.line_offset += 1;
        self.source_code.next();
    }

    fn next_equals(&mut self, c: char) -> bool {
        self.source_code.peek().map(|_c| *_c == c).unwrap_or(false)
    }

    fn skip_line(&mut self) {
        while let Some(c) = self.source_code.next() {
            if c == '\n' {
                return self.next_line();
            }
        }
    }

    fn next_line(&mut self) {
        self.line += 1;
        self.line_offset = 1;
    }

    fn get_string(&mut self) -> Result<TokenType, Error> {
        let mut value = String::new();
        while let Some(c) = self.source_code.next() {
            if c == '"' {
                return Ok(TokenType::Literal(Literal::String(value)));
            } else {
                value.push(c);
            }
        }
        Err(self.raise_error(ErrorType::StringNotClosed))
    }

    fn get_number(&mut self, first_number: char) -> Result<TokenType, Error> {
        let mut value = first_number.to_string();
        while let Some(c) = self.source_code.peek().cloned() {
            if c.is_numeric() || (c == '.' && !self.next_equals('.')) {
                value.push(c);
                self.source_code.next();
            } else {
                break;
            }
        }
        match value.parse::<f64>() {
            Ok(value) => Ok(TokenType::Literal(Literal::Number(value))),
            _ => Err(self.raise_error(ErrorType::UnexpectedCharacter)),
        }
    }

    fn get_identifier(&mut self, first_character: char) -> TokenType {
        let mut identifier_literal = first_character.to_string();
        while let Some(c) = self.source_code.next() {
            if c == '\n' || c == ' ' {
                break;
            }
            identifier_literal.push(c);
            // check if identifier is one of the keywords
            let identifier = KEYWORDS.get::<str>(&identifier_literal);
            if let Some(value) = identifier {
                self.next_line();
                return TokenType::Keyword((*value).clone());
            }
        }
        // if not return it as an identifier
        TokenType::Literal(Literal::Unknown(identifier_literal))
    }
    fn raise_error(&mut self, error_type: ErrorType) -> Error {
        let error = Error {
            line: self.line,
            line_offset: self.line_offset,
            error_type,
        };
        println!(
            "{:#?} at {}.{}",
            error.error_type, error.line, error.line_offset
        );
        self.errors.push(error.clone());
        error
    }

    pub fn scan_tokens(&mut self) {
        while let Some(c) = self.source_code.next() {
            match c {
                ' ' | '\t' | '\r' => {
                    self.line_offset += 1;
                    continue;
                }
                '\n' => {
                    self.next_line();
                }
                '(' => self.add_token(TokenType::OpenParenthesis),
                ')' => self.add_token(TokenType::CloseParenthesis),
                '{' => self.add_token(TokenType::OpenBrace),
                '}' => self.add_token(TokenType::CloseBrace),
                ',' => self.add_token(TokenType::Coma),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                '*' => self.add_token(TokenType::Star),
                ';' => self.add_token(TokenType::Semicolon),
                '!' => {
                    if self.next_equals('=') {
                        self.add_token_and_advance(TokenType::BangEquals);
                    } else {
                        self.add_token(TokenType::Bang);
                    }
                }
                '<' => {
                    if self.next_equals('=') {
                        self.add_token_and_advance(TokenType::LessEquals);
                    } else {
                        self.add_token(TokenType::Less);
                    }
                }
                '>' => {
                    if self.next_equals('=') {
                        self.add_token_and_advance(TokenType::GreaterEquals);
                    } else {
                        self.add_token(TokenType::Greater);
                    }
                }
                '=' => {
                    if self.next_equals('=') {
                        self.add_token_and_advance(TokenType::Compare);
                    } else {
                        self.add_token(TokenType::Equals);
                    }
                }
                '/' => {
                    if self.next_equals('/') {
                        self.add_token(TokenType::Comment);
                        self.skip_line();
                    } else {
                        self.add_token(TokenType::Divide);
                    }
                }
                '"' => {
                    let result = self.get_string();
                    match result {
                        Ok(token_type) => self.add_token(token_type),
                        _ => (),
                    }
                }
                _ => {
                    if is_numeric(c) {
                        let result = self.get_number(c);
                        match result {
                            Ok(token_type) => self.add_token(token_type),
                            _ => (),
                        }
                    } else if c.is_alphanumeric() {
                        let identifier = self.get_identifier(c);
                        self.add_token(identifier);
                    } else {
                        self.raise_error(ErrorType::UnexpectedCharacter);
                    }
                }
            };
            self.line_offset += 1;
        }
        self.add_token(TokenType::EOF);
    }
}
