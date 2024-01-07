use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, VariableHashMap},
    lexer::lexer::Token,
    trace,
};
use std::{cell::RefCell, rc::Rc};

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct Expression {
    left: Box<dyn AST>,
    operand: Box<Token>,
    right: Box<dyn AST>,
}

impl AST for Expression {
    fn visit_com(&self, x: &mut VariableHashMap, _: Rc<RefCell<Functions>>, asm: &mut ASM) {
        todo!()
    }

    fn visit(&self, _: &mut VariableHashMap, _: Rc<RefCell<Functions>>) -> VisitResult {
        trace!("Expression visit");
        VisitResult {
            token: Box::new(self.operand.token.clone()),
        }
    }

    fn get_token(&self) -> &Token {
        return &self.operand;
    }

    fn print(&self) {
        trace!("{:#?}", self);
    }

    fn type_check(&self, call_stack: &crate::semantic::semantic_analyzer::CallStackRecord) {
        todo!()
    }
}

impl Expression {
    pub fn new(left: Box<dyn AST>, operand: Box<Token>, right: Box<dyn AST>) -> Self {
        Expression {
            left,
            operand,
            right,
        }
    }
}
