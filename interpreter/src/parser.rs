use super::token::{Literal, TokenType};
use crate::ast::print_ast;
use crate::error::{Error, ErrorType};
use crate::expr::Expr;
use crate::statement::Stmt;
use crate::token::Token;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

type ExprResult = Result<Expr, Error>;
type StmtResult = Result<Stmt, Error>;

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.current)
            .expect("Unexpected peek into empty stream")
    }

    fn previous(&self) -> &Token {
        self.tokens
            .get(self.current - 1)
            .expect("Unexpected failure to get previous token")
    }

    fn next_matches(&mut self, to_match: Vec<TokenType>) -> bool {
        if to_match.contains(&self.peek().token_type) {
            self.advance();
            return true;
        } else {
            return false;
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn consume(&mut self, expected: TokenType, error_type: ErrorType) -> Result<(), Error> {
        if self.peek().token_type == expected {
            self.advance();
            Ok(())
        } else {
            Err(Error {
                token: self.advance().clone(),
                error_type,
            })
        }
    }

    pub fn parse_tokens(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.is_at_end() {
            let stmt = self.statement()?;
            statements.push(stmt);
        }

        return Ok(statements);
    }

    fn error<T>(&mut self, error_type: ErrorType) -> Result<T, Error> {
        let token = self.previous();
        Err(Error {
            token: token.clone(),
            error_type,
        })
    }

    fn statement(&mut self) -> StmtResult {
        if self.next_matches(vec![TokenType::Print]) {
            self.print_statement()
        } else {
            self.expr_statement()
        }
    }

    fn print_statement(&mut self) -> StmtResult {
        let expr = self.expr()?;
        self.consume(TokenType::Semicolon, ErrorType::ExpectedSemicolon)?;
        Ok(Stmt::Print { expr })
    }

    fn expr_statement(&mut self) -> StmtResult {
        let expr = self.expr()?;
        self.consume(TokenType::Semicolon, ErrorType::ExpectedSemicolon)?;
        Ok(Stmt::Expr { expr })
    }

    fn expr(&mut self) -> ExprResult {
        self.equality()
    }

    fn equality(&mut self) -> ExprResult {
        let mut expr = self.comparison()?;
        while self.next_matches(vec![TokenType::Equals, TokenType::BangEquals]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Ok(expr);
    }

    fn comparison(&mut self) -> ExprResult {
        let mut expr = self.addition()?;
        while self.next_matches(vec![
            TokenType::Less,
            TokenType::LessEquals,
            TokenType::Greater,
            TokenType::GreaterEquals,
        ]) {
            let operator = self.previous().clone();
            let right = self.addition()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Ok(expr);
    }

    fn addition(&mut self) -> ExprResult {
        let mut expr = self.multiplication()?;
        while self.next_matches(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.multiplication()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Ok(expr);
    }

    fn multiplication(&mut self) -> ExprResult {
        let mut expr = self.unary()?;
        while self.next_matches(vec![TokenType::Star, TokenType::Divide]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Ok(expr);
    }

    fn unary(&mut self) -> ExprResult {
        if self.next_matches(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                expr: Box::new(right),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> ExprResult {
        let token = self.advance();
        match &token.token_type {
            TokenType::Literal(literal) => Ok(Expr::Literal {
                value: literal.clone(),
            }),
            TokenType::OpenParenthesis => {
                let body = self.expr()?;
                self.consume(TokenType::CloseParenthesis, ErrorType::UnclosedParenthesis)?;
                Ok(Expr::Grouping {
                    expr: Box::new(body),
                })
            }
            _ => self.error::<Expr>(ErrorType::UnparsableExpression),
        }
    }
}
