use super::token::{Literal, Token, TokenType, KEYWORDS};
use crate::errors::{Error, ErrorType};
use crate::utils::is_numeric;
use std::iter::Peekable;
use std::str::Chars;


pub struct Lexer {
  source_code: Vec<char>,
  pub tokens: Vec<Token>,
  current: usize,
  line_offset: usize,
  errors: Vec<Error>,
}

impl Lexer {
  pub fn new(source_code: &str) -> Self {
    Lexer {
      source_code: source_code.chars().collect(),
      tokens: Vec::new(),
      current: 0,
      line_offset: 1,
      errors: Vec::new(),
    }
  }
  
  
  fn get_current(&self) -> Option<char> {
    self.source_code.get(self.current).cloned()
  }
  
  fn peek(&self) -> Option<char> {
    self.source_code
        .get(self.current + 1).cloned()
  }
  
  fn previous(&self) -> Option<char> {
    self.source_code
        .get(self.current - 1).cloned()
  }
  
  fn next_matches(&mut self, to_match: char) -> bool {
    match self.get_current() {
      Some(t) => {
        if to_match == t {
          self.advance();
          return true;
        }
        false
      }
      _ => false,
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
  
  fn advance(&mut self) -> char {
    let item = self.source_code[self.current];
    self.current += 1;
    item
  }
  
  fn has_next(&self) -> bool {
    self.peek().is_some()
  }
  
  fn get_string(&mut self) -> TokenType {
    let mut value = String::new();
    while self.has_next() {
      let c = self.advance();
      if c == '"' {
        return TokenType::Literal(Literal::String(value));
      } else {
        value.push(c);
      }
    }
    self.raise_error(ErrorType::StringNotClosed);
    TokenType::Literal(Literal::String(value))
  }
  
  pub fn scan_tokens(&mut self) {
    while self.has_next() {
      let c = self.advance();
      println!("{}", c);
      match c {
        ' ' | '\t' | '\r' => {
          self.line_offset += 1;
          continue;
        }
        '\n' => self.next_line(),
        '(' => TokenType::OpenParenthesis,
        ')' => TokenType::CloseParenthesis,
        '{' => TokenType::OpenBrace,
        '}' => TokenType::CloseBrace,
        ',' => TokenType::Coma,
        '.' => TokenType::Dot,
        '-' => TokenType::Minus,
        '+' => TokenType::Plus,
        '*' => TokenType::Star,
        ';' => TokenType::Semicolon,
        '!' => if self.next_matches('=') { TokenType::BangEquals } else { TokenType::Bang },
        '<' => if self.next_matches('=') { TokenType::LessEquals } else { TokenType::Less },
        '>' => if self.next_matches('=') { TokenType::GreaterEquals } else { TokenType::Greater }
        '=' => if self.next_equals('=') { self.add_token_and_advance(TokenType::Compare) } else { TokenType::Equals },
        '/' => if self.next_comment() { TokenType::Comment } else { TokenType::Divide }
        '"' => self.get_string(),
        _ => {
          if is_numeric(c) {
            return self.get_number(c);
          } else if c.is_alphanumeric() {
            let identifier = self.get_identifier(c);
            identifier;
          } else {
            self.raise_error(ErrorType::UnexpectedCharacter);
          }
        }
      };
      self.line_offset += 1;
    }
  
  }
  }
}
