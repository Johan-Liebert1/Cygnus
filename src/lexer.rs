use crate::{
    helpers::print_only_tokens,
    parser::{Parser, Token},
    tokens::TokenType,
};

pub struct Lexer<'a> {
    parser: Box<Parser<'a>>,
    parsed_tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(file: &'a Vec<u8>) -> Self {
        let parser = Parser::new(file);

        Self {
            parser: Box::new(parser),
            parsed_tokens: vec![],
        }
    }

    pub fn start(&mut self) {
        loop {
            let token = self.parser.get_next_token();

            match &token.token {
                TokenType::EOF => break,
                _ => {
                },
            }

            self.parsed_tokens.push(token);
        }

        print_only_tokens(&self.parsed_tokens);
    }
}
