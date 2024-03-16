use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{lexer::Token, types::VarType},
    semantic_analyzer::semantic_analyzer::CallStack, trace,
};

use super::{
    abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut, VisitResult, AST},
    variable::Variable,
};

pub type StructMembers = Vec<Variable>;

#[derive(Debug)]
pub struct Struct {
    name: String,
    members: StructMembers,
}

impl Struct {
    pub fn new(name: String, members: StructMembers) -> Self {
        Self { name, members }
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
        trace!("STRUCT semantic_visit TODO");
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
        todo!()
    }
}
