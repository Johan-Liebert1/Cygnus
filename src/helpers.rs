use crate::lexer::{lexer::Token, tokens::TokenEnum};

#[macro_export]
macro_rules! trace {
    // The pattern for a single argument
    ($($arg:tt)*) => {
        println!("{}:{}: {}", file!(), line!(), format_args!($($arg)*))
    };
}

pub fn print_only_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        print!("{:?}, ", token.token);
    }

    trace!("");
}

pub fn unexpected_token(message: &'static str, unexpected: &TokenEnum, expected: &TokenEnum) {
    print!(
        "{}. Unexpected Token: {:?}. Expected: {:?} ",
        message, unexpected, expected
    );
}
