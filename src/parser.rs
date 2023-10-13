use crate::{
    helpers::print_only_tokens,
    lexer::{Lexer, Token},
    tokens::TokenType,
};

pub struct Parser<'a> {
    parser: Box<Lexer<'a>>,
    parsed_tokens: Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(file: &'a Vec<u8>) -> Self {
        let parser = Lexer::new(file);

        Self {
            parser: Box::new(parser),
            parsed_tokens: vec![],
        }
    }

    pub fn start(&mut self) {
        loop {
            let token = self.parser.get_next_token(false);

            match &token.token {
                TokenType::EOF => break,

                TokenType::Integer(_) => {
                    let next_token = self.parser.get_next_token(true);

                    match &next_token.token {
                        TokenType::Op(_) => {
                            println!("nice got, {:?}", &next_token);
                        },

                        _ => {
                            println!("Expected an operand, got {:?}", &next_token);
                            return;
                        }
                    }
                }
                _ => {}
            }

            self.parsed_tokens.push(token);
        }

        print_only_tokens(&self.parsed_tokens);
    }
}
