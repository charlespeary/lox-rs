use super::token::{Literal, Token, TokenType, KEYWORDS};
use crate::error::{Error, ErrorType};
use crate::token::Literal::Number;
use crate::utils::is_numeric;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub struct Lexer {
    source_code: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    offset_start: usize,
    offset_current: usize,
    errors: Vec<Error>,
}

impl Lexer {
    pub fn new(source_code: &str) -> Self {
        Lexer {
            source_code: source_code.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            offset_current: 0,
            offset_start: 0,
            errors: Vec::new(),
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.offset_current += 1;
        self.source_code
            .get(self.current - 1)
            .expect("Unexpected end of stream")
            .clone()
    }

    fn is_not_empty(&self) -> bool {
        self.source_code.get(self.current).is_some()
    }

    fn peek(&self, offset: i16) -> char {
        self.source_code
            .get((self.current as i16 + offset) as usize)
            .unwrap_or(&'\0')
            .clone()
    }

    fn next_matches(&mut self, to_match: char) -> bool {
        if to_match == self.peek(0) {
            self.advance();
            return true;
        }
        false
    }

    fn next_line(&mut self) {
        self.line += 1;
        self.offset_current = 0;
        self.offset_start = 0;
    }

    // TODO: while skipping line and there is no new line at the end the program crashes
    fn skip_line(&mut self) {
        while self.peek(0) != '\n' {
            self.advance();
        }
        self.next_line();
    }

    fn next_comment(&mut self) -> bool {
        if self.peek(0) == '/' {
            self.skip_line();
            return true;
        }
        false
    }

    fn raise_error(&mut self, error_type: ErrorType) -> Result<Token, Error> {
        Err(Error {
            token: Token {
                token_type: TokenType::Invalid,
                start: self.offset_start + 1,
                end: self.offset_current,
                line: self.line,
            },
            error_type,
        })
    }

    fn create_token(&self, token_type: TokenType) -> Result<Token, Error> {
        Ok(Token {
            line: self.line,
            start: self.offset_start + 1,
            end: self.offset_current,
            token_type,
        })
    }

    fn get_slice(&self) -> String {
        self.source_code
            .get(self.start..self.current)
            .expect("Unexpected end of stream")
            .into_iter()
            .collect::<String>()
    }

    fn get_string(&mut self) -> Result<Token, Error> {
        while self.is_not_empty() {
            let c = self.advance();
            if c == '"' {
                let slice = self.get_slice();
                let value = slice.chars().skip(1).take(&slice.len() - 2).collect();

                return self.create_token(TokenType::Literal(Literal::String(value)));
            }
        }
        self.raise_error(ErrorType::StringNotClosed)
    }

    fn omit_number(&mut self) {
        while self.peek(0).is_digit(10) {
            self.advance();
        }
    }

    fn get_number(&mut self) -> Result<Token, Error> {
        self.omit_number();

        if self.peek(0) == '.' && self.peek(1).is_digit(10) {
            self.omit_number();
        }

        let num = self.get_slice().parse::<f64>();

        match num {
            Ok(value) => self.create_token(TokenType::Literal(Literal::Number(value))),
            _ => self.raise_error(ErrorType::UnexpectedCharacter),
        }
    }

    fn get_identifier(&mut self) -> Result<Token, Error> {
        let mut identifier_literal = String::new();
        loop {
            // check if identifier is one of the keywords
            identifier_literal = self.get_slice();
            let keyword = KEYWORDS.get::<str>(&identifier_literal);
            if let Some(token_type) = keyword {
                return self.create_token(token_type.clone());
            } else if !self.peek(0).is_alphabetic() {
                break;
            } else {
                self.advance();
            }
        }

        // if not return it as an identifier
        self.create_token(TokenType::Identifier(identifier_literal))
    }

    fn get_literal(&mut self) -> Result<Token, Error> {
        let c = self.peek(-1);

        if c == '"' {
            self.get_string()
        } else if c.is_digit(10) {
            self.get_number()
        } else if c.is_alphanumeric() {
            self.get_identifier()
        } else {
            self.raise_error(ErrorType::UnexpectedCharacter)
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<Error>> {
        while self.is_not_empty() {
            // early match to discard items that won't return token type
            self.start = self.current;
            self.offset_start = self.offset_current;

            let c = self.advance();
            match c {
                ' ' | '\t' | '\r' => {
                    continue;
                }
                '\n' => {
                    self.next_line();
                    continue;
                }
                '/' => {
                    if self.next_comment() {
                        continue;
                    }
                }
                _ => (),
            }

            let token_type: Option<TokenType> = match c {
                '(' => Some(TokenType::OpenParenthesis),
                ')' => Some(TokenType::CloseParenthesis),
                '{' => Some(TokenType::OpenBrace),
                '}' => Some(TokenType::CloseBrace),
                ',' => Some(TokenType::Coma),
                '.' => Some(TokenType::Dot),
                '-' => Some(TokenType::Minus),
                '+' => Some(TokenType::Plus),
                '*' => Some(TokenType::Star),
                ';' => Some(TokenType::Semicolon),
                '%' => Some(TokenType::Modulo),
                '|' => Some(TokenType::Bar),
                '!' => {
                    let token_type = if self.next_matches('=') {
                        TokenType::BangEquals
                    } else {
                        TokenType::Bang
                    };
                    Some(token_type)
                }
                '<' => {
                    let token_type = if self.next_matches('=') {
                        TokenType::LessEquals
                    } else {
                        TokenType::Less
                    };
                    Some(token_type)
                }
                '>' => {
                    let token_type = if self.next_matches('=') {
                        TokenType::GreaterEquals
                    } else {
                        TokenType::Greater
                    };
                    Some(token_type)
                }
                '=' => {
                    let token_type = if self.next_matches('=') {
                        TokenType::Compare
                    } else if self.next_matches('>') {
                        TokenType::Arrow
                    } else {
                        TokenType::Assign
                    };
                    Some(token_type)
                }
                '/' => Some(TokenType::Divide),
                _ => None,
            };

            let token = match token_type {
                Some(t) => self.create_token(t),
                None => self.get_literal(),
            };

            match token {
                Ok(t) => {
                    self.tokens.push(t);
                }
                Err(e) => {
                    self.errors.push(e);
                }
            }
        }

        self.tokens.push(self.create_token(TokenType::EOF).unwrap());

        if self.errors.len() > 0 {
            return Err(self.errors.clone());
        }
        Ok(self.tokens.clone())
    }
}

mod tests {
    use crate::lexer::Lexer;
    use crate::token::{Literal, Token, TokenType};
    #[cfg(test)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn parse_literals() {
        let code = "((10 * 5) + 5) - 3 == 20";
        let mut lexer = Lexer::new(code);
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    line: 1,
                    start: 1,
                    end: 1,
                    token_type: TokenType::OpenParenthesis
                },
                Token {
                    line: 1,
                    start: 2,
                    end: 2,
                    token_type: TokenType::OpenParenthesis
                },
                Token {
                    line: 1,
                    start: 3,
                    end: 4,
                    token_type: TokenType::Literal(Literal::Number(10.0)),
                },
                Token {
                    line: 1,
                    start: 6,
                    end: 6,
                    token_type: TokenType::Star
                },
                Token {
                    line: 1,
                    start: 8,
                    end: 8,
                    token_type: TokenType::Literal(Literal::Number(5.0))
                },
                Token {
                    line: 1,
                    start: 9,
                    end: 9,
                    token_type: TokenType::CloseParenthesis
                },
                Token {
                    line: 1,
                    start: 11,
                    end: 11,
                    token_type: TokenType::Plus
                },
                Token {
                    line: 1,
                    start: 13,
                    end: 13,
                    token_type: TokenType::Literal(Literal::Number(5.0))
                },
                Token {
                    line: 1,
                    start: 14,
                    end: 14,
                    token_type: TokenType::CloseParenthesis
                },
                Token {
                    line: 1,
                    start: 16,
                    end: 16,
                    token_type: TokenType::Minus
                },
                Token {
                    line: 1,
                    start: 18,
                    end: 18,
                    token_type: TokenType::Literal(Literal::Number(3.0))
                },
                Token {
                    line: 1,
                    start: 20,
                    end: 21,
                    token_type: TokenType::Compare,
                },
                Token {
                    line: 1,
                    start: 23,
                    end: 24,
                    token_type: TokenType::Literal(Literal::Number(20.0))
                },
                Token {
                    line: 1,
                    start: 23,
                    end: 24,
                    token_type: TokenType::EOF
                }
            ]
        )
    }
}
