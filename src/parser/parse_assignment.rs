use crate::types::ASTNode;

use std::{cell::RefCell, rc::Rc};

use crate::{ast::assignment_statement::AssignmentStatement, lexer::tokens::TokenEnum};

use super::parser::Parser;

impl<'a> Parser<'a> {
    pub fn parse_assignment_statement(&mut self, var_name: String) -> ASTNode {
        // we get here after parsing the variable name
        let validated_token = self.validate_any_token(vec![
            TokenEnum::Equals,
            TokenEnum::PlusEquals,
            TokenEnum::MinusEquals,
        ]);

        let right = self.parse_logical_expression();

        return Rc::new(RefCell::new(Box::new(AssignmentStatement::new(
            var_name,
            validated_token.get_assignment_type(),
            right,
        ))));
    }
}
