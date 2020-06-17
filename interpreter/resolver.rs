use crate::error::{Error, ErrorType};
use crate::expr::{Expr, Visitor as ExprVisitor};
use crate::interpreter::Interpreter;
use crate::statement::{Stmt, Visitor as StmtVisitor};
use crate::token::{Literal, Token};
use log::debug;
use std::collections::{HashMap, LinkedList};

/// Distance to the variable from the scope it is referenced in
#[derive(Clone, Debug)]
pub struct VarRef {
    token: Token,
    pub name: String,
}

impl VarRef {
    pub fn new(token: &Token, name: &String) -> Self {
        VarRef {
            token: token.clone(),
            name: name.clone(),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}-{}-{}-{}",
            self.token.line, self.token.start, self.token.end, self.name
        )
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum ClassType {
    Subclass,
    Class,
}

#[derive(Debug)]
pub struct ResolverState {
    pub current_class: Option<ClassType>,
    pub inside_loop: bool,
}

impl ResolverState {
    pub fn new() -> Self {
        ResolverState {
            current_class: None,
            inside_loop: false,
        }
    }

    pub fn is_inside_loop(&self, token: &Token) -> ResolverResult {
        if !self.inside_loop {
            return error(token, ErrorType::NotAllowedOutsideLoop);
        }
        Ok(())
    }
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: LinkedList<HashMap<String, bool>>,
    pub state: ResolverState,
}

type ResolverResult = Result<(), Error>;

fn error(token: &Token, error_type: ErrorType) -> ResolverResult {
    Err(Error {
        token: token.clone(),
        error_type,
    })
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        let mut scopes = LinkedList::new();
        // add the top "global" like scope
        scopes.push_back(HashMap::new());
        Resolver {
            interpreter,
            scopes,
            state: ResolverState::new(),
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> ResolverResult {
        stmt.accept(self)
    }

    pub fn resolve_stmts(&mut self, stmts: &Vec<Stmt>) -> Result<(), Vec<Error>> {
        let mut errors: Vec<Error> = vec![];
        for stmt in stmts {
            match self.resolve_stmt(stmt) {
                Err(e) => errors.push(e),
                _ => (),
            }
        }
        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors),
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) -> ResolverResult {
        expr.accept(self)
    }

    fn resolve_distance(&mut self, distance: VarRef) {
        for (depth, scope) in self.scopes.iter().rev().enumerate() {
            if let Some(_) = scope.get(&distance.name) {
                self.interpreter.resolve_distance(distance.clone(), depth);
                return;
            }
        }
    }

    fn resolve_function(
        &mut self,
        params: &Vec<String>,
        body: &Vec<Stmt>,
    ) -> Result<(), Vec<Error>> {
        self.begin_scope();
        for param in params {
            self.declare(param);
            self.define(param);
        }
        self.resolve_stmts(body)?;
        self.end_scope();
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push_back(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop_back();
    }

    fn declare(&mut self, name: &String) {
        let scope = self.scopes.back_mut();
        match scope {
            Some(s) => {
                if s.contains_key(name) {
                } else {
                    s.insert(name.clone(), false);
                }
            }
            None => (),
        }
    }

    fn define(&mut self, name: &String) {
        let scope = self.scopes.back_mut();
        match scope {
            Some(s) => {
                s.insert(name.clone(), true);
            }
            None => (),
        }
    }
}

impl<'a> ExprVisitor<()> for Resolver<'a> {
    fn visit_binary(&mut self, left: &Expr, _operator: &Token, right: &Expr) -> ResolverResult {
        self.resolve_expr(left)?;
        self.resolve_expr(right)?;
        Ok(())
    }

    fn visit_literal(&mut self, _literal: &Literal) -> ResolverResult {
        Ok(())
    }
    fn visit_unary(&mut self, _operator: &Token, expr: &Expr) -> ResolverResult {
        self.resolve_expr(expr)?;
        Ok(())
    }

    fn visit_grouping(&mut self, expr: &Expr) -> ResolverResult {
        self.resolve_expr(expr)?;
        Ok(())
    }

    fn visit_var(&mut self, name: &String, token: &Token) -> ResolverResult {
        if let Some(s) = self.scopes.back() {
            if let Some(is_ready) = s.get(name) {
                if !is_ready {
                    return Err(Error {
                        token: token.clone(),
                        error_type: ErrorType::CantUseVariableInItsInitializer,
                    });
                }
            }
        }
        self.resolve_distance(VarRef::new(token, name));
        Ok(())
    }

    fn visit_assignment(&mut self, name: &String, expr: &Expr, token: &Token) -> ResolverResult {
        self.resolve_expr(expr)?;
        self.resolve_distance(VarRef::new(token, name));
        Ok(())
    }

    fn visit_logical(&mut self, left: &Expr, _operator: &Token, right: &Expr) -> ResolverResult {
        self.resolve_expr(left)?;
        self.resolve_expr(right)?;
        Ok(())
    }

    fn visit_call(
        &mut self,
        callee: &Expr,
        _token: &Token,
        arguments: &Vec<Expr>,
    ) -> ResolverResult {
        self.resolve_expr(callee)?;

        for arg in arguments {
            self.resolve_expr(arg)?;
        }

        Ok(())
    }

    fn visit_closure(
        &mut self,
        params: &Vec<String>,
        body: &Vec<Stmt>,
        _name: &String,
        _token: &Token,
    ) -> ResolverResult {
        self.resolve_function(params, body)?;
        Ok(())
    }

    fn visit_get(&mut self, _name: &String, _token: &Token, expr: &Expr) -> ResolverResult {
        self.resolve_expr(expr)?;
        Ok(())
    }

    fn visit_set(
        &mut self,
        _token: &Token,
        _name: &String,
        value: &Expr,
        obj: &Expr,
    ) -> ResolverResult {
        self.resolve_expr(value)?;
        self.resolve_expr(obj)?;
        Ok(())
    }

    fn visit_this(&mut self, token: &Token) -> ResolverResult {
        if self.state.current_class.is_none() {
            return error(token, ErrorType::CantUseThis);
        }
        self.resolve_distance(VarRef::new(token, &String::from("this")));
        Ok(())
    }

    fn visit_super(&mut self, token: &Token, _method_name: &String) -> ResolverResult {
        if let Some(ClassType::Subclass) = self.state.current_class {
            debug!("Can use super");
            self.resolve_distance(VarRef::new(token, &String::from("super")));
            Ok(())
        } else {
            debug!("Cant use super {:?}", self.state.current_class);
            error(token, ErrorType::CantUseSuper)
        }
    }
}

impl<'a> StmtVisitor<()> for Resolver<'a> {
    fn visit_print_stmt(&mut self, expr: &Expr) -> ResolverResult {
        self.resolve_expr(expr)?;
        Ok(())
    }

    fn visit_expr_stmt(&mut self, expr: &Expr) -> ResolverResult {
        self.resolve_expr(expr)?;
        Ok(())
    }

    fn visit_var(&mut self, name: &String, expr: &Option<Expr>) -> ResolverResult {
        self.declare(name);
        match expr {
            Some(e) => self.resolve_expr(e)?,
            None => (),
        };
        self.define(name);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> ResolverResult {
        self.begin_scope();
        self.resolve_stmts(stmts)?;
        self.end_scope();
        Ok(())
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_body: &Stmt,
        else_body: &Option<Box<Stmt>>,
    ) -> ResolverResult {
        self.resolve_expr(condition)?;
        self.resolve_stmt(then_body)?;
        if let Some(stmt) = else_body {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> ResolverResult {
        self.state.inside_loop = true;
        self.resolve_expr(condition)?;
        self.resolve_stmt(body)?;
        self.state.inside_loop = false;
        Ok(())
    }

    fn visit_break_stmt(&mut self, token: &Token) -> ResolverResult {
        self.state.is_inside_loop(token)?;
        Ok(())
    }

    fn visit_continue_stmt(&mut self, token: &Token) -> ResolverResult {
        self.state.is_inside_loop(token)?;
        Ok(())
    }
    fn visit_function_stmt(
        &mut self,
        name: &String,
        params: &Vec<String>,
        body: &Vec<Stmt>,
        _token: &Token,
    ) -> Result<(), Error> {
        self.declare(name);
        self.define(name);
        self.resolve_function(params, body)?;
        Ok(())
    }
    fn visit_class_stmt(
        &mut self,
        name: &String,
        _token: &Token,
        members: &Vec<Stmt>,
        superclass: &Option<Expr>,
    ) -> ResolverResult {
        self.declare(name);
        self.define(name);

        if let Some(sc) = superclass {
            self.state.current_class = Some(ClassType::Subclass);
            let (sc_name, sc_token) = sc.as_var().expect("Expected Expr::Var");
            if sc_name == name {
                return error(sc_token, ErrorType::CantInheritFromItself);
            }
            self.resolve_expr(sc)?;
            self.scopes
                .back_mut()
                .unwrap()
                .insert("super".to_owned(), true);
        } else {
            self.state.current_class = Some(ClassType::Class);
        }

        self.begin_scope();

        self.scopes
            .back_mut()
            .unwrap()
            .insert("this".to_owned(), true);

        for stmt in members {
            if let Some((params, body, ..)) = stmt.as_function() {
                self.resolve_function(params, body)?;
            }
        }
        self.end_scope();
        self.state.current_class = None;
        Ok(())
    }
    fn visit_return_stmt(&mut self, value: &Option<Expr>, _token: &Token) -> ResolverResult {
        if let Some(val) = value {
            self.resolve_expr(val)?;
        }
        Ok(())
    }
}
