use parser::Parser;

mod parser;
mod tokens;
mod lexer;
mod  helpers;
mod ast;

fn main() {
    let file = std::fs::read("test/first.txt").unwrap();

    let mut parser = Parser::new(&file);

    parser.start();
}
