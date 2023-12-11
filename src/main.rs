#![allow(dead_code)]

use parser::parser::Parser;

use crate::interpreter::interpreter::Interpreter;

mod asm;
mod ast;
mod constants;
mod helpers;
mod interpreter;
mod lexer;
mod parser;
mod tests;

fn main() {
    let file = std::fs::read("test/first.txt").unwrap();

    let mut parser = Parser::new(&file);
    let ast = parser.parse_program();

    let mut interpreter = Interpreter::new(ast, parser.functions);
    let _result = interpreter.compile();

    println!("{:#?}", interpreter.asm);

    // println!("\n\nRESULT\n{:#?}", result);
    // println!("{:#?}", interpreter.variables)
}
