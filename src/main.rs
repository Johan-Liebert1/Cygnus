use lexer::Lexer;

mod parser;
mod tokens;
mod lexer;
mod  helpers;
mod ast;

fn main() {
    let file = std::fs::read("test/first.txt").unwrap();

    let mut lexer = Lexer::new(&file);

    lexer.start();
}
