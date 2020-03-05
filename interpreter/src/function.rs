use crate::class::Instance;
use crate::environment::Environment;
use crate::error::{error, Error, ErrorType};
use crate::expr::Expr;
use crate::interpreter::Interpreter;
use crate::runtime_value::Value;
use crate::statement::Stmt;
use crate::token::Token;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: &Vec<Value>) -> Result<Value, Error>;
}

#[derive(Clone, Debug)]
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
        this: Option<Rc<RefCell<Instance>>>,
        closure: Rc<RefCell<Environment>>,
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
        let val = match self {
            Function::Standard {
                params,
                name,
                body,
                token,
                this,
                closure,
            } => {
                let mut env = Environment::from(closure);
                if self.arity() != args.len() {
                    return error(token, ErrorType::InvalidNumberOfArguments);
                }

                if let Some(instance) = this {
                    let inst = Value::Instance(instance.clone());
                    env.define_or_update("this", &inst);

                    if let Some(super_instance) = instance.borrow().get_super() {
                        let super_obj = Value::Class(super_instance);
                        env.define_or_update("super", &super_obj);
                    }
                }

                for (arg, name) in args.into_iter().zip(params.into_iter()) {
                    env.define_or_update(name, arg);
                }
                let val = interpreter.execute_block(body, Rc::new(RefCell::new(env)))?;
                val
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

    pub fn bind(self, instance: Rc<RefCell<Instance>>) -> Self {
        match self {
            Function::Standard {
                params,
                name,
                body,
                token,
                closure,
                ..
            } => Function::Standard {
                params,
                name,
                body,
                token,
                this: Some(instance),
                closure,
            },
            _ => self,
        }
    }
}
