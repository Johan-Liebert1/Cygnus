use crate::semantic_analyzer::semantic_analyzer::CallStack;

use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::tokens::TokenEnum,
};

use super::abstract_syntax_tree::{VisitResult, AST};

pub enum JumpType {
    Return,
    Break,
}

pub struct Jump {
    typ: JumpType,
}

impl Jump {
    pub fn new(typ: JumpType) -> Self {
        Self { typ }
    }
}

impl AST for Jump {
    fn visit(&self, _v: &mut Variables, _f: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
        todo!();

        // this is pretty straightforward. We simply return
        return VisitResult {
            token: Box::new(TokenEnum::Unknown("".into())),
        };
    }

    fn visit_com(&self, _v: &mut Variables, _f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        // Since we break out of a loop or return from a function, we need to pop the call stack
        call_stack.pop();

        match self.typ {
            JumpType::Return => asm.function_return(),
            JumpType::Break => asm.loop_break(),
        }
    }

    fn get_token(&self) -> &crate::lexer::lexer::Token {
        todo!()
    }

    fn print(&self) {
        todo!()
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, _f: Rc<RefCell<Functions>>) {
        // Since we break out of a loop or return from a function, we need to pop the call stack
        call_stack.pop();
    }
}
