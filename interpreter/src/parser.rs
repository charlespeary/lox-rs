use super::token::{Literal, TokenType};
use crate::ast::print_ast;
use crate::error::{Error, ErrorType};
use crate::expr::Expr;
use crate::statement::Stmt;
use crate::token::Token;
use crate::token::TokenType::{CloseParenthesis, Var};
use std::mem;

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

    fn consume(&mut self, expected: TokenType, error_type: ErrorType) -> Result<&Token, Error> {
        if mem::discriminant((&self.peek().token_type)) == mem::discriminant(&expected) {
            Ok((self.advance()))
        } else {
            Err(Error {
                token: self.advance().clone(),
                error_type,
            })
        }
    }

    fn error<T>(&mut self, error_type: ErrorType, token: &Token) -> Result<T, Error> {
        Err(Error {
            token: token.clone(),
            error_type,
        })
    }

    pub fn parse_tokens(&mut self) -> Result<Vec<Stmt>, Vec<Error>> {
        let mut statements: Vec<Stmt> = Vec::new();
        let mut errors: Vec<Error> = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(s) => statements.push(s),
                Err(e) => errors.push(e),
            }
        }
        match errors.is_empty() {
            true => Ok(statements),
            false => Err(errors),
        }
    }

    fn get_identifier(&mut self) -> Result<(String, Token), Error> {
        let token = self.advance().clone();
        if let TokenType::Identifier(identifier) = &token.token_type {
            Ok((identifier.clone(), token.clone()))
        } else {
            self.error(ErrorType::ExpectedIdentifier, &token)
        }
    }

    fn declaration(&mut self) -> StmtResult {
        if self.next_matches(vec![TokenType::Var]) {
            self.variable()
        } else if self.next_matches(vec![TokenType::Function]) {
            self.function_statement()
        } else if self.next_matches(vec![TokenType::Class]) {
            self.class_statement()
        } else {
            self.statement()
        }
    }

    fn variable(&mut self) -> StmtResult {
        let (name, _) = self.get_identifier()?;
        let expr = if self.next_matches(vec![TokenType::Assign]) {
            Some(self.expr()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, ErrorType::ExpectedSemicolon)?;
        return Ok(Stmt::Var { name, value: expr });
    }

    fn statement(&mut self) -> StmtResult {
        // TODO: maybe a match would be prettier here
        if self.next_matches(vec![TokenType::Print]) {
            self.print_statement()
        } else if self.next_matches(vec![TokenType::For]) {
            self.for_stmt()
        } else if self.next_matches(vec![TokenType::OpenBrace]) {
            self.block()
        } else if self.next_matches(vec![TokenType::Return]) {
            self.return_stmt()
        } else if self.next_matches(vec![TokenType::If]) {
            self.if_statement()
        } else if self.next_matches(vec![TokenType::While]) {
            self.while_statement()
        } else if self.next_matches(vec![TokenType::Break, TokenType::Continue]) {
            self.break_or_continue_statement()
        } else {
            self.expr_statement()
        }
    }

    fn return_stmt(&mut self) -> StmtResult {
        let token = self.previous().clone();
        let value = Some(self.expr()?);

        self.consume(TokenType::Semicolon, ErrorType::ExpectedSemicolon)?;

        Ok(Stmt::Return { value, token })
    }

    fn block(&mut self) -> StmtResult {
        let mut stmts: Vec<Stmt> = Vec::new();

        while &self.peek().token_type != &TokenType::CloseBrace && !self.is_at_end() {
            let stmt = self.declaration()?;
            stmts.push(stmt);
        }
        self.consume(TokenType::CloseBrace, ErrorType::ExpectedBlockEnd)?;
        Ok(Stmt::Block { stmts })
    }

    fn parse_params(&mut self, delimiter: TokenType) -> Result<Vec<String>, Error> {
        let mut params: Vec<String> = Vec::new();
        if self.peek().token_type != delimiter {
            loop {
                let token = self.advance().clone();

                if let TokenType::Identifier(param) = token.token_type {
                    params.push(param);
                } else {
                    let token = self.previous().clone();
                    return self.error(ErrorType::UnexpectedCharacter, &token);
                }

                if !self.next_matches(vec![TokenType::Coma]) {
                    break;
                }
            }
        }

        let error_type = match delimiter {
            TokenType::Bar => ErrorType::ExpectedCloseBar,
            _ => ErrorType::ExpectedCloseParenthesis,
        };

        self.consume(delimiter, error_type)?;

        Ok(params)
    }

    fn class_statement(&mut self) -> StmtResult {
        let (name, token) = self.get_identifier()?;
        let superclass = if self.next_matches(vec![TokenType::Inherit]) {
            let (name, token) = self.get_identifier()?;
            Some(Expr::Var { name, token })
        } else {
            None
        };

        self.consume(TokenType::OpenBrace, ErrorType::ExpectedBlockStart)?;

        let mut members: Vec<Stmt> = Vec::new();

        while !self.next_matches(vec![TokenType::CloseBrace, TokenType::EOF]) {
            let val = if self.next_matches(vec![TokenType::Var]) {
                self.variable()
            } else {
                self.function_statement()
            };
            members.push(val?);
        }

        Ok(Stmt::Class {
            name,
            token,
            members,
            superclass,
        })
    }

    fn function_statement(&mut self) -> StmtResult {
        let (name, token) = self.get_identifier()?;

        self.consume(
            TokenType::OpenParenthesis,
            ErrorType::ExpectedOpenParenthesis,
        )?;

        let params = self.parse_params(TokenType::CloseParenthesis)?;

        self.consume(TokenType::OpenBrace, ErrorType::ExpectedBlockStart)?;
        let body = vec![self.block()?];

        Ok(Stmt::Function {
            params,
            body,
            name,
            token,
        })
    }

    fn for_stmt(&mut self) -> StmtResult {
        self.consume(
            TokenType::OpenParenthesis,
            ErrorType::ExpectedOpenParenthesis,
        )?;
        let initializer = self.declaration()?;
        let condition = self.expr()?;
        self.consume(TokenType::Semicolon, ErrorType::ExpectedSemicolon)?;
        let executor = self.expr()?;
        self.consume(
            TokenType::CloseParenthesis,
            ErrorType::ExpectedCloseParenthesis,
        )?;

        // TODO: I feel like this allows infinite amount of for loops after for loop and some other pointless stuff
        let body = self.statement()?;

        let body = Box::new(Stmt::Block {
            stmts: vec![body, Stmt::Expr { expr: executor }],
        });

        let while_loop = Stmt::While { condition, body };

        Ok(Stmt::Block {
            stmts: vec![initializer, while_loop],
        })
    }

    fn while_statement(&mut self) -> StmtResult {
        self.consume(
            TokenType::OpenParenthesis,
            ErrorType::ExpectedCloseParenthesis,
        )?;
        let condition = self.expr()?;
        self.consume(
            TokenType::CloseParenthesis,
            ErrorType::ExpectedCloseParenthesis,
        )?;
        let body = Box::new(self.statement()?);

        Ok(Stmt::While { condition, body })
    }

    fn if_statement(&mut self) -> StmtResult {
        self.consume(
            TokenType::OpenParenthesis,
            ErrorType::ExpectedOpenParenthesis,
        )?;
        let condition = self.expr()?;
        self.consume(
            TokenType::CloseParenthesis,
            ErrorType::ExpectedCloseParenthesis,
        )?;
        let then_body = Box::new(self.statement()?);
        let else_body = if self.next_matches(vec![TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_body,
            else_body,
        })
    }

    fn break_or_continue_statement(&mut self) -> StmtResult {
        let token = self.previous().clone();
        let stmt = if self.previous().token_type == TokenType::Break {
            Stmt::Break { token }
        } else {
            Stmt::Continue { token }
        };

        self.consume(TokenType::Semicolon, ErrorType::ExpectedSemicolon)?;
        Ok(stmt)
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
        self.closure()
    }

    fn closure(&mut self) -> ExprResult {
        let token = self.previous().clone();

        if self.next_matches(vec![TokenType::Bar]) {
            let params = self.parse_params(TokenType::Bar)?;
            self.consume(TokenType::Arrow, ErrorType::ExpectedArrow)?;
            let body = if self.next_matches(vec![TokenType::OpenBrace]) {
                self.block()?
            } else {
                let expr = self.expr()?;
                Stmt::Return {
                    value: Some(expr),
                    token: token.clone(),
                }
            };
            return Ok(Expr::Closure {
                params,
                body: vec![body],
                name: String::from("closure"),
                token,
            });
        }
        self.assignment()
    }

    fn assignment(&mut self) -> ExprResult {
        let mut expr = self.or()?;

        if self.next_matches(vec![TokenType::Assign]) {
            let token = self.previous().clone();

            let value = self.assignment()?;

            if let Expr::Var { name, token } = expr {
                return Ok(Expr::Assign {
                    name,
                    expr: Box::new(value),
                    token,
                });
            }

            if let Some((name, token, obj)) = expr.as_get() {
                return Ok(Expr::Set {
                    token: token.clone(),
                    obj: obj.clone(),
                    name: name.clone(),
                    value: Box::new(value),
                });
            }

            self.error::<Expr>(ErrorType::InvalidAssignment, &token);
        }

        Ok(expr)
    }

    fn or(&mut self) -> ExprResult {
        let mut expr = self.and()?;
        while self.next_matches(vec![TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Ok(expr);
    }

    fn and(&mut self) -> ExprResult {
        let mut expr = self.equality()?;
        while self.next_matches(vec![TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Ok(expr);
    }

    fn equality(&mut self) -> ExprResult {
        let mut expr = self.comparison()?;
        while self.next_matches(vec![TokenType::Compare, TokenType::BangEquals]) {
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
        while self.next_matches(vec![TokenType::Star, TokenType::Divide, TokenType::Modulo]) {
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
        self.call()
    }

    fn finish_call(&mut self, callee: Expr) -> ExprResult {
        let mut arguments: Vec<Expr> = Vec::new();

        if !(self.peek().token_type == TokenType::CloseParenthesis) {
            loop {
                arguments.push(self.expr()?);
                if !self.next_matches(vec![TokenType::Coma]) {
                    break;
                }
            }
        }

        let token = self
            .consume(
                TokenType::CloseParenthesis,
                ErrorType::ExpectedCloseParenthesis,
            )?
            .clone();

        if arguments.len() >= 255 {
            println!("Function exceeded maximum number of arguments");
            //            self.error(ErrorType::MaximumArguments, &token)
        }

        Ok(Expr::Call {
            callee: Box::new(callee),
            arguments,
            token,
        })
    }

    fn call(&mut self) -> ExprResult {
        let mut expr = self.primary()?;
        loop {
            if self.next_matches(vec![TokenType::OpenParenthesis]) {
                expr = self.finish_call(expr)?;
            } else if self.next_matches(vec![TokenType::Dot]) {
                let (name, token) = self.get_identifier()?;
                expr = Expr::Get {
                    expr: Box::new(expr),
                    name,
                    token,
                };
            } else {
                break;
            }
        }

        return Ok(expr);
    }

    fn primary(&mut self) -> ExprResult {
        let token = self.advance();
        let _token = token.clone();

        match &token.token_type {
            TokenType::Literal(literal) => Ok(Expr::Literal {
                value: literal.clone(),
            }),
            TokenType::Identifier(name) => Ok(Expr::Var {
                name: name.clone(),
                token: token.clone(),
            }),
            TokenType::This => Ok(Expr::This {
                token: token.clone(),
            }),
            TokenType::Super => {
                self.consume(TokenType::Dot, ErrorType::DotAfterSuper)?;
                let (method_name, token) = self.get_identifier()?;
                Ok(Expr::Super { method_name, token })
            }
            TokenType::OpenParenthesis => {
                let body = self.expr()?;
                self.consume(TokenType::CloseParenthesis, ErrorType::UnclosedParenthesis)?;
                Ok(Expr::Grouping {
                    expr: Box::new(body),
                })
            }
            _ => {
                // TODO: figure out better name
                self.error::<Expr>(ErrorType::UnparsableExpression, &_token)
            }
        }
    }
}
