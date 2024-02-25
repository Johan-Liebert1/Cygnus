use crate::{
    ast::variable::Variable,
    lexer::{lexer::Token, types::VarType},
    trace,
    types::ASTNode,
};

use std::{cell::RefCell, rc::Rc};

use crate::{ast::assignment_statement::AssignmentStatement, lexer::tokens::TokenEnum};

use super::parser::Parser;

impl<'a> Parser<'a> {
    pub fn parse_assignment_statement(
        &mut self,
        var_token: Token,
        var_name: String,
        times_dereferenced: usize,
    ) -> ASTNode {
        // we get here after parsing the variable name
        let validated_token =
            self.validate_any_token(vec![TokenEnum::Equals, TokenEnum::PlusEquals, TokenEnum::MinusEquals]);

        let right = self.parse_logical_expression();

        return Rc::new(RefCell::new(Box::new(AssignmentStatement::new(
            Variable::new(
                Box::new(var_token),
                VarType::Unknown,
                var_name,
                times_dereferenced > 0,
                false,
                times_dereferenced,
            ),
            validated_token.get_assignment_type(),
            right,
        ))));
    }
}
