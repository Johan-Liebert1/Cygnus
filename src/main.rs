#![allow(dead_code)]

use parser::parser::Parser;

mod ast;
mod helpers;
mod lexer;
mod parser;
mod tokens;

fn main() {
    let file = std::fs::read("test/first.txt").unwrap();

    let mut parser = Parser::new(&file);

    let mut ast = parser.parse_statements();

    // println!("AST: {:#?}", ast);

    ast.visit();
}
