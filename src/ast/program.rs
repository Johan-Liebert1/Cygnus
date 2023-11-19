use crate::{interpreter::interpreter::Variables, lexer::tokens::TokenEnum};

use super::abstract_syntax_tree::{AST, VisitResult};

pub struct Program {
    statements: Vec<Box<dyn AST>>,
}

impl Program {
    pub fn new(statements: Vec<Box<dyn AST>>) -> Self {
        Self { statements }
    }
}

impl AST for Program {
    fn visit(&self, x: &mut Variables) -> VisitResult {
        for statement in &self.statements {
            let result = statement.visit(x);

            // println!("{:?}", result);
        }

        VisitResult {
            token: Box::new(TokenEnum::Unknown("".to_string()))
        }
    }

    fn get_token(&self) -> &crate::lexer::lexer::Token {
        todo!()
    }

    fn print(&self) {
        todo!()
    }
}
