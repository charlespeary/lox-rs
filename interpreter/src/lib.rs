#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate enum_as_inner;
mod ast;
mod class;
mod environment;
mod error;
mod expr;
mod function;
mod interpreter;
mod lexer;
mod parser;
mod resolver;
mod runtime_value;
mod statement;
mod token;
mod utils;
use crate::ast::print_ast;
use crate::error::{print_errors, Error};
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::resolver::Resolver;
use std::fs::read_to_string;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{self, BufRead};
use std::{env, fs::File};

pub fn run_prompt() {
    loop {
        println!(">");
        let mut code = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut code).unwrap();
    }
}

pub fn execute(source_code: &str) {
    match run_code(source_code) {
        Ok(_) => (),
        Err(errors) => print_errors(&errors),
    }
}

pub fn run_code(source_code: &str) -> Result<(), Vec<Error>> {
    let mut lexer = Lexer::new(source_code);
    let tokens = lexer.scan_tokens()?;
    let mut parser = Parser::new(&tokens);
    let stmts = parser.parse_tokens()?;
    let mut interpreter = Interpreter::new();
    let mut resolver = Resolver::new(&mut interpreter);
    resolver.resolve_stmts(&stmts)?;
    println!("{:#?}", interpreter.distances);
    interpreter.interpret(&stmts);
    Ok(())
}

pub fn run_file(path: &str) {
    let mut source_code = read_to_string(path).expect("This file doesn't exist");
    execute(&source_code);
}
