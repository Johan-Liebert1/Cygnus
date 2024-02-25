use std::process::exit;

use crate::lexer::{lexer::Token, tokens::TokenEnum};

#[macro_export]
macro_rules! trace {
    // The pattern for a single argument
    ($($arg:tt)*) => {
        println!("{}:{}: {}", file!(), line!(), format_args!($($arg)*));
    };
}

pub fn print_only_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        print!("{:?}, ", token.token);
    }

    trace!("");
}

pub fn unexpected_token(unexpected: &Token, expected: Option<&TokenEnum>) {
    println!(
        "{}:{}:{} Unexpected Token: '{}' {}",
        unexpected.file,
        unexpected.line_number,
        unexpected.col_number,
        unexpected,
        match expected {
            Some(tok) => " Expected: {tok}",
            None => "",
        }
    );
}
