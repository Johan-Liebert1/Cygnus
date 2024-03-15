use crate::{
    helpers::unexpected_token,
    lexer::{
        tokens::{Bracket, Number},
        types::VarType,
    },
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
    /// VARIABLE_DECLARATION -> def VAR_NAME: (*)* VAR_TYPE
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
                                let mut actual_var_type = var_type.clone();

                                let type_token = self.get_next_token();

                                if let TokenEnum::Bracket(Bracket::LSquare) = self.peek_next_token().token {
                                    // is of type int[4]
                                    self.get_next_token();

                                    let peeked_token = self.peek_next_token();

                                    if let TokenEnum::Number(Number::Integer(int)) = peeked_token.token {
                                        let size = self.validate_token(TokenEnum::Number(Number::Integer(0)));
                                        self.validate_token(TokenEnum::Bracket(Bracket::RSquare));

                                        actual_var_type = VarType::Array(Box::new(var_type.clone()), int as usize);
                                    } else {
                                        unexpected_token(&peeked_token, Some(&TokenEnum::Number(Number::Integer(0))));
                                        exit(1);
                                    }
                                }

                                return Variable::new(
                                    Box::new(type_token),
                                    actual_var_type.clone(),
                                    var_name,
                                    false,
                                    false,
                                    0,
                                );
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

    /// VARIABLE_DECLARATION -> def VAR_NAME: (*)* VAR_TYPE (= ASSIGNED_STATEMENT)*
    pub fn parse_declaration_statement(&mut self) -> ASTNode {
        // we get here after consuming 'def'

        let left = self.parse_variable();

        self.validate_token(TokenEnum::Equals);

        // TODO: handle function calls and strings and stuff here
        let right: ASTNode;

        if let TokenEnum::Bracket(Bracket::LCurly) = self.peek_next_token().token {
            trace!("gonna parsing struct");
            right = self.parse_struct();
        } else {
            right = self.parse_logical_expression();
        }

        return Rc::new(RefCell::new(Box::new(DeclarationStatement::new(left, right))));
    }
}
