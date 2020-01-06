use crate::environment::Environment;
use crate::error::{error, Error, ErrorType};
use crate::expr::{Expr, Visitor as ExprVisitor};
use crate::function::{Callable, Function};
use crate::runtime_value::Value;
use crate::statement::{Stmt, Visitor as StmtVisitor};
use crate::token::{Literal, Token, TokenType};
use std::cell::RefCell;
use std::rc::Rc;

struct State {
    should_continue: bool,
    should_break: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            should_break: false,
            should_continue: false,
        }
    }

    fn will_break(&mut self) -> bool {
        let should_break = self.should_break;
        self.should_break = false;
        should_break
    }

    fn will_continue(&mut self) -> bool {
        let should_continue = self.should_continue;
        self.should_continue = false;
        should_continue
    }
}

pub struct Interpreter {
    pub env: Rc<RefCell<Environment>>,
    globals: Rc<RefCell<Environment>>,
    state: State,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = {
            let e = Rc::new(RefCell::new(Environment::new()));
            let clock = Value::Function(Function::Native {
                arity: 0,
                body: || Value::Number(100.0),
            });

            e.borrow_mut().define_or_update("clock", &clock);
            e
        };

        Interpreter {
            env: Rc::new(RefCell::new(Environment::from(&globals))),
            state: State::new(),
            globals,
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, Error> {
        expr.accept(self)
    }

    pub fn interpret(&mut self, stmts: &Vec<Stmt>) -> Result<Value, Error> {
        let mut last_val: Option<Value> = None;
        for stmt in stmts {
            if self.state.will_continue() || self.state.should_break {
                break;
            }
            last_val = Some(stmt.accept(self)?);
        }
        Ok(last_val.map_or_else(|| Value::Null, |v| v))
    }

    pub fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, Error> {
        let mut prev_env = self.env.clone();
        self.env = env;
        let val = self.interpret(statements)?;
        self.env = prev_env;
        Ok(val)
    }
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
            TokenType::Modulo => match (a, b) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a % b)),
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

    fn visit_call(
        &mut self,
        callee: &Expr,
        token: &Token,
        arguments: &Vec<Expr>,
    ) -> Result<Value, Error> {
        let callee = self.evaluate(callee)?;

        let args: Result<Vec<Value>, Error> = arguments.iter().map(|a| self.evaluate(a)).collect();

        match callee {
            Value::Function(func) => func.call(self, &args?),
            _ => error(token, ErrorType::ValueNotCallable),
        }
    }
}

impl StmtVisitor<Value> for Interpreter {
    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<Value, Error> {
        let value = self.evaluate(expr)?;
        println!("{}", value.to_string());
        Ok(Value::Null)
    }

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<Value, Error> {
        Ok(self.evaluate(expr)?)
    }

    fn visit_var(&mut self, name: &String, expr: &Expr) -> Result<Value, Error> {
        let value = self.evaluate(expr)?;
        self.env.borrow_mut().define_or_update(name, &value);
        Ok(value)
    }

    fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> Result<Value, Error> {
        // TODO: figure out if I can avoid the clones
        let env = Rc::new(RefCell::new(Environment::from(&self.env)));
        Ok(self.execute_block(statements, env)?)
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_body: &Stmt,
        else_body: &Option<Box<Stmt>>,
    ) -> Result<Value, Error> {
        let cond = self.evaluate(condition)?.to_bool();
        if cond {
            Ok(then_body.accept(self)?)
        } else {
            let val = match else_body {
                Some(stmt) => stmt.accept(self)?,
                _ => Value::Null,
            };
            Ok(val)
        }
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<Value, Error> {
        loop {
            if self.evaluate(condition)?.to_bool() {
                if self.state.will_break() {
                    break;
                }
                body.accept(self)?;
            } else {
                break;
            }
        }
        Ok(Value::Null)
    }

    fn visit_break_stmt(&mut self) -> Result<Value, Error> {
        self.state.should_break = true;
        Ok(Value::Null)
    }

    fn visit_continue_stmt(&mut self) -> Result<Value, Error> {
        self.state.should_continue = true;
        Ok(Value::Null)
    }

    fn visit_function_stmt(
        &mut self,
        name: &String,
        params: &Vec<String>,
        body: &Vec<Stmt>,
        token: &Token,
    ) -> Result<Value, Error> {
        // TODO: Is clone necessary? Probably not, it's ugly
        let function = Value::Function(Function::Standard {
            name: name.clone(),
            body: body.clone(),
            params: params.clone(),
            token: token.clone(),
        });

        self.env.borrow_mut().define_or_update(name, &function);

        Ok(function)
    }

    fn visit_return_stmt(&mut self, value: &Option<Expr>, token: &Token) -> Result<Value, Error> {
        let val = match value {
            Some(val) => self.evaluate(val)?,
            None => Value::Null,
        };

        Ok(val)
    }
}
