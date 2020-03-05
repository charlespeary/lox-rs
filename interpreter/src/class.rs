use crate::environment::Environment;
use crate::error::{error, Error, ErrorType};
use crate::expr::Expr;
use crate::function::{Callable, Function};
use crate::interpreter::Interpreter;
use crate::runtime_value::Value;
use crate::statement::Stmt;
use crate::token::Literal;
use crate::token::{Token, TokenType};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

type Properties = HashMap<String, Value>;
type Methods = HashMap<String, Function>;
type Superclass = Option<Box<Class>>;

#[derive(Debug, Clone)]
pub struct Class {
    name: String,
    properties: Properties,
    methods: Methods,
    superclass: Superclass,
}

impl Class {
    pub fn new(
        name: &String,
        members: &Vec<Stmt>,
        superclass: Option<Box<Class>>,
        interpreter: &mut Interpreter,
    ) -> Result<Self, Error> {
        let mut properties: HashMap<String, Value> = HashMap::new();
        let mut methods: HashMap<String, Function> = HashMap::new();

        for member in members {
            match member {
                Stmt::Var { name, value } => {
                    let val = value.clone().unwrap_or(Expr::Literal {
                        value: Literal::Null,
                    });
                    properties.insert(name.clone(), interpreter.evaluate(&val)?);
                }
                Stmt::Function {
                    name,
                    token,
                    body,
                    params,
                } => {
                    methods.insert(
                        name.clone(),
                        Function::Standard {
                            params: params.clone(),
                            body: body.clone(),
                            name: name.clone(),
                            token: token.clone(),
                            this: None,
                            closure: Rc::clone(&interpreter.env),
                        },
                    );
                }
                _ => (),
            }
        }

        Ok(Class {
            name: name.clone(),
            properties,
            methods,
            superclass,
        })
    }

    pub fn to_string(&self) -> String {
        self.name.clone()
    }

    pub fn find_method(&self, name: &String) -> Option<&Function> {
        self.methods.get(name).or_else(|| match &self.superclass {
            Some(sc) => sc.find_method(name).clone(),
            _ => None,
        })
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: &Vec<Value>) -> Result<Value, Error> {
        let instance = Rc::new(RefCell::new(Instance {
            class: self.clone(),
            properties: self.properties.clone(),
        }));

        if let Some(constructor) = self.methods.get("constructor") {
            constructor
                .clone()
                .bind(Rc::clone(&instance))
                .call(interpreter, arguments)?;
        }

        Ok(Value::Instance(instance))
    }
}

#[derive(Clone, Debug)]
pub struct Instance {
    class: Class,
    properties: Properties,
}

impl Instance {
    pub fn to_string(&self) -> String {
        format!("{} instance", self.class.name)
    }

    pub fn get_super(&self) -> Option<Class> {
        self.class.superclass.as_ref().map(|v| *v.clone())
    }

    pub fn get(&self, name: &String, token: &Token) -> Result<Value, Error> {
        self.properties.get(name).map_or_else(
            || {
                self.class.find_method(name).map_or_else(
                    || error(token, ErrorType::PropertyDoesntExist),
                    |fun| {
                        Ok(Value::Function(
                            fun.clone().bind(Rc::new(RefCell::new(self.clone()))),
                        ))
                    },
                )
            },
            |val| Ok(val.clone()),
        )
    }

    pub fn set(&mut self, name: &String, token: &Token, value: Value) {
        self.properties.insert(name.clone(), value);
    }
}
