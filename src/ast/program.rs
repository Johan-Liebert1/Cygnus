use crate::types::ASTNode;

use crate::semantic_analyzer::semantic_analyzer::{
    ActivationRecord, ActivationRecordType, CallStack,
};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::tokens::TokenEnum,
};
use std::{cell::RefCell, rc::Rc};

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct Program {
    statements: Vec<ASTNode>,
}

impl Program {
    pub fn new(statements: Vec<ASTNode>) -> Self {
        Self { statements }
    }
}

impl AST for Program {
    fn visit_com(&self, x: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM) {
        for statement in &self.statements {
            statement.borrow().visit_com(x, Rc::clone(&f), asm);
        }
    }

    fn visit(&self, x: &mut Variables, f: Rc<RefCell<Functions>>) -> VisitResult {
        let mut last: Option<VisitResult> = None;

        for statement in &self.statements {
            let result = statement.borrow().visit(x, Rc::clone(&f));
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
        println!("{:#?}", self);
    }

    fn semantic_visit(&self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        for statement in &self.statements {
            statement.borrow().semantic_visit(call_stack, Rc::clone(&f));
        }
    }
}
