#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;

mod ast;
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
use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::resolver::Resolver;
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
        run_code(&code);
    }
}

pub fn run_code(source_code: &str) -> Result<(), Error> {
    let mut lexer = Lexer::new(source_code);
    match lexer.scan_tokens() {
        Ok(tokens) => {
            let mut parser = Parser::new(&tokens);
            let stmts = parser.parse_tokens()?;
            let mut interpreter = Interpreter::new();
            let mut resolver = Resolver::new(&mut interpreter);
            resolver.resolve_stmts(&stmts);
            interpreter.interpret(&stmts)?;
            Ok(())
        }
        Err(e) => Err(e[0].clone()),
    }
}

pub fn run_file(path: &str) {
    match File::open(path) {
        Ok(f) => {
            let mut source_code = String::new();
            let mut buf_reader = BufReader::new(f);
            buf_reader
                .read_to_string(&mut source_code)
                .expect("This file is empty!");
            println!("{:#?}", run_code(&source_code));
        }
        _ => println!("This file doesn't exist!"),
    }
}
