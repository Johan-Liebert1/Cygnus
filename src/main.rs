use parser::Token;
use tokens::TokenType;

use crate::parser::Parser;

mod parser;
mod tokens;

fn print_only_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        print!("{:?}, ", token.token);
    }

    println!("");
}

fn main() {
    let file = std::fs::read("test/first.txt").unwrap();

    let mut parsed_tokens = vec![];

    let mut parser = Parser::new(&file);

    loop {
        let token = parser.get_next_token();

        match token.token {
            TokenType::EOF => break,
            _ => parsed_tokens.push(token),
        }
    }

    print_only_tokens(&parsed_tokens);
}
