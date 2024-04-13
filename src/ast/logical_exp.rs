use crate::{lexer::types::VarType, types::ASTNode};

use crate::semantic_analyzer::semantic_analyzer::CallStack;

use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{lexer::Token, tokens::TokenEnum},
};

use super::abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut, VisitResult, AST};

#[derive(Debug)]
pub struct LogicalExpression {
    left: Option<ASTNode>,
    op: Token,
    right: ASTNode,
    pub result_type: VarType,
}

impl LogicalExpression {
    pub fn new(left: Option<ASTNode>, op: Token, right: ASTNode) -> Self {
        Self {
            left,
            op,
            right,
            result_type: VarType::Int,
        }
    }
}

impl AST for LogicalExpression {
    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
        if let Some(left) = &self.left {
            left.borrow().visit(v, Rc::clone(&f), call_stack);
        };

        let _right = self.right.borrow().visit(v, Rc::clone(&f), call_stack);

        todo!()
    }

    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        if let Some(left) = &self.left {
            left.borrow().visit_com(v, Rc::clone(&f), asm, call_stack);
        }

        self.right.borrow().visit_com(v, Rc::clone(&f), asm, call_stack);

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
        if let Some(left) = &self.left {
            left.borrow_mut().semantic_visit(call_stack, f.clone());
        }

        self.right.borrow_mut().semantic_visit(call_stack, f);
    }

    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::LogicalExp(&self);
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::LogicalExp(self);
    }

    fn get_type(&self) -> (VarType, VarType) {
        return (
            self.result_type.get_actual_type(0, self.get_token()),
            self.result_type.clone(),
        );
    }
}
