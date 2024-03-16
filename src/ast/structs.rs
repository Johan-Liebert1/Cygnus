use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        lexer::Token,
        types::{StructMemberType, VarType},
    },
    semantic_analyzer::semantic_analyzer::CallStack,
    trace,
    types::ASTNode,
};

use super::{
    abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut, VisitResult, AST},
    variable::Variable,
};

#[derive(Debug)]
pub struct StructMember {
    pub var_token: Token,
    pub name: String,
    pub rhs: ASTNode,
}

#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub members: Vec<StructMember>,
    result_type: VarType,
}

impl Struct {
    pub fn new(name: String, members: Vec<StructMember>) -> Self {
        Self {
            name,
            members,
            result_type: VarType::Unknown,
        }
    }
}

impl AST for Struct {
    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
        todo!()
    }

    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        todo!()
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        let mut member_types = vec![];

        for member in &mut self.members {
            member.rhs.borrow_mut().semantic_visit(call_stack, f.clone());

            member_types.push(StructMemberType {
                name: member.name.clone(),
                member_type: member.rhs.borrow().get_type().0,
            });
        }

        self.result_type = VarType::Struct(self.name.clone(), Rc::new(RefCell::new(member_types)));
    }

    fn get_token(&self) -> &Token {
        todo!()
    }

    fn get_node(&self) -> ASTNodeEnum {
        ASTNodeEnum::Struct(self)
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        ASTNodeEnumMut::Struct(self)
    }

    fn print(&self) {
        todo!()
    }

    fn get_type(&self) -> (VarType, VarType) {
        (self.result_type.clone(), self.result_type.clone())
    }
}
