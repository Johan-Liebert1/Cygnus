use crate::types::ASTNode;

use crate::semantic_analyzer::semantic_analyzer::CallStack;

use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{lexer::Token, tokens::TokenEnum},
};

use super::abstract_syntax_tree::{VisitResult, AST, ASTNodeEnum, ASTNodeEnumMut};

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
    fn visit(
        &self,
        v: &mut Variables,
        f: Rc<RefCell<Functions>>,
        call_stack: &mut CallStack,
    ) -> VisitResult {
        let _left = self.left.borrow().visit(v, Rc::clone(&f), call_stack);
        let _right = self.right.borrow().visit(v, Rc::clone(&f), call_stack);

        todo!()
    }

    fn visit_com(
        &self,
        v: &mut Variables,
        f: Rc<RefCell<Functions>>,
        asm: &mut ASM,
        call_stack: &mut CallStack,
    ) {
        self.left
            .borrow()
            .visit_com(v, Rc::clone(&f), asm, call_stack);
        self.right
            .borrow()
            .visit_com(v, Rc::clone(&f), asm, call_stack);

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

    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        self.left.borrow_mut().semantic_visit(call_stack, f.clone());
        self.right.borrow_mut().semantic_visit(call_stack, f);
    }


    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::LogicalExp(&self);
    }


    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::LogicalExp(self);
    }
}
