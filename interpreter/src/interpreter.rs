use crate::environment::Environment;
use crate::error::{Error, ErrorType};
use crate::expr::{Expr, Visitor as ExprVisitor};
use crate::runtime_value::Value;
use crate::statement::{Stmt, Visitor as StmtVisitor};
use crate::token::{Literal, Token, TokenType};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, Error> {
        expr.accept(self)
    }

    pub fn interpret(&mut self, stmts: &Vec<Stmt>) -> Result<(), Error> {
        for stmt in stmts {
            stmt.accept(self)?;
        }
        Ok(())
    }
}

fn error(token: &Token, error_type: ErrorType) -> Result<Value, Error> {
    Err(Error {
        token: token.clone(),
        error_type,
    })
}

impl ExprVisitor<Value> for Interpreter {
    fn visit_binary(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Value, Error> {
        let a = self.evaluate(left)?;
        let b = self.evaluate(right)?;

        match operator.token_type {
            TokenType::Plus => match (a, b) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::String(a), Value::String(b)) => Ok(Value::String([a, b].concat())),
                _ => error(operator, ErrorType::WrongType),
            },
            TokenType::Minus => match (a, b) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                _ => error(operator, ErrorType::WrongType),
            },
            TokenType::Star => match (a, b) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                _ => error(operator, ErrorType::WrongType),
            },
            TokenType::Divide => match (a, b) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
                _ => error(operator, ErrorType::WrongType),
            },
            TokenType::BangEquals => match (a, b) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a != b)),
                (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a != b)),
                (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a != b)),
                (Value::Null, Value::Null) => Ok(Value::Boolean(false)),
                _ => error(operator, ErrorType::WrongType),
            },
            TokenType::Compare => match (a, b) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a == b)),
                (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a == b)),
                (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a == b)),
                (Value::Null, Value::Null) => Ok(Value::Boolean(true)),
                _ => error(operator, ErrorType::WrongType),
            },
            TokenType::Less => match (a, b) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)),
                (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a.len() < b.len())),
                _ => error(operator, ErrorType::WrongType),
            },
            TokenType::LessEquals => match (a, b) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a <= b)),
                (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a.len() <= b.len())),
                _ => error(operator, ErrorType::WrongType),
            },
            TokenType::Greater => match (a, b) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)),
                (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a.len() > b.len())),
                _ => error(operator, ErrorType::WrongType),
            },
            TokenType::GreaterEquals => match (a, b) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a >= b)),
                (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a.len() >= b.len())),
                _ => error(operator, ErrorType::WrongType),
            },
            _ => unreachable!(),
        }
    }

    fn visit_literal(&mut self, literal: &Literal) -> Result<Value, Error> {
        Ok(Value::new(literal))
    }

    fn visit_unary(&mut self, operator: &Token, expr: &Expr) -> Result<Value, Error> {
        let val = self.evaluate(expr)?;

        match operator.token_type {
            TokenType::Minus => match val {
                Value::Number(val) => Ok(Value::Number(val * -1.0)),
                _ => error(operator, ErrorType::WrongType),
            },
            TokenType::Bang => Ok(Value::Boolean(!val.to_bool())),
            _ => unreachable!(),
        }
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Result<Value, Error> {
        self.evaluate(expr)
    }

    fn visit_var(&mut self, name: &String, token: &Token) -> Result<Value, Error> {
        let var = self.env.borrow().get(name);
        // not sure if we should clone anything at all here
        match var {
            Some(val) => Ok(val.clone()),
            None => error(token, ErrorType::UndefinedVariable),
        }
    }

    fn visit_assignment(
        &mut self,
        name: &String,
        expr: &Expr,
        token: &Token,
    ) -> Result<Value, Error> {
        let value = self.evaluate(expr)?;
        if let Some(val) = self.env.borrow_mut().define_or_update(name, &value) {
            Ok(val)
        } else {
            error(token, ErrorType::UndefinedVariable)
        }
    }

    fn visit_logical(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Value, Error> {
        // TODO: this needs more testing
        let left_val = self.evaluate(left)?;
        let right_val = self.evaluate(right)?;
        let res = match operator.token_type {
            TokenType::Or => {
                if left_val.to_bool() {
                    left_val
                } else {
                    right_val
                }
            }
            TokenType::And => Value::Boolean(left_val.to_bool() && right_val.to_bool()),
            _ => right_val,
        };
        Ok(res)
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), Error> {
        let value = self.evaluate(expr)?;
        println!("{}", value.to_string());
        Ok(())
    }

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<(), Error> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_var(&mut self, name: &String, expr: &Expr) -> Result<(), Error> {
        let value = self.evaluate(expr)?;
        self.env.borrow_mut().define_or_update(name, &value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> Result<(), Error> {
        // TODO: figure out if I can avoid the clones
        let mut prev_env = self.env.clone();
        self.env = Rc::new(RefCell::new(Environment::from(&self.env)));
        self.interpret(statements);
        self.env = prev_env;
        Ok(())
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_body: &Stmt,
        else_body: &Stmt,
    ) -> Result<(), Error> {
        let cond = self.evaluate(condition)?.to_bool();
        if cond {
            then_body.accept(self)?;
        } else {
            else_body.accept(self)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<(), Error> {
        loop {
            if self.evaluate(condition)?.to_bool() {
                body.accept(self)?;
            } else {
                break;
            }
        }
        Ok(())
    }
}
