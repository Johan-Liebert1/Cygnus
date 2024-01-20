use std::rc::Rc;

use crate::{
    ast::{abstract_syntax_tree::AST, assignment_statement::AssignmentStatement},
    lexer::tokens::TokenEnum,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    pub fn parse_assignment_statement(&mut self, var_name: String) -> Rc<Box<dyn AST>> {
        // we get here after parsing the variable name
        self.validate_token(TokenEnum::Equals);

        let right = self.parse_logical_expression();

        let mut var = self.variables.get(&var_name);

        if var.is_none() {
            for (key, value) in self.function_variables.borrow().iter() {
            }
        }

        return Rc::new(Box::new(AssignmentStatement::new(var_name, None, right)));
    }
}
