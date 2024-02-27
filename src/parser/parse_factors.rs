use crate::{
    ast::{
        abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut, AST},
        function_call::FunctionCall,
        variable::Variable,
    },
    helpers::unexpected_token,
    lexer::{tokens::Operations, types::VarType},
    trace,
    types::ASTNode,
};

use std::{cell::RefCell, process::exit, rc::Rc};

use crate::{
    ast::factor::Factor,
    constants, helpers,
    lexer::tokens::{Bracket, Number, TokenEnum},
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /// FACTOR -> (*|&)* INTEGER | FLOAT | VARIABLE | STRING_LITERAL | LPAREN EXPRESSION RPAREN | FUNCTION_CALL
    pub fn parse_factor(&mut self) -> ASTNode {
        let next_token = self.peek_next_token();

        match &next_token.token {
            TokenEnum::Number(..) | TokenEnum::StringLiteral(..) => {
                self.get_next_token();
                return Rc::new(RefCell::new(Box::new(Factor::new(Box::new(next_token)))));
            }

            // This could also be a function call
            TokenEnum::Variable(var_name) => {
                // this is not a variable declaration, only a variable
                // name so we don't have type information here
                // This is handled via the call stack
                // This is done in the assignment_statemetn

                let var_token = self.get_next_token();

                let peeked_token = self.peek_next_token();

                match &peeked_token.token {
                    TokenEnum::Bracket(b) => match b {
                        Bracket::LParen => self.parse_function_call(var_name.into()),

                        // WE cannot check for other type of parenthesis here as
                        // write(variable) will result in error as there's a ')' following the
                        // variable, but it should be perfectly fine
                        _ => Rc::new(RefCell::new(Box::new(Variable::new(
                            Box::new(var_token),
                            VarType::Unknown,
                            var_name.into(),
                            false,
                            false,
                            0,
                        )))),
                    },

                    _ => Rc::new(RefCell::new(Box::new(Variable::new(
                        Box::new(var_token),
                        VarType::Unknown,
                        var_name.into(),
                        false,
                        false,
                        0,
                    )))),
                }
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
                                return Rc::new(RefCell::new(Box::new(Factor::new(Box::new(next_token)))));
                            }

                            Bracket::RParen => {
                                panic!(") never opened");
                            }

                            _ => {
                                panic!("Invalid token {:?}", next_token);
                            }
                        }
                    }

                    None => {
                        panic!(") never opened");
                    }
                },

                _ => {
                    panic!(
                        "Invalid token {:?}.\nInside func: {} \nInside Loop: {} \nInside If Else: {}\n",
                        next_token, self.inside_function_depth, self.inside_loop_depth, self.inside_if_else_depth
                    );
                }
            },

            TokenEnum::Op(op) => {
                if let Operations::Multiply = op {
                    self.get_next_token();

                    self.parsing_pointer_deref = true;
                    self.times_dereferenced = 1;

                    while let TokenEnum::Op(Operations::Multiply) = self.peek_next_token().token {
                        self.times_dereferenced += 1;
                        self.get_next_token();
                    }

                    let mut exp = self.parse_expression();

                    match exp.borrow_mut().get_node_mut() {
                        ASTNodeEnumMut::Variable(ref mut var) => {
                            var.dereference = true;
                            var.times_dereferenced = self.times_dereferenced;
                        }

                        _ => {}
                    };

                    self.parsing_pointer_deref = false;
                    self.times_dereferenced = 0;

                    return exp;
                }

                helpers::unexpected_token(&next_token, None);

                exit(1);
            }

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
                            VarType::Unknown,
                            var_name,
                            false,
                            true,
                            0,
                        ))))
                    }

                    _ => {
                        helpers::unexpected_token(&next_next_token, Some(&TokenEnum::Variable("".into())));
                        exit(1);
                    }
                }
            }

            _ => {
                helpers::unexpected_token(&next_token, None);
                exit(1);
            }
        }
    }
}
