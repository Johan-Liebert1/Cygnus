use crate::{interpreter::interpreter::Variables, lexer::tokens::TokenEnum};

use super::abstract_syntax_tree::{VisitResult, AST};

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
        let mut last: Option<VisitResult> = None;

        for statement in &self.statements {
            let result = statement.visit(x);
            last = Some(result);
        }

        if let Some(res) = last {
            return res;
        }

        VisitResult {
            token: Box::new(TokenEnum::Unknown("".into())),
        }
    }

    fn get_token(&self) -> &crate::lexer::lexer::Token {
        todo!()
    }

    fn print(&self) {
        todo!()
    }
}
