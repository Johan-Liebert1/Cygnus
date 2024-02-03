use crate::{ast::variable::Variable, lexer::tokens::Operations, trace, types::ASTNode};

use std::{cell::RefCell, process::exit, rc::Rc};

use crate::{
    ast::factor::Factor,
    constants, helpers,
    lexer::tokens::{Bracket, Number, TokenEnum},
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /// FACTOR -> (*|&) INTEGER | FLOAT | VARIABLE | STRING_LITERAL | LPAREN EXPRESSION RPAREN
    pub fn parse_factor(&mut self) -> ASTNode {
        let next_token = self.peek_next_token();

        if constants::PARSER_DEBUG {
            trace!("parse_factor next_token {:#?}", next_token);
        }

        match &next_token.token {
            TokenEnum::Number(..) | TokenEnum::StringLiteral(..) => {
                self.get_next_token();
                return Rc::new(RefCell::new(Box::new(Factor::new(Box::new(next_token)))));
            }

            TokenEnum::Variable(var_name) => {
                Rc::new(RefCell::new(Box::new(Variable::new(
                    Box::new(self.get_next_token()),
                    // this is not a variable declaration, only a variable
                    // name so we don't have type information here
                    // This is handled via the call stack
                    "".into(),
                    var_name.into(),
                    false,
                    false,
                ))))
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
                                return Rc::new(RefCell::new(Box::new(Factor::new(Box::new(
                                    next_token,
                                )))));
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

            TokenEnum::Op(op) => match op {
                // pointer deref
                Operations::Multiply => {
                    let get_next_token = self.get_next_token();

                    let next_next_token = self.peek_next_token();

                    // the next token has to be a variable, else this is a syntax error
                    match next_next_token.token {
                        TokenEnum::Variable(var_name) => {
                            Rc::new(RefCell::new(Box::new(Variable::new(
                                Box::new(self.get_next_token()),
                                // this is not a variable declaration, only a variable
                                // name so we don't have type information here
                                // This is handled via the call stack
                                "".into(),
                                var_name,
                                true,
                                false,
                            ))))
                        }

                        _ => {
                            helpers::unexpected_token(
                                "parse_factor",
                                &next_next_token.token,
                                &TokenEnum::Variable("".into()),
                            );

                            exit(1);
                        }
                    }
                }

                _ => {
                    helpers::unexpected_token(
                        "parse_factor",
                        &next_token.token,
                        &TokenEnum::Op(Operations::Multiply),
                    );

                    exit(1);
                }
            },

            TokenEnum::Ampersand => {
                // consume '&'
                let get_next_token = self.get_next_token();

                let next_next_token = self.peek_next_token();

                // the next token has to be a variable, else this is a syntax error
                match next_next_token.token {
                    TokenEnum::Variable(var_name) => {
                        Rc::new(RefCell::new(Box::new(Variable::new(
                            Box::new(self.get_next_token()),
                            // this is not a variable declaration, only a variable
                            // name so we don't have type information here
                            // This is handled via the call stack
                            "".into(),
                            var_name,
                            false,
                            true,
                        ))))
                    }

                    _ => {
                        helpers::unexpected_token(
                            "parse_factor",
                            &next_next_token.token,
                            &TokenEnum::Variable("".into()),
                        );

                        exit(1);
                    }
                }
            }

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
