use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::structs::{StructDecleration, StructMember},
    helpers::{unexpected_token, unexpected_token_string},
    lexer::{
        tokens::{Bracket, TokenEnum},
        types::{StructMemberType, VarType},
    },
    types::ASTNode,
};

use super::parser::{Parser, UserDefinedType};

impl Parser {
    pub fn parse_struct_definition(&mut self) {
        let name: String;

        let next_token = self.get_next_token();

        if let TokenEnum::Variable(var_name) = next_token.token {
            name = var_name;
        } else {
            unexpected_token(&next_token, Some(&TokenEnum::Variable("".into())));
        }

        self.validate_token(TokenEnum::Bracket(Bracket::LCurly));

        let mut members = vec![];

        loop {
            if matches!(self.peek_next_token().token, TokenEnum::Bracket(Bracket::RCurly)) {
                break;
            }

            // struct A {
            // a: int,
            // b: str,
            // }

            let var = self.parse_variable();

            members.push(StructMemberType {
                name: var.var_name,
                member_type: var.var_type,
                offset: 0,
            });

            if matches!(self.peek_next_token().token, TokenEnum::Comma) {
                self.get_next_token();
            }
        }

        self.validate_token(TokenEnum::Bracket(Bracket::RCurly));

        self.user_defined_types.push(UserDefinedType {
            name: name.clone(),
            // TODO: This clone can be easily not cloned
            type_: VarType::Struct(name.clone(), Rc::new(RefCell::new(members))),
        });
    }

    pub fn parse_struct_decleration(&mut self) -> ASTNode {
        let struct_name: String;

        let name_token = self.get_next_token();

        if let TokenEnum::Variable(ref name) = name_token.token {
            struct_name = name.into();
        } else {
            unexpected_token_string(&name_token, "Struct name");
        }

        self.validate_token(TokenEnum::Bracket(Bracket::LCurly));

        let mut members = vec![];

        loop {
            if matches!(self.peek_next_token().token, TokenEnum::Bracket(Bracket::RCurly)) {
                break;
            }

            // A {
            //      a: 1,
            //      b: "hello",
            // }
            let var_token = self.get_next_token();

            let var_name: String;

            if let TokenEnum::Variable(ref name) = var_token.token {
                var_name = name.into();
            } else {
                unexpected_token(&var_token, Some(&TokenEnum::Variable("".into())));
            }

            self.validate_token(TokenEnum::Colon);
            let variable_assigned_to = self.parse_logical_expression();

            members.push(StructMember {
                var_token,
                name: var_name.into(),
                rhs: variable_assigned_to,
            });

            if matches!(self.peek_next_token().token, TokenEnum::Comma) {
                self.get_next_token();
            }
        }

        self.validate_token(TokenEnum::Bracket(Bracket::RCurly));

        return Rc::new(RefCell::new(Box::new(StructDecleration::new(
            struct_name,
            members,
            name_token,
        ))));
    }
}
