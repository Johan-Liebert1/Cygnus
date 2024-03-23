use crate::{
    helpers::{compiler_error, unexpected_token},
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

impl Parser {
    fn check_if_array_type(&mut self, actual_var_type: &mut VarType, var_type: &VarType) {
        if let TokenEnum::Bracket(Bracket::LSquare) = self.peek_next_token().token {
            // is of type int[4]
            self.get_next_token();

            let peeked_token = self.peek_next_token();

            if let TokenEnum::Number(Number::Integer(int)) = peeked_token.token {
                let size = self.validate_token(TokenEnum::Number(Number::Integer(0)));
                self.validate_token(TokenEnum::Bracket(Bracket::RSquare));

                *actual_var_type = VarType::Array(Box::new(var_type.clone()), int as usize);
            } else {
                unexpected_token(&peeked_token, Some(&TokenEnum::Number(Number::Integer(0))));
                exit(1);
            }
        }
    }

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

                                self.check_if_array_type(&mut actual_var_type, var_type);

                                return Variable::new(
                                    Box::new(type_token),
                                    actual_var_type.clone(),
                                    var_name,
                                    false,
                                    false,
                                    0,
                                );
                            }

                            // This could be a user defined type
                            TokenEnum::Variable(var_type_name) => {
                                let var_name_clone = var_type_name.clone();

                                let found = self.user_defined_types.iter().find(|var| var.name == *var_name_clone);

                                if found.is_none() {
                                    compiler_error(format!("No such type '{}'", var_type_name), &token);
                                    exit(1);
                                }

                                let var_type = found.unwrap().type_.clone();

                                let type_token = self.get_next_token();

                                let mut actual_var_type = var_type.clone();

                                self.check_if_array_type(&mut actual_var_type, &var_type);

                                return Variable::new(
                                    Box::new(type_token),
                                    actual_var_type.clone(),
                                    var_name.into(),
                                    false,
                                    false,
                                    0,
                                );
                            }

                            tok => {
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
    pub fn parse_declaration_statement(&mut self, is_const: bool) -> ASTNode {
        // we get here after consuming 'def'

        let mut left = self.parse_variable();
        left.is_const = is_const;

        self.validate_token(TokenEnum::Equals);

        let peeked = self.peek_next_token();

        let mut parse_struct = false;

        if let TokenEnum::Variable(var_peeked) = &peeked.token {
            let next_token = self.peek_nth_token(2).token;

            // as we could also enter this if condition with the following assignment
            // def a: int = array[4];
            parse_struct = matches!(next_token, TokenEnum::Bracket(Bracket::LCurly));
        }

        let right = if parse_struct {
            self.parse_struct_decleration()
        } else {
            self.parse_logical_expression()
        };

        return Rc::new(RefCell::new(Box::new(DeclarationStatement::new(left, right))));
    }
}
