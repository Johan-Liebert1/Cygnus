use crate::types::ASTNode;

use crate::semantic_analyzer::semantic_analyzer::CallStack;

use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{lexer::Token, tokens::TokenEnum},
};

use super::abstract_syntax_tree::{VisitResult, AST};

pub struct LogicalExpression {
    left: ASTNode,
    op: Token,
    right: ASTNode,
}

impl LogicalExpression {
    pub fn new(left: ASTNode, op: Token, right: ASTNode) -> Self {
        Self { left, op, right }
    }
}

impl AST for LogicalExpression {
    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>) -> VisitResult {
        let _left = self.left.borrow().visit(v, Rc::clone(&f));
        let _right = self.right.borrow().visit(v, Rc::clone(&f));

        todo!()
    }

    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM) {
        self.left.borrow().visit_com(v, Rc::clone(&f), asm);
        self.right.borrow().visit_com(v, Rc::clone(&f), asm);

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

    fn semantic_visit(&self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        self.left.borrow().semantic_visit(call_stack, f.clone());
        self.right.borrow().semantic_visit(call_stack, f);
    }
}
