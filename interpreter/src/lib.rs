#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;
mod ast;
mod errors;
mod interpreter;
mod lexer;
mod parser;
mod runtime_value;
mod token;
mod utils;
use crate::ast::print_ast;
use crate::errors::{print_lexer_errors, print_parser_errors, LexerError, ParserError};
use crate::interpreter::interpret;
use crate::lexer::Lexer;
use crate::parser::Parser;
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

pub fn run_code(source_code: &str) {
    let mut lexer = Lexer::new(source_code);
    let tokens = lexer.scan_tokens();
    match tokens {
        Ok(tokens) => {
            let mut parser = Parser::new(&tokens);
            let expr = parser.parse_tokens();
            match expr {
                Ok(expr) => {
                    print_ast(expr.clone());
                    println!("Result : {:#?}", interpret(expr));
                }
                Err(e) => print_parser_errors(&e),
            }
        }
        Err(errors) => print_lexer_errors(&errors),
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
            run_code(&source_code);
        }
        _ => println!("This file doesn't exist!"),
    }
}
