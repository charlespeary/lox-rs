use crate::class::{Class, Instance};
use crate::environment::Environment;
use crate::error::{error, Error, ErrorType};
use crate::expr::{Expr, Visitor as ExprVisitor};
use crate::function::{Callable, Function};
use crate::resolver::VarRef;
use crate::runtime_value::Value;
use crate::statement::{Stmt, Visitor as StmtVisitor};
use crate::token::{Literal, Token, TokenType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// this kind of control flow can be done with exceptions, but I'm not a big fan of that idea
struct State {
    should_continue: bool,
    should_break: bool,
    should_return: bool,
    inside_call: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            should_break: false,
            should_continue: false,
            should_return: false,
            inside_call: false,
        }
    }

    fn will_return(&mut self) -> bool {
        if self.should_return && self.inside_call {
            self.should_return = false;
            return true;
        }
        false
    }

    fn enter_call(&mut self) {
        self.inside_call = true;
    }

    fn exit_call(&mut self) {
        self.should_return = false;
        self.inside_call = false;
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
    pub distances: HashMap<String, usize>,
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
            distances: HashMap::new(),
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, Error> {
        expr.accept(self)
    }

    fn lookup_variable(&mut self, var: VarRef, token: &Token) -> Result<Value, Error> {
        let distance = self.get_distance(&var);
        let name = &var.name;
        let var = self.env.borrow().get_at(name, distance.unwrap_or(0));

        match var {
            Some(val) => Ok(val.clone()),
            None => error(token, ErrorType::UndefinedVariable),
        }
    }

    fn lookup_deep(&mut self, name: &str, token: &Token) -> Result<Value, Error> {
        self.env
            .borrow()
            .get_deep(name)
            .map_or_else(|| error(token, ErrorType::UndefinedVariable), |v| Ok(v))
    }

    fn get_distance(&self, var: &VarRef) -> Option<usize> {
        self.distances.get(&var.to_string()).map(|v| *v)
    }

    pub fn resolve_distance(&mut self, var: VarRef, depth: usize) {
        self.distances.insert(var.to_string(), depth);
    }

    pub fn interpret(&mut self, stmts: &Vec<Stmt>) -> Result<Value, Error> {
        let mut last_val: Option<Value> = None;
        for stmt in stmts {
            if self.state.will_continue() || self.state.will_return() || self.state.should_break {
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
        self.lookup_variable(VarRef::new(token, name), token)
    }

    fn visit_assignment(
        &mut self,
        name: &String,
        expr: &Expr,
        token: &Token,
    ) -> Result<Value, Error> {
        let value = self.evaluate(expr)?;
        let distance = self.get_distance(&VarRef::new(token, name));

        if let Some(dist) = distance {
            match self.env.borrow_mut().assign_at(name, &value, dist) {
                Some(val) => Ok(val),
                None => error(token, ErrorType::UndefinedVariable),
            }
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

        let result = match callee {
            Value::Function(func) => {
                self.state.enter_call();
                func.call(self, &args?)
            }
            Value::Class(class) => class.call(self, &args?),
            _ => error(token, ErrorType::ValueNotCallable),
        };

        self.state.exit_call();
        result
    }

    fn visit_closure(
        &mut self,
        args: &Vec<String>,
        body: &Vec<Stmt>,
        name: &String,
        token: &Token,
    ) -> Result<Value, Error> {
        Ok(Value::Function(Function::Standard {
            params: args.clone(),
            body: body.clone(),
            name: name.clone(),
            token: token.clone(),
            this: None,
            closure: Rc::clone(&self.env),
        }))
    }

    fn visit_get(&mut self, name: &String, token: &Token, expr: &Expr) -> Result<Value, Error> {
        let obj = self.evaluate(expr)?;
        match obj {
            Value::Instance(instance) => instance.borrow().get(name, token),
            _ => error(token, ErrorType::ValueNotInstance),
        }
    }

    fn visit_set(
        &mut self,
        token: &Token,
        name: &String,
        value: &Expr,
        obj: &Expr,
    ) -> Result<Value, Error> {
        let mut instance = self.evaluate(obj)?;

        match instance {
            Value::Instance(ref mut instance) => {
                let val = self.evaluate(value)?;
                instance.borrow_mut().set(name, token, val);
            }
            _ => return error(token, ErrorType::ValueNotInstance),
        }

        Ok(instance)
    }

    fn visit_this(&mut self, token: &Token) -> Result<Value, Error> {
        self.lookup_deep("this", token)
    }

    fn visit_super(&mut self, token: &Token, method_name: &String) -> Result<Value, Error> {
        let superclass = self.lookup_deep("super", token)?;

        match superclass.as_class().unwrap().find_method(method_name) {
            Some(method) => {
                let instance = self
                    .env
                    .borrow()
                    .get_deep("this")
                    .unwrap()
                    .as_instance()
                    .unwrap()
                    .clone();
                Ok(Value::Function(method.clone().bind(instance)))
            }
            None => error(token, ErrorType::MethodNotFound),
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

    fn visit_var(&mut self, name: &String, expr: &Option<Expr>) -> Result<Value, Error> {
        let value = match expr {
            Some(e) => self.evaluate(e)?,
            None => Value::Null,
        };
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

    fn visit_break_stmt(&mut self, token: &Token) -> Result<Value, Error> {
        self.state.should_break = true;
        Ok(Value::Null)
    }

    fn visit_continue_stmt(&mut self, token: &Token) -> Result<Value, Error> {
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
            this: None,
            closure: Rc::clone(&self.env),
        });

        self.env.borrow_mut().define_or_update(name, &function);

        Ok(function)
    }

    fn visit_class_stmt(
        &mut self,
        name: &String,
        token: &Token,
        members: &Vec<Stmt>,
        superclass: &Option<Expr>,
    ) -> Result<Value, Error> {
        self.env.borrow_mut().define_or_update(name, &Value::Null);

        let superclass = if let Some(superclass) = superclass {
            match self.evaluate(superclass)? {
                Value::Class(sc) => Some(Box::new(sc)),
                _ => return error(token, ErrorType::CanOnlyInheritFromClass),
            }
        } else {
            None
        };

        let class = Class::new(name, members, superclass, self)?;
        self.env
            .borrow_mut()
            .define_or_update(name, &Value::Class(class));
        Ok(Value::Null)
    }

    fn visit_return_stmt(&mut self, value: &Option<Expr>, token: &Token) -> Result<Value, Error> {
        let val = match value {
            Some(val) => self.evaluate(val)?,
            None => Value::Null,
        };
        self.state.should_return = true;
        Ok(val)
    }
}
