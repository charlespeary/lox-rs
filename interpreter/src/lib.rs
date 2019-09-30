#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;
mod ast;
mod errors;
mod parser;
mod scanner;
mod token;
mod utils;
mod lexer;
use crate::ast::print_ast;
use crate::parser::Parser;
use scanner::Scanner;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{self, BufRead};
use std::{env, fs::File};
use crate::lexer::Lexer;

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
    lexer.scan_tokens();
    let mut scanner = Scanner::new(source_code);
    scanner.scan_tokens();
    println!("Tokens: {:#?}", scanner.tokens);
    let mut parser = Parser::new(&scanner.tokens);
    let ast = parser.parse_tokens();
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
