use std::process::exit;

use crate::{
    ast::{abstract_syntax_tree::AST, binary_op::BinaryOP, factor::Factor},
    constants,
    helpers::{self},
    lexer::{Lexer, Token},
    tokens::{Bracket, Number, Operations, TokenEnum},
};

pub struct Parser<'a> {
    pub lexer: Box<Lexer<'a>>,
    parsed_tokens: Vec<Token>,
    bracket_stack: Vec<Bracket>,
}

impl<'a> Parser<'a> {
    pub fn new(file: &'a Vec<u8>) -> Self {
        let parser = Lexer::new(file);

        Self {
            lexer: Box::new(parser),
            parsed_tokens: vec![],
            bracket_stack: vec![],
        }
    }

    pub fn validate_token(&self, _token: TokenEnum, _expected_token: TokenEnum) {}

    /// FACTOR -> INTEGER | FLOAT
    pub fn parse_factor(&mut self) -> Box<dyn AST> {
        let next_token = self.peek_next_token();

        if constants::PARSER_DEBUG {
            println!("parse_factor next_token {:#?}", next_token);
        }

        match &next_token.token {
            TokenEnum::Number(..) => {
                self.get_next_token();
                return Box::new(Factor::new(Box::new(next_token)));
            }

            TokenEnum::Bracket(paren) => match paren {
                Bracket::LParen => {
                    self.get_next_token();
                    let return_value = self.parse_expression();

                    match self.peek_next_token().token {
                        TokenEnum::Bracket(b) => match b {
                            Bracket::LParen => self.parse_expression(),

                            Bracket::RParen => {
                                self.get_next_token();
                                return return_value;
                            }
                        },

                        _ => {
                            panic!("Unclosed (");
                        }
                    };

                    return return_value;
                }

                Bracket::RParen => match self.bracket_stack.last() {
                    Some(bracket) => {
                        match bracket {
                            Bracket::LParen => {
                                // all good. A left paren was closed
                                self.get_next_token();
                                return Box::new(Factor::new(Box::new(next_token)));
                            }

                            Bracket::RParen => {
                                println!(") never opened");
                                exit(1);
                            }
                        }
                    }

                    None => {
                        println!(") never opened");
                        exit(1);
                    }
                },
            },

            _ => {
                helpers::unexpected_token(
                    "parse_factor",
                    &next_token.token,
                    &TokenEnum::Number(Number::Integer(1)),
                );

                exit(1);
            }
        }
    }

    /// TERM -> FACTOR (*|/) FACTOR
    pub fn parse_term(&mut self) -> Box<dyn AST> {
        let mut result = self.parse_factor();

        loop {
            let next_token = self.peek_next_token();

            if constants::PARSER_DEBUG {
                println!("parse_term next_token {:#?}", next_token);
            }

            match &next_token.token {
                TokenEnum::Op(op) => match op {
                    Operations::Divide | Operations::Multiply => {
                        let token = self.get_next_token();

                        // reassign the result 
                        // if we have 1*2*3
                        // in the first iteration, result is (left: 1, op: *, right: 2)
                        // in the next iteration, result is 
                        // [left: (left: 1, op: *, right: 2), op: *, right: 3]
                        // and so on
                        result = Box::new(BinaryOP::new(
                            result,
                            Box::new(token),
                            self.parse_factor(),
                        ));
                    }

                    _ => {
                        return result;
                    } 
                },

                _ => {
                    return result;
                }
            }
        }
    }

    /// EXPRESSION -> BINARY_OP (+|-) BINARY_OP
    /// for precedence as term will be calculated first
    pub fn parse_expression(&mut self) -> Box<dyn AST> {
        let mut result = self.parse_term();

        // to handle multiple
        loop {
            let next_token = self.peek_next_token();

            match &next_token.token {
                TokenEnum::Op(op) => match op {
                    Operations::Plus | Operations::Minus => {
                        self.get_next_token();

                        // reassign the result 
                        // if we have 1+2+3
                        // in the first iteration, result is (left: 1, op: +, right: 2)
                        // in the next iteration, result is 
                        // [left: (left: 1, op: +, right: 2), op: +, right: 3]
                        // and so on
                        result = Box::new(BinaryOP::new(
                            result,
                            Box::new(next_token),
                            self.parse_term(),
                        ));
                    }

                    _ => {
                        return result;
                    }
                },

                _ => {
                    return result;
                }
            };
        }
    }

    /// STATEMENT -> EXPRESSION
    pub fn parse_statements(&mut self) -> Box<dyn AST> {
        return self.parse_expression();
    }

    pub fn parse(&mut self) -> Box<dyn AST> {
        return self.parse_statements();
    }
}
