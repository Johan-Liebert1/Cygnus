

use crate::lexer::{lexer::Token, tokens::TokenEnum};

pub fn print_only_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        print!("{:?}, ", token.token);
    }

    println!("");
}

pub fn unexpected_token(message: &'static str, unexpected: &TokenEnum, expected: &TokenEnum) {
    print!(
        "{}. Unexpected Token: {:?}. Expected: {:?} ",
        message, unexpected, expected
    );
}
