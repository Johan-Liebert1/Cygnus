use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables}, lexer::tokens::TokenEnum,
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
    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>) -> VisitResult {
        todo!();

        // this is pretty straightforward. We simply return
        return VisitResult {
            token: Box::new(TokenEnum::Unknown("".into()))
        };
    }

    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM) {
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
}
