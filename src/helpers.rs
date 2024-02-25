use std::{fmt::Display, process::exit};

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

pub fn unexpected_keyword<S: AsRef<str> + Display>(token: &Token, unexpected: S, expected: Option<S>) {
    println!(
        "{}:{}:{} Unexpected Keyword: '{}'{}",
        token.file,
        token.line_number,
        token.col_number,
        unexpected,
        match expected {
            Some(tok) => format!(" Expected: '{tok}'"),
            None => "".into(),
        }
    );

    exit(1);
}

pub fn unexpected_token(unexpected: &Token, expected: Option<&TokenEnum>) {
    println!(
        "{}:{}:{} Unexpected Token: '{}'{}",
        unexpected.file,
        unexpected.line_number,
        unexpected.col_number,
        unexpected,
        match expected {
            Some(tok) => format!(" Expected: '{tok}'"),
            None => "".into(),
        }
    );

    exit(1);
}

pub fn compiler_error<S: AsRef<str> + Display>(message: S, tok: &Token) {
    println!("{}:{}:{} {}", tok.file, tok.line_number, tok.col_number, message);
    exit(1);
}
