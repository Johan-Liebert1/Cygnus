use std::process::exit;

use crate::{
    helpers::{self, print_only_tokens},
    lexer::{Lexer, Token},
    tokens::{Number, TokenEnum, Operations}, ast::{binary_op::BinaryOP},
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
                TokenEnum::EOF => break,
                _ => {
                    self.parsed_tokens.push(token);
                }
            }
        }

        print_only_tokens(&self.parsed_tokens);
    }

    pub fn validate_token(&self, _token: TokenEnum, _expected_token: TokenEnum) {}

    pub fn parse_operand(&mut self) -> Token {
        let token = self.parser.get_next_token(false);

        match &token.token {
            TokenEnum::Number(..) => {
                return token;
            }

            _ => {
                helpers::unexpected_token(&token.token, &TokenEnum::Number(Number::Integer(1)));
                exit(1);
            }
        }
    }

    pub fn parse_operator(&mut self) -> Token {
        let token = self.parser.get_next_token(false);

        match &token.token {
            TokenEnum::Op(..) => {
                return token;
            }

            _ => {
                helpers::unexpected_token(&token.token, &TokenEnum::Op(Operations::Plus));
                println!("Line: {}, Column: {}", self.parser.line_number, self.parser.col_number);
                exit(1);
            }
        }
    }

    /// BINARY_OPRATION -> NUMBER (+|*|/|-) NUMBER
    pub fn parse_binary_op(&mut self) -> BinaryOP {
        let left_operand = Box::new(self.parse_operand());
        let operator = Box::new(self.parse_operator());
        let right_operand = Box::new(self.parse_operand());

        return BinaryOP::new(left_operand, operator, right_operand);
    }

    /// STATEMENT -> BINARY_OPRATION
    pub fn parse_statements(&mut self) -> BinaryOP {
        return self.parse_binary_op();
    }

    pub fn parse(&mut self) -> BinaryOP {
        return self.parse_statements();
    }
}
