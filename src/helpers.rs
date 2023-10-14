use std::process::exit;

use crate::{lexer::Token, tokens::TokenEnum};

pub fn print_only_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        print!("{:?}, ", token.token);
    }

    println!("");
}

pub fn unexpected_token(unexpected: &TokenEnum, expected: &TokenEnum) {
    print!(
        "Unexpected Token: {:?}. Expected: {:?} ",
        unexpected, expected
    );
}
