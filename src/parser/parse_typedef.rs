use std::{cell::RefCell, clone, process::exit, rc::Rc};

use crate::{
    ast::typedef::{Typedef, TypedefType},
    helpers::unexpected_token,
    lexer::{
        keywords::{FUNCTION_DEFINE, TYPE_DEF},
        lexer::Token,
        tokens::{Bracket, TokenEnum},
        types::VarType,
    },
    trace,
    types::ASTNode,
};

use super::parser::{Parser, UserDefinedType};

impl Parser {
    fn get_all_params(&mut self) -> Vec<VarType> {
        let mut parameters: Vec<VarType> = vec![];

        loop {
            let token = self.peek_next_token();

            match &token.token {
                TokenEnum::Bracket(b) => match b {
                    Bracket::RParen => {
                        return parameters;
                    }

                    _ => {
                        unexpected_token(&token, None);
                    }
                },

                TokenEnum::Comma => {
                    self.get_next_token();
                    continue;
                }

                _ => {
                    parameters.push(self.parse_var_type().1);
                }
            };
        }

        return parameters;
    }

    // Returns parameters and return type
    pub fn parse_function_typedef(&mut self) -> (Vec<VarType>, VarType) {
        // Consume 'def'
        self.validate_token(TokenEnum::Keyword(FUNCTION_DEFINE.into()));

        self.validate_token(TokenEnum::Bracket(Bracket::LParen));

        let parameters = self.get_all_params();

        self.validate_token(TokenEnum::Bracket(Bracket::RParen));

        self.validate_token(TokenEnum::FunctionReturnIndicator);

        let return_type = self.parse_var_type();

        return (parameters, return_type.1);
    }

    // We get here after parsing 'type'
    pub fn parse_typedef(&mut self) {
        let mut type_name: String = String::new();

        let next_token = self.get_next_token();

        if let TokenEnum::Variable(var_name) = next_token.token {
            type_name = var_name;
        } else {
            unexpected_token(&next_token, Some(&TokenEnum::Variable("".into())));
        }

        self.validate_token(TokenEnum::Equals);

        let next_token = self.peek_next_token();

        let typedef_type = match &next_token.token {
            // Type alias
            TokenEnum::Type(type_) => type_.clone(),

            // Struct typedef alias
            TokenEnum::Variable(type_name) => {
                println!("TokenEnum::Variable {type_name}");
                todo!()
            }

            // Function typedef
            TokenEnum::Keyword(..) => {
                let (params, return_type) = self.parse_function_typedef();

                VarType::Function(type_name.clone(), params, Box::new(return_type))
            }

            _ => {
                unexpected_token(&next_token, None);
                exit(1);
            }
        };

        // consume the type
        self.get_next_token();

        self.user_defined_types.push(UserDefinedType {
            name: type_name,
            type_: typedef_type,
        });
    }

    // we arrive here after parsing the 'extren' keyword
    //
    // extern fun FuncName(type1, type2, type3, ...) (-> ReturnType)*
    pub fn parse_extern_func_definition(&mut self) {
        self.validate_token(TokenEnum::Keyword(FUNCTION_DEFINE.into()));
        todo!()
    }
}
