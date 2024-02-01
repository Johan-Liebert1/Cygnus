use crate::semantic_analyzer::semantic_analyzer::{CallStack, PopTypes};

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
    fn visit(
        &self,
        _v: &mut Variables,
        _f: Rc<RefCell<Functions>>,
        call_stack: &mut CallStack,
    ) -> VisitResult {
        todo!();

        // this is pretty straightforward. We simply return
        return VisitResult {
            token: Box::new(TokenEnum::Unknown("".into())),
        };
    }

    fn visit_com(
        &self,
        _v: &mut Variables,
        _f: Rc<RefCell<Functions>>,
        asm: &mut ASM,
        call_stack: &mut CallStack,
    ) {
        match self.typ {
            JumpType::Return => {
                // Since we break out of a loop or return from a function, we need to pop the call stack
                // call_stack.pop_special(PopTypes::EarlyReturn);
                asm.function_return()
            }
            JumpType::Break => {
                // Since we break out of a loop or return from a function, we need to pop the call stack
                // call_stack.pop_special(PopTypes::LoopBreak);
                asm.loop_break(call_stack.loop_num())
            }
        }
    }

    fn get_token(&self) -> &crate::lexer::lexer::Token {
        todo!()
    }

    fn print(&self) {
        todo!()
    }

    // TODO: Figure out if this matters
    fn semantic_visit(&mut self, call_stack: &mut CallStack, _f: Rc<RefCell<Functions>>) {
        // Since we break out of a loop or return from a function, we need to pop the call stack
        // match self.typ {
        //     JumpType::Return => {
        //         // Since we break out of a loop or return from a function, we need to pop the call stack
        //         call_stack.pop_special(PopTypes::EarlyReturn);
        //     }

        //     JumpType::Break => {
        //         // Since we break out of a loop or return from a function, we need to pop the call stack
        //         call_stack.pop_special(PopTypes::LoopBreak);
        //     }
        // }
    }
}
