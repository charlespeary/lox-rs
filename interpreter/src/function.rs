use crate::environment::Environment;
use crate::error::{error, Error, ErrorType};
use crate::expr::Expr;
use crate::interpreter::Interpreter;
use crate::runtime_value::Value;
use crate::statement::Stmt;
use crate::token::Token;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: &Vec<Value>) -> Result<Value, Error>;
}

#[derive(Clone)]
pub enum Function {
    Native {
        arity: usize,
        body: fn() -> Value,
    },
    Standard {
        params: Vec<String>,
        name: String,
        body: Vec<Stmt>,
        token: Token,
    },
}

impl Callable for Function {
    fn arity(&self) -> usize {
        match self {
            Function::Native { arity, body } => *arity,
            Function::Standard { params, .. } => params.len(),
        }
    }

    fn call(&self, interpreter: &mut Interpreter, args: &Vec<Value>) -> Result<Value, Error> {
        let env = Rc::new(RefCell::new(Environment::from(&interpreter.env)));

        let val = match self {
            Function::Standard {
                params,
                name,
                body,
                token,
            } => {
                if self.arity() != args.len() {
                    return error(token, ErrorType::InvalidNumberOfArguments);
                }

                for (arg, name) in args.into_iter().zip(params.into_iter()) {
                    env.borrow_mut().define_or_update(name, arg);
                }
                interpreter.execute_block(body, env)?
            }
            Function::Native { body, .. } => body(),
        };

        Ok(val)
    }
}

impl Function {
    pub fn to_string(&self) -> String {
        match self {
            Function::Native { .. } => String::from("<native function>"),
            Function::Standard { name, .. } => format!("<{} function>", name),
        }
    }
}
