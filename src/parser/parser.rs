use std::process::exit;

use crate::{
    ast::{abstract_syntax_tree::AST, binary_op::BinaryOP, factor::Factor},
    helpers::{self},
    lexer::{Lexer, Token},
    tokens::{Bracket, Number, Operations, TokenEnum},
};

pub struct Parser<'a> {
    lexer: Box<Lexer<'a>>,
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
        let next_token = self.lexer.peek_next_token();

        // println!("parse_factor next_token {:#?}", next_token);

        match &next_token.token {
            TokenEnum::Number(..) => {
                self.lexer.get_next_token();
                return Box::new(Factor::new(Box::new(next_token)));
            }

            TokenEnum::Bracket(paren) => match paren {
                Bracket::LParen => {
                    self.lexer.get_next_token();
                    let return_value = self.parse_expression();
                    // need a right parenthesis here
                    return return_value;
                },

                Bracket::RParen => match self.bracket_stack.last() {
                    Some(bracket) => {
                        match bracket {
                            Bracket::LParen => {
                                // all good. A left paren was closed
                                todo!();
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
        let left = self.parse_factor();

        let next_token = self.lexer.peek_next_token();

        // println!("parse_term next_token {:#?}", next_token);

        match &next_token.token {
            TokenEnum::Op(op) => match op {
                Operations::Divide | Operations::Multiply => {
                    let token = self.lexer.get_next_token();

                    return Box::new(BinaryOP::new(
                        left,
                        Box::new(token),
                        self.parse_factor(),
                    ));
                }

                _ => {
                    return left;
                }
            },

            _ => {
                return left;
            }
        }
    }

    /// EXPRESSION -> BINARY_OP (+|-) BINARY_OP
    /// for precedence as term will be calculated first
    pub fn parse_expression(&mut self) -> Box<dyn AST> {
        let left_operand = self.parse_term();

        let next_token = self.lexer.peek_next_token();

        // println!("parse_expression, left_operand {:?}", left_operand);
        // println!("parse_expression, next_token {:#?}", next_token);

        match &next_token.token {
            TokenEnum::Op(op) => match op {
                Operations::Plus | Operations::Minus => {
                    self.lexer.get_next_token();

                    return Box::new(BinaryOP::new(
                        left_operand,
                        Box::new(next_token),
                        self.parse_term(),
                    ));
                }

                _ => {}
            },

            _ => {}
        };

        return left_operand;
    }

    /// STATEMENT -> EXPRESSION
    pub fn parse_statements(&mut self) -> Box<dyn AST> {
        return self.parse_expression();
    }

    pub fn parse(&mut self) -> Box<dyn AST> {
        return self.parse_statements();
    }
}
