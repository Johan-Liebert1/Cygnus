#![allow(dead_code)]

use parser::Parser;

use crate::ast::abstract_syntax_tree::AST;

mod parser;
mod tokens;
mod lexer;
mod  helpers;
mod ast;

fn main() {
    let file = std::fs::read("test/first.txt").unwrap();

    let mut parser = Parser::new(&file);

    let mut ast = parser.parse_statements();

    ast.visit();
}
