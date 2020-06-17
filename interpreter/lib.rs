#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate enum_as_inner;
mod class;
mod environment;
pub mod error;
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
use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::resolver::Resolver;
use crate::utils::print_errors;
use log::{debug, Level};
use std::fs::read_to_string;
use std::io::{self, BufRead};

pub fn run_prompt() {
    loop {
        println!(">");
        let mut code = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut code).unwrap();
    }
}

#[wasm_bindgen]
pub fn execute(source_code: &str) {
    console_log::init_with_level(Level::Debug);

    match run_code(source_code) {
        Ok(_) => (),
        Err(errors) => {
            debug!("{:#?}", errors);
            print_errors(&errors)
        }
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
    if let Err(e) = interpreter.interpret(&stmts) {
        return Err(vec![e]);
    }
    Ok(())
}

pub fn run_file(path: &str) {
    let source_code = read_to_string(path).expect("This file doesn't exist");
    execute(&source_code);
}

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
