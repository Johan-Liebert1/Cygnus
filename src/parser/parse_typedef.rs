use std::{cell::RefCell, clone, process::exit, rc::Rc};

use crate::{
    ast::typedef::{Typedef, TypedefType},
    helpers::unexpected_token,
    lexer::{keywords::TYPE_DEF, tokens::TokenEnum},
    types::ASTNode,
};

use super::parser::{Parser, UserDefinedType};

impl Parser {
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
            TokenEnum::Keyword(_) => todo!(),

            _ => {
                unexpected_token(&next_token, None);
                exit(1);
            }
        };

        // consume the type
        self.get_next_token();

        self.user_defined_types.push(UserDefinedType { name: type_name, type_: typedef_type });
    }
}
