use core::panic;
use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    helpers::compiler_error,
    interpreter::interpreter::{Functions, Variables},
    lexer::{lexer::Token, types::VarType},
    semantic_analyzer::semantic_analyzer::CallStack,
    types::ASTNode, trace,
};

use super::abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut, VisitResult, AST};

#[derive(Debug)]
pub struct Array {
    members: Vec<ASTNode>,
    token: Token,
    size: usize,
    pub result_type: VarType,
}

impl Array {
    pub fn new(members: Vec<ASTNode>, token: Token) -> Self {
        Self {
            size: members.len(),
            members,
            token,
            result_type: VarType::Unknown,
        }
    }

    fn get_member_type(member: &ASTNode) -> VarType {
        return match member.borrow().get_node() {
            ASTNodeEnum::BinaryOp(node) => node.result_type.clone(),
            ASTNodeEnum::ComparisonExp(node) => node.result_type.clone(),
            ASTNodeEnum::Factor(node) => node.result_type.clone(),
            ASTNodeEnum::FunctionCall(node) => node.result_type.clone(),
            ASTNodeEnum::LogicalExp(node) => node.result_type.clone(),
            ASTNodeEnum::Variable(node) => node.result_type.clone(),
            ASTNodeEnum::MemoryAlloc(node) => node.result_type.clone(),

            _ => {
                panic!("");
            }
        };
    }
}

impl AST for Array {
    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
        todo!()
    }

    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        for member in self.members.iter().rev() {
            member.borrow().visit_com(v, f.clone(), asm, call_stack);
        }
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        if self.size == 0 {
            compiler_error("Zero length arrays are not supported", &self.token);
        }

        self.members[0].borrow_mut().semantic_visit(call_stack, f.clone());

        let first_member_type = Array::get_member_type(&self.members[0]);

        for member in self.members.iter().skip(1) {
            member.borrow_mut().semantic_visit(call_stack, f.clone());

            let new_member_type = Array::get_member_type(&member);

            if first_member_type != new_member_type {
                compiler_error(
                    format!(
                        "{new_member_type} is not assignable to array of type {}",
                        self.result_type
                    ),
                    member.borrow().get_token(),
                );
            }
        }

        self.result_type = VarType::Array(Box::new(first_member_type), self.members.len());

    }

    fn get_token(&self) -> &Token {
        todo!()
    }

    fn get_node(&self) -> ASTNodeEnum {
        ASTNodeEnum::Array(self)
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        ASTNodeEnumMut::Array(self)
    }

    fn print(&self) {
        todo!()
    }
}
