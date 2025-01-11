use crate::semantic_analyzer::semantic_analyzer::CallStack;

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{lexer::Token, types::VarType},
};
use std::{cell::RefCell, rc::Rc};

use super::abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut};
use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub enum TypedefType {
    Primitive(VarType),
    FunctionType,
}

#[derive(Debug)]
pub struct FunctionType {
    pub parameters: Vec<VarType>,
    pub return_type: VarType,
}

#[derive(Debug)]
pub struct Typedef {
    name: String,
    token: Token,
    typedef_type: TypedefType,
}

impl Typedef {
    pub fn new(name: String, token: Token, typedef_type: TypedefType) -> Self {
        Self {
            name,
            token,
            typedef_type,
        }
    }
}

impl AST for Typedef {
    fn visit(&self, _: &mut Variables, _: Rc<RefCell<Functions>>, _: &mut CallStack) -> VisitResult {
        todo!()
    }

    fn visit_com(&self, _: &mut Variables, _: Rc<RefCell<Functions>>, _: &mut ASM, _: &mut CallStack) {
        todo!()
    }

    fn semantic_visit(&mut self, _: &mut CallStack, _: Rc<RefCell<Functions>>) {
        todo!()
    }

    fn get_token(&self) -> &Token {
        todo!()
    }

    fn get_node(&self) -> ASTNodeEnum {
        todo!()
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        todo!()
    }

    fn get_type(&self) -> (VarType, VarType) {
        todo!()
    }

    fn print(&self) {
        todo!()
    }
}
