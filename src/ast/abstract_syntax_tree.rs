use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, VariableHashMap},
    lexer::{lexer::Token, tokens::TokenEnum},
    semantic::semantic_analyzer::CallStackRecord,
};

#[derive(Debug)]
pub struct VisitResult {
    pub token: Box<TokenEnum>,
}

pub trait AST {
    fn visit(&self, v: &mut VariableHashMap, f: Rc<RefCell<Functions>>) -> VisitResult;
    fn visit_com(&self, v: &mut VariableHashMap, f: Rc<RefCell<Functions>>, asm: &mut ASM);
    fn type_check(&self, call_stack: &CallStackRecord);
    fn get_token(&self) -> &Token;
    fn print(&self);
}

impl Debug for dyn AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.print())
    }
}

impl Display for dyn AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.get_token())
    }
}
