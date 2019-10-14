#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;
mod ast;
mod errors;
mod interpreter;
mod lexer;
mod parser;
mod token;
mod utils;
use crate::ast::print_ast;
use crate::errors::{print_lexer_errors, print_parser_errors, LexerError, ParserError};
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{self, BufRead};
use std::{env, fs::File};
use crate::interpreter::visit_expr;

pub fn run_prompt() {
    loop {
        println!(" >");
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
            println!("{:#?}", &tokens);
            let mut parser = Parser::new(&tokens);
            let expr = parser.parse_tokens();
            match expr {
                Ok(expr) => {
                    println!("This program is valid!");
                    print_ast(expr.clone());
                    println!("Result : {}", visit_expr(expr));
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1);
    println!("test");
    //    match file_name {
    //        Some(file_name) => {
    //            println!("Opening file...");
    //            run_file(file_name);
    //        }
    //        _ => run_prompt(),
    //    }
}
