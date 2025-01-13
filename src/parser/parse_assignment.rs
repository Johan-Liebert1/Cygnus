use crate::{
    ast::variable::Variable,
    helpers::unexpected_token,
    lexer::{lexer::Token, types::VarType},
    types::ASTNode,
};

use std::{cell::RefCell, rc::Rc};

use crate::{ast::assignment_statement::AssignmentStatement, lexer::tokens::TokenEnum};

use super::parser::{Generic, Parser};

impl Parser {
    pub fn parse_assignment_statement(
        &mut self,
        var_token: Token,
        var_name: String,
        times_dereferenced: usize,
        array_access_index: Option<ASTNode>,
    ) -> ASTNode {
        // We get here after consuming the variable token

        self.parsing_variable_assignment = Generic {
            status: true,
            value: var_name.clone(),
        };

        let mut member_access = vec![];

        while matches!(self.peek_next_token().token, TokenEnum::Dot) {
            // this is a struct member assignment

            // consume '.'
            self.consume_token();

            let next = self.consume_token();

            match next.token {
                TokenEnum::Variable(name) => {
                    member_access.push(name);
                }

                _ => {
                    unexpected_token(&next, Some(&TokenEnum::Variable("".into())));
                }
            };
        }

        let validated_token =
            self.validate_any_token(vec![TokenEnum::Equals, TokenEnum::PlusEquals, TokenEnum::MinusEquals]);

        let right = self.parse_logical_expression();

        let mut variable = Variable::new(
            var_token,
            VarType::Unknown,
            var_name,
            times_dereferenced > 0,
            false,
            times_dereferenced,
        );

        variable.array_aceess_index = array_access_index;
        variable.member_access = member_access;

        return Rc::new(RefCell::new(Box::new(AssignmentStatement::new(
            variable,
            validated_token.get_assignment_type(),
            right,
        ))));
    }
}
