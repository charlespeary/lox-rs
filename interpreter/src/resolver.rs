use crate::error::{error, Error, ErrorType};
use crate::expr::{Expr, Visitor as ExprVisitor};
use crate::interpreter::Interpreter;
use crate::runtime_value::Value;
use crate::statement::{Stmt, Visitor as StmtVisitor};
use crate::token::{Literal, Token};
use std::collections::{HashMap, LinkedList};
use std::hash::{Hash, Hasher};

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

enum Init {
    Declare,
    Define,
}

impl Init {
    fn is_ready(&self) -> bool {
        match self {
            Init::Declare => false,
            Init::Define => true,
        }
    }
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: LinkedList<HashMap<String, bool>>,
}

type ResolverResult = Result<(), Error>;

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        let mut scopes = LinkedList::new();
        // add the top "global" like scope
        scopes.push_back(HashMap::new());
        Resolver {
            interpreter,
            scopes,
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        stmt.accept(self);
    }

    pub fn resolve_stmts(&mut self, stmts: &Vec<Stmt>) {
        for stmt in stmts {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        expr.accept(self);
    }

    fn resolve_distance(&mut self, distance: VarRef) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if let Some(_) = scope.get(&distance.name) {
                let depth = if i == 0 { 0 } else { i - 1 };
                self.interpreter.resolve_distance(distance.clone(), depth);
            }
        }
    }

    fn resolve_function(&mut self, params: &Vec<String>, body: &Vec<Stmt>) {
        self.begin_scope();
        for param in params {
            self.declare(param);
            self.define(param);
        }
        self.resolve_stmts(body);
        self.end_scope();
    }

    fn begin_scope(&mut self) {
        self.scopes.push_back(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop_back();
    }

    fn declare(&mut self, name: &String) -> ResolverResult {
        let x = self.scopes.len().clone();
        let scope = self.scopes.back_mut();
        match scope {
            Some(s) => {
                if s.contains_key(name) {
                } else {
                    s.insert(name.clone(), false);
                }
            }
            None => return Ok(()),
        }
        Ok(())
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
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> ResolverResult {
        self.resolve_expr(left);
        self.resolve_expr(right);
        Ok(())
    }

    fn visit_literal(&mut self, literal: &Literal) -> ResolverResult {
        Ok(())
    }
    fn visit_unary(&mut self, operator: &Token, expr: &Expr) -> ResolverResult {
        self.resolve_expr(expr);
        Ok(())
    }

    fn visit_grouping(&mut self, expr: &Expr) -> ResolverResult {
        self.resolve_expr(expr);
        Ok(())
    }

    fn visit_var(&mut self, name: &String, token: &Token) -> ResolverResult {
        let scope = self.scopes.back();
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
        self.resolve_expr(expr);
        self.resolve_distance(VarRef::new(token, name));
        Ok(())
    }

    fn visit_logical(&mut self, left: &Expr, operator: &Token, right: &Expr) -> ResolverResult {
        self.resolve_expr(left);
        self.resolve_expr(right);
        Ok(())
    }
    fn visit_call(
        &mut self,
        callee: &Expr,
        token: &Token,
        arguments: &Vec<Expr>,
    ) -> ResolverResult {
        self.resolve_expr(callee);

        for arg in arguments {
            self.resolve_expr(arg);
        }

        Ok(())
    }

    fn visit_closure(
        &mut self,
        params: &Vec<String>,
        body: &Vec<Stmt>,
        name: &String,
        token: &Token,
    ) -> ResolverResult {
        self.resolve_function(params, body);
        Ok(())
    }
}

impl<'a> StmtVisitor<()> for Resolver<'a> {
    fn visit_print_stmt(&mut self, expr: &Expr) -> ResolverResult {
        self.resolve_expr(expr);
        Ok(())
    }

    fn visit_expr_stmt(&mut self, expr: &Expr) -> ResolverResult {
        self.resolve_expr(expr);
        Ok(())
    }

    fn visit_var(&mut self, name: &String, expr: &Option<Expr>) -> ResolverResult {
        self.declare(name);
        match expr {
            Some(e) => self.resolve_expr(e),
            None => (),
        };
        self.define(name);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> ResolverResult {
        self.begin_scope();
        self.resolve_stmts(stmts);
        self.end_scope();
        Ok(())
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_body: &Stmt,
        else_body: &Option<Box<Stmt>>,
    ) -> ResolverResult {
        self.resolve_expr(condition);
        self.resolve_stmt(then_body);
        if let Some(stmt) = else_body {
            self.resolve_stmt(stmt);
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> ResolverResult {
        self.resolve_expr(condition);
        self.resolve_stmt(body);
        Ok(())
    }

    fn visit_break_stmt(&mut self) -> ResolverResult {
        Ok(())
    }

    fn visit_continue_stmt(&mut self) -> ResolverResult {
        Ok(())
    }
    fn visit_function_stmt(
        &mut self,
        name: &String,
        params: &Vec<String>,
        body: &Vec<Stmt>,
        token: &Token,
    ) -> Result<(), Error> {
        self.declare(name);
        self.define(name);
        self.resolve_function(params, body);
        Ok(())
    }
    fn visit_return_stmt(&mut self, value: &Option<Expr>, token: &Token) -> ResolverResult {
        if let Some(val) = value {
            self.resolve_expr(val);
        }
        Ok(())
    }
}
