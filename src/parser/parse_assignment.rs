use crate::types::ASTNode;

use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{abstract_syntax_tree::AST, assignment_statement::AssignmentStatement},
    lexer::tokens::TokenEnum,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    pub fn parse_assignment_statement(&mut self, var_name: String) -> ASTNode {
        // we get here after parsing the variable name

        self.validate_token(TokenEnum::Equals);

        let right = self.parse_logical_expression();

        return Rc::new(RefCell::new(Box::new(AssignmentStatement::new(
            var_name, right,
        ))));
    }
}
