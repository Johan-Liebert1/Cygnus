use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::tokens::TokenEnum,
};
use std::{cell::RefCell, rc::Rc};

use super::abstract_syntax_tree::{VisitResult, AST};

pub struct Program {
    statements: Vec<Rc<Box<dyn AST>>>,
}

impl Program {
    pub fn new(statements: Vec<Rc<Box<dyn AST>>>) -> Self {
        Self { statements }
    }
}

impl AST for Program {
    fn visit_com(&self, x: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM) {
        for statement in &self.statements {
            statement.visit_com(x, Rc::clone(&f), asm);
        }
    }

    fn visit(&self, x: &mut Variables, f: Rc<RefCell<Functions>>) -> VisitResult {
        let mut last: Option<VisitResult> = None;

        for statement in &self.statements {
            let result = statement.visit(x, Rc::clone(&f));
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
