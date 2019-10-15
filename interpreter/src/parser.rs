use super::token::{Literal, TokenType};
use crate::ast::print_ast;
use crate::errors::{Error, ErrorType, ParserError};
use crate::parser::Expression::Unary;
use crate::token::Token;

#[derive(Debug, Clone, Display)]
pub enum Operator {
    #[display(fmt = "+")]
    Plus,
    #[display(fmt = "-")]
    Minus,
    #[display(fmt = "*")]
    Multiply,
    #[display(fmt = "/")]
    Divide,
}

#[derive(Debug, Clone, Display)]
pub enum UnaryOperator {
    #[display(fmt = "!")]
    Bang,
    #[display(fmt = "-")]
    Minus,
}

fn operator(token_type: TokenType) -> Operator {
    match token_type {
        TokenType::Divide => Operator::Divide,
        TokenType::Star => Operator::Multiply,
        TokenType::Minus => Operator::Minus,
        _ => Operator::Plus,
    }
}

fn unary_operator(token_type: TokenType) -> UnaryOperator {
    match token_type {
        TokenType::Minus => UnaryOperator::Minus,
        _ => UnaryOperator::Bang,
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Binary(Box<Expression>, Operator, Box<Expression>),
    Literal(Literal),
    Unary(UnaryOperator, Box<Expression>),
    Grouping(Box<Expression>),
    Error(ErrorType), // temp
}

pub enum Statement {}

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
    errors: Vec<ParserError>,
}

//type ParserResult = Result<Box<Expression>, ErrorType>;
type ParserResult = Box<Expression>;
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.errors.len() == 0
    }

    // TODO: most of this is shared between lexer and parser, move it to separate data container and reuse it
    fn get_current(&self) -> Option<Token> {
        self.tokens.get(self.current).map(|t| t.clone())
    }

    fn peek(&self) -> Option<TokenType> {
        self.tokens
            .get(self.current + 1)
            .map(|t| t.token_type.clone())
    }

    fn previous(&self) -> Option<TokenType> {
        self.tokens
            .get(self.current - 1)
            .map(|t| t.token_type.clone())
    }

    fn get_operator(&self) -> Operator {
        operator(self.previous().unwrap())
    }

    fn get_unary_operator(&self) -> UnaryOperator {
        unary_operator(self.previous().unwrap())
    }

    fn next_matches(&mut self, to_match: Vec<TokenType>) -> bool {
        match self.get_current() {
            Some(t) => {
                if to_match.contains(&t.token_type) {
                    self.advance();
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    fn advance(&mut self) -> TokenType {
        let item = self.tokens[self.current].token_type.clone();
        self.current += 1;
        item
    }

    pub fn parse_tokens(&mut self) -> Result<ParserResult, Vec<ParserError>> {
        let expr = self.expression();
        if self.is_valid() {
            return Ok(expr);
        }
        Err(self.errors.clone())
    }

    fn error(&mut self, error_type: ErrorType) -> Expression {
        let token = self.get_current().unwrap();
        let error = ParserError {
            error: Error {
                line: token.line,
                line_offset: token.line_offset,
                error_type: error_type.clone(),
            },
        };
        self.errors.push(error);
        Expression::Error(error_type)
    }

    fn expression(&mut self) -> ParserResult {
        self.equality()
    }

    fn equality(&mut self) -> ParserResult {
        let mut expr = self.comparison();
        while self.next_matches(vec![TokenType::Bang, TokenType::BangEquals]) {
            let operator = self.get_operator();
            let right = self.comparison();
            expr = Box::new(Expression::Binary(expr, operator, right));
        }
        return expr;
    }

    fn comparison(&mut self) -> ParserResult {
        let mut expr = self.addition();
        while self.next_matches(vec![
            TokenType::Less,
            TokenType::LessEquals,
            TokenType::Greater,
            TokenType::GreaterEquals,
        ]) {
            let operator = self.get_operator();
            let right = self.addition();
            expr = Box::new(Expression::Binary(expr, operator, right));
        }
        return expr;
    }

    fn addition(&mut self) -> ParserResult {
        let mut expr = self.multiplication();
        while self.next_matches(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.get_operator();
            let right = self.multiplication();
            expr = Box::new(Expression::Binary(expr, operator, right));
        }
        return expr;
    }

    fn multiplication(&mut self) -> ParserResult {
        let mut expr = self.unary();
        while self.next_matches(vec![TokenType::Star, TokenType::Divide]) {
            let operator = self.get_operator();
            let right = self.unary();
            expr = Box::new(Expression::Binary(expr, operator, right));
        }
        return expr;
    }

    fn unary(&mut self) -> ParserResult {
        if self.next_matches(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.get_unary_operator();
            let right = self.unary();
            return Box::new(Expression::Unary(operator, right));
        }
        self.primary()
    }

    fn primary(&mut self) -> ParserResult {
        let expr = match self.advance() {
            TokenType::Literal(literal) => Expression::Literal(literal),
            TokenType::OpenParenthesis => {
                let body = self.expression();
                if self.next_matches(vec![TokenType::CloseParenthesis]) {
                    Expression::Grouping(body)
                } else {
                    self.error(ErrorType::UnclosedParenthesis)
                }
            }
            _ => self.error(ErrorType::UnparsableExpression),
        };
        Box::new(expr)
    }
}
