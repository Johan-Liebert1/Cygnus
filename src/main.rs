#![allow(dead_code)]

use parser::parser::Parser;

mod ast;
mod constants;
mod helpers;
mod lexer;
mod parser;
mod tests;

fn main() {
    let file = std::fs::read("test/first.txt").unwrap();

    let mut parser = Parser::new(&file);

    let ast = parser.parse_statements();

    let result = ast.visit();

    println!("\n\nRESULT\n{:#?}", result);
    // println!("AST\n{:#?}", ast);
}
