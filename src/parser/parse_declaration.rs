use crate::{
    helpers::unexpected_token,
    lexer::{tokens::Bracket, types::VarType},
    trace,
    types::ASTNode,
};

use std::{cell::RefCell, process::exit, rc::Rc};

use crate::{
    ast::{declaration_statement::DeclarationStatement, variable::Variable},
    lexer::tokens::TokenEnum,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /// VARIABLE_DECLARATION -> def VAR_NAME: (*)* VAR_TYPE (= COMPARISON_EXPRESSION)*
    pub fn parse_variable(&mut self) -> Variable {
        let token = self.get_next_token();

        match token.token {
            TokenEnum::Variable(var_name) => {
                let token = self.get_next_token();

                match token.token {
                    // : after variable name, so can only be VAR_NAME: VAR_TYPE
                    TokenEnum::Colon => {
                        let token = self.peek_next_token();

                        match &token.token {
                            TokenEnum::Type(var_type) => {
                                let token = self.get_next_token();

                                return Variable::new(Box::new(token), var_type.clone(), var_name, false, false, 0);
                            }

                            _ => {
                                unexpected_token(&token, None);
                                exit(1);
                            }
                        }
                    }

                    _ => {
                        unexpected_token(&token, Some(&TokenEnum::Colon));
                        exit(1);
                    }
                }
            }

            _ => {
                unexpected_token(&token, Some(&TokenEnum::Colon));
                exit(1);
            }
        }
    }

    pub fn parse_declaration_statement(&mut self) -> ASTNode {
        // we get here after consuming 'def'

        let left = self.parse_variable();

        self.validate_token(TokenEnum::Equals);

        let right = self.parse_logical_expression();

        // TODO: handle function calls and strings and stuff here
        return Rc::new(RefCell::new(Box::new(DeclarationStatement::new(left, right))));
    }
}
