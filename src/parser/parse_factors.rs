use std::{process::exit, rc::Rc};

use crate::{
    ast::{abstract_syntax_tree::AST, factor::Factor},
    constants, helpers,
    lexer::tokens::{Bracket, Number, TokenEnum},
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /// FACTOR -> INTEGER | FLOAT | VARIABLE | STRING_LITERAL | LPAREN EXPRESSION RPAREN
    pub fn parse_factor(&mut self) -> Rc<Box<dyn AST>> {
        let next_token = self.peek_next_token();

        if constants::PARSER_DEBUG {
            println!("parse_factor next_token {:#?}", next_token);
        }

        match &next_token.token {
            TokenEnum::Number(..) | TokenEnum::Variable(..) | TokenEnum::StringLiteral(..) => {
                // println!("Inside the Number Variable thing {:?}", &next_token.token);

                self.get_next_token();
                return Rc::new(Box::new(Factor::new(Box::new(next_token))));
            }

            TokenEnum::Bracket(paren) => match paren {
                Bracket::LParen => {
                    self.get_next_token();
                    let return_value = self.parse_logical_expression();

                    let next_next_token = self.peek_next_token();

                    match &next_next_token.token {
                        TokenEnum::Bracket(b) => match b {
                            Bracket::LParen => self.parse_logical_expression(),

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
                                return Rc::new(Box::new(Factor::new(Box::new(next_token))));
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
                    panic!("Invalid token {:?}.\nInside func: {} \nInside Loop: {} \nInside If Else: {}\n", next_token, self.inside_function_depth, self.inside_loop_depth, self.inside_if_else_depth);
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
