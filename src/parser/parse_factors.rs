use std::process::exit;

use crate::{
    ast::{abstract_syntax_tree::AST, factor::Factor},
    constants, helpers,
    lexer::tokens::{Bracket, Number, TokenEnum},
};

use super::parser::Parser;

impl<'a> Parser<'a> {
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
                    let return_value = self.parse_comparison_expression();

                    let next_next_token = self.peek_next_token();

                    match &next_next_token.token {
                        TokenEnum::Bracket(b) => match b {
                            Bracket::LParen => self.parse_comparison_expression(),

                            Bracket::RParen => {
                                self.get_next_token();
                                return return_value;
                            }

                            _ => {
                                panic!("Invalid token {:?}", &next_next_token);
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

                            _ => {
                                panic!("Invalid token {:?}", next_token);
                            }
                        }
                    }

                    None => {
                        println!(") never opened");
                        exit(1);
                    }
                },

                _ => {
                    panic!("Invalid token {:?}", next_token);
                }
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
}
