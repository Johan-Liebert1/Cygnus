#![allow(dead_code)]

use parser::parser::Parser;

use crate::interpreter::interpreter::Interpreter;

mod ast;
mod constants;
mod helpers;
mod lexer;
mod parser;
mod tests;
mod interpreter;

fn main() {
    let file = std::fs::read("test/first.txt").unwrap();

    let mut parser = Parser::new(&file);
    let ast = parser.parse_statements();

    let interpreter = Interpreter::new(ast);
    let result = interpreter.interpret();

    println!("\n\nRESULT\n{:#?}", result);
    // println!("AST\n{:#?}", ast);
}
