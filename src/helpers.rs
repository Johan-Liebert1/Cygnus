use crate::parser::Token;


pub fn print_only_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        print!("{:?}, ", token.token);
    }

    println!("");
}

