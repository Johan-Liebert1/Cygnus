use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{lexer::Token, tokens::TokenEnum},
    semantic_analyzer::semantic_analyzer::CallStack,
};

#[derive(Debug)]
pub struct VisitResult {
    pub token: Box<TokenEnum>,
}

pub trait AST {
    fn visit(
        &self,
        v: &mut Variables,
        f: Rc<RefCell<Functions>>,
        call_stack: &mut CallStack,
    ) -> VisitResult;
    fn visit_com(
        &self,
        v: &mut Variables,
        f: Rc<RefCell<Functions>>,
        asm: &mut ASM,
        call_stack: &mut CallStack,
    );
    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>);
    fn get_token(&self) -> &Token;
    fn print(&self);
}

impl Debug for dyn AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.print())
    }
}
