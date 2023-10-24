use crate::{
    ast::{
        abstract_syntax_tree::AST, assignment_statement::AssignmentStatement, variable::Variable,
    },
    lexer::tokens::TokenEnum,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /// VARIABLE_DECLARATION -> def VAR_NAME (= COMPARISON_EXPRESSION)*
    pub fn parse_variable(&mut self) -> Variable {
        let token = self.get_next_token();

        if let TokenEnum::Variable(_) = token.token {
            return Variable::new(Box::new(token));
        }

        panic!("Expected a variable found {:?}", token);
    }

    pub fn parse_assignment_statement(&mut self) -> Box<dyn AST> {
        let left = self.parse_variable();

        match self.get_next_token().token {
            TokenEnum::Equals => {
                // fine just consume the token
            }

            _ => {
                panic!("Expected assignment")
            }
        };

        // TODO: handle function calls and strings and stuff here
        return Box::new(AssignmentStatement::new(
            left,
            self.parse_comparison_expression(),
        ));
    }
}
