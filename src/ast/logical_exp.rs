use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, VariableHashMap},
    lexer::{lexer::Token, tokens::TokenEnum},
};

use super::abstract_syntax_tree::{VisitResult, AST};

pub struct LogicalExpression {
    left: Rc<Box<dyn AST>>,
    op: Token,
    right: Rc<Box<dyn AST>>,
}

impl LogicalExpression {
    pub fn new(left: Rc<Box<dyn AST>>, op: Token, right: Rc<Box<dyn AST>>) -> Self {
        Self { left, op, right }
    }
}

impl AST for LogicalExpression {
    fn visit(&self, v: &mut VariableHashMap, f: Rc<RefCell<Functions>>) -> VisitResult {
        let left = self.left.visit(v, Rc::clone(&f));
        let right = self.right.visit(v, Rc::clone(&f));

        todo!()
    }

    fn visit_com(&self, v: &mut VariableHashMap, f: Rc<RefCell<Functions>>, asm: &mut ASM) {
        self.left.visit_com(v, Rc::clone(&f), asm);
        self.right.visit_com(v, Rc::clone(&f), asm);

        match &self.op.token {
            TokenEnum::LogicalOp(lo) => asm.gen_logical_statement(lo.clone()),

            _ => {
                panic!("Expected `or` or `and`")
            }
        };
    }

    fn get_token(&self) -> &Token {
        &self.op
    }

    fn print(&self) {
        todo!()
    }

    fn type_check(&self, call_stack: &crate::semantic::semantic_analyzer::CallStackRecord) {
        todo!()
    }
}
