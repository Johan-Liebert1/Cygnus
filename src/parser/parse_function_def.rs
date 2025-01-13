use crate::{
    ast::void::Void, helpers::unexpected_token, interpreter::interpreter::FunctionHashMapValue, lexer::types::VarType,
    types::ASTNode,
};

use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{function_def::FunctionDefinition, variable::Variable},
    lexer::tokens::{Bracket, TokenEnum},
};

use super::parser::{Parser, ParserFunctions};

impl Parser {
    fn parse_function_definition_parameters(&mut self) -> Vec<Rc<RefCell<Variable>>> {
        let mut parameters = vec![];

        loop {
            let token = self.peek_next_token();

            match &token.token {
                TokenEnum::Bracket(b) => match b {
                    Bracket::RParen => {
                        self.consume_token();
                        break;
                    }

                    _ => {
                        unexpected_token(&token, None);
                    }
                },

                TokenEnum::Comma => {
                    self.consume_token();
                    continue;
                }

                _ => {
                    let variable = self.parse_variable();
                    parameters.push(Rc::new(RefCell::new(variable)));
                }
            };
        }

        return parameters;
    }

    /// FUNCTION_DEF -> fun VAR_NAME LPAREN (VAR_NAME : VAR_TYPE)* RPAREN (-> VarType)* LCURLY (STATEMENT[] - FUNCTION_DEF) RCURLY
    pub fn parse_function_definition(&mut self, f: ParserFunctions, is_extern_function_definition: bool) -> ASTNode {
        // we get here after consuming 'fun'
        let func_name_token = self.consume_token();

        let function_name = match &func_name_token.token {
            TokenEnum::Variable(n) => n,

            _ => {
                unexpected_token(&func_name_token, None);
            }
        };

        self.current_function_being_parsed = Some(function_name.clone());

        self.validate_and_consume_token(TokenEnum::Bracket(Bracket::LParen));

        // we validate closing ')' in the following function
        let parameters = self.parse_function_definition_parameters();

        let mut return_type = VarType::Unknown;

        if let TokenEnum::FunctionReturnIndicator = self.peek_next_token().token {
            self.consume_token();

            let (_, var_type) = self.parse_var_type();

            return_type = var_type;
        };

        let ff = function_name.clone();

        let function_def = if !is_extern_function_definition {
            self.validate_and_consume_token(TokenEnum::Bracket(Bracket::LCurly));

            // As we can fit an entire program inside a function
            // TODO: This introduces function and variable scoping issues
            self.inside_function_depth += 1;
            let block = self.parse_program();
            self.inside_function_depth -= 1;

            // println!("next token after parse_statements in parse_function_definition {:?}", self.peek_next_token().token);

            self.validate_and_consume_token(TokenEnum::Bracket(Bracket::RCurly));

            // Create an Rc from the Box
            FunctionDefinition::new(
                function_name.into(),
                parameters,
                block,
                return_type.clone(),
                func_name_token,
                is_extern_function_definition,
            )
        } else {
            FunctionDefinition::new(
                function_name.into(),
                parameters,
                Rc::new(RefCell::new(Box::new(Void))),
                return_type.clone(),
                func_name_token,
                is_extern_function_definition,
            )
        };

        let fdef: ASTNode = Rc::new(RefCell::new(Box::new(function_def)));

        // Use Rc::clone to get a reference-counted clone of the Rc, not the inner value
        f.borrow_mut().insert(
            ff,
            FunctionHashMapValue {
                func: Rc::clone(&fdef),
                return_type,
                is_extern_func: is_extern_function_definition,
            },
        );

        self.current_function_being_parsed = None;

        // Convert Rc back to Box for the return value
        return Rc::clone(&fdef);
    }
}
