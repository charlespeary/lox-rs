use super::token::{Literal, Token, TokenType, KEYWORDS};
use crate::errors::{Error, ErrorType, LexerError};
use crate::token::Literal::Number;
use crate::utils::is_numeric;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer {
    source_code: Vec<char>,
    pub tokens: Vec<Token>,
    current: usize,
    line_offset: usize,
    line: usize,
    errors: Vec<LexerError>,
}

impl Lexer {
    pub fn new(source_code: &str) -> Self {
        Lexer {
            source_code: source_code.chars().collect(),
            tokens: Vec::new(),
            current: 0,
            line_offset: 1,
            line: 1,
            errors: Vec::new(),
        }
    }

    fn get_current(&self) -> char {
        // this function is always called inside code that is wrapped in while self.has_next(), so we're sure that we won't run out of chars
        self.source_code.get(self.current).cloned().unwrap()
    }

    fn has_current(&self) -> bool {
        self.current < self.source_code.len()
    }

    fn get_next(&mut self) -> Option<char> {
        if self.has_next() {
            return Some(self.advance());
        }
        None
    }

    fn peek(&self) -> Option<char> {
        self.source_code.get(self.current + 1).cloned()
    }

    fn previous(&self) -> Option<char> {
        self.source_code.get(self.current - 1).cloned()
    }

    fn next_matches(&mut self, to_match: char) -> bool {
        if to_match == self.get_current() {
            self.advance();
            return true;
        }
        false
    }

    fn peek_match(&self, to_match: char) -> bool {
        if let Some(c) = self.peek() {
            if c == to_match {
                return true;
            }
        }
        false
    }

    fn next_line(&mut self) {
        self.line += 1;
    }

    fn skip_line(&mut self) {
        while self.has_next() {
            let c = self.advance();
            if c == '\n' {
                self.next_line();
                self.advance();
                return;
            }
        }
    }

    fn next_comment(&mut self) -> bool {
        if let Some(next) = self.peek() {
            if next == '/' {
                self.skip_line();
                return true;
            }
        }
        false
    }

    // TODO: rethink flow of the chars
    fn advance(&mut self) -> char {
        let item = self.source_code[self.current];
        self.current += 1;
        self.line_offset += 1;
        item
    }

    fn has_next(&self) -> bool {
        self.peek().is_some()
    }

    fn raise_error(&mut self, error_type: ErrorType) -> Result<Token, LexerError> {
        Err(LexerError {
            error: Error {
                line_offset: self.line_offset,
                line: self.line,
                error_type,
            },
            literal: self.get_current(),
        })
    }

    fn create_token(&self, token_type: TokenType) -> Result<Token, LexerError> {
        Ok(Token {
            line: self.line,
            line_offset: self.line_offset,
            token_type,
        })
    }

    fn get_string(&mut self) -> Result<Token, LexerError> {
        let mut value = String::new();
        while self.has_next() {
            let c = self.advance();
            if c == '"' {
                return self.create_token(TokenType::Literal(Literal::String(value)));
            } else {
                value.push(c);
            }
        }
        self.raise_error(ErrorType::StringNotClosed)
    }

    fn get_number(&mut self) -> Result<Token, LexerError> {
        let mut value = self.get_current().to_string();
        while let Some(c) = self.peek() {
            if is_numeric(c) {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }
        match value.parse::<f64>() {
            Ok(value) => self.create_token(TokenType::Literal(Literal::Number(value))),
            _ => self.raise_error(ErrorType::UnexpectedCharacter),
        }
    }

    fn get_identifier(&mut self, first_character: char) -> Result<Token, LexerError> {
        let mut identifier_literal = self.get_current().to_string();
        while let Some(c) = self.get_next() {
            if c == '\n' || c == ' ' {
                break;
            }
            identifier_literal.push(c);
            // check if identifier is one of the keywords
            let identifier = KEYWORDS.get::<str>(&identifier_literal);
            if let Some(value) = identifier {
                self.next_line();
                return self.create_token(TokenType::Keyword((*value).clone()));
            }
        }
        // if not return it as an identifier
        self.create_token(TokenType::Literal(Literal::Unknown(identifier_literal)))
    }

    fn get_literal(&mut self) -> Result<Token, LexerError> {
        let c = self.get_current();
        if c == '"' {
            self.get_string()
        } else if is_numeric(c) {
            self.get_number()
        } else if c.is_alphanumeric() {
            self.get_identifier(c)
        } else {
            self.raise_error(ErrorType::UnexpectedCharacter)
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<LexerError>> {
        while self.has_current() {
            let c = self.get_current();
            // early match to discard items that won't return token type
            match c {
                ' ' | '\t' | '\r' => {
                    self.advance();
                    continue;
                }
                '\n' => {
                    self.next_line();
                    continue;
                }
                _ => (),
            }

            let token = match c {
                '(' => self.create_token(TokenType::OpenParenthesis),
                ')' => self.create_token(TokenType::CloseParenthesis),
                '{' => self.create_token(TokenType::OpenBrace),
                '}' => self.create_token(TokenType::CloseBrace),
                ',' => self.create_token(TokenType::Coma),
                '.' => self.create_token(TokenType::Dot),
                '-' => self.create_token(TokenType::Minus),
                '+' => self.create_token(TokenType::Plus),
                '*' => self.create_token(TokenType::Star),
                ';' => self.create_token(TokenType::Semicolon),
                '!' => {
                    let token_type = if self.next_matches('=') {
                        TokenType::BangEquals
                    } else {
                        TokenType::Bang
                    };
                    self.create_token(token_type)
                }
                '<' => {
                    let token_type = if self.next_matches('=') {
                        TokenType::LessEquals
                    } else {
                        TokenType::Less
                    };
                    self.create_token(token_type)
                }
                '>' => {
                    let token_type = if self.next_matches('=') {
                        TokenType::GreaterEquals
                    } else {
                        TokenType::Greater
                    };
                    self.create_token(token_type)
                }
                '=' => {
                    let token_type = if self.next_matches('=') {
                        TokenType::Compare
                    } else {
                        TokenType::Equals
                    };
                    self.create_token(token_type)
                }
                '/' => {
                    let token_type = if self.next_comment() {
                        TokenType::Comment
                    } else {
                        TokenType::Divide
                    };
                    self.create_token(token_type)
                }
                _ => self.get_literal(),
            };

            self.advance();

            match token {
                Ok(t) => {
                    self.tokens.push(t);
                }
                Err(e) => {
                    self.errors.push(e);
                }
            }
        }

        self.create_token(TokenType::EOF);

        if self.errors.len() > 0 {
            return Err(self.errors.clone());
        }
        Ok(self.tokens.clone())
    }
}
