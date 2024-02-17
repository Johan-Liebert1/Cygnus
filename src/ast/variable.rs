use crate::{
    lexer::types::{TYPE_FLOAT, TYPE_INT, TYPE_STRING, VarType},
    semantic_analyzer::semantic_analyzer::CallStack,
    trace,
};

use core::panic;
use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        lexer::Token,
        tokens::{Number, VariableEnum},
    },
};

use super::abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut, VisitResult, AST};

#[derive(Debug)]
pub struct Variable {
    token: Box<Token>,
    pub var_name: String,
    pub var_type: VarType,
    pub dereference: bool,
    pub store_address: bool,
    pub times_dereferenced: usize,
}

impl Variable {
    pub fn new(
        token: Box<Token>,
        var_type: VarType,
        var_name: String,
        dereference: bool,
        store_address: bool,
        times_dereferenced: usize,
    ) -> Self {
        Self {
            token,
            var_type,
            var_name,
            dereference,
            store_address,
            times_dereferenced,
        }
    }

    pub fn get_var_enum_from_type(&self) -> VariableEnum {
        return match self.var_type {

            // TYPE_STRING => VariableEnum::String(String::from("")),
            // TYPE_INT => VariableEnum::Number(Number::Integer(0)),

            // t => match &t[1..] {
            //     TYPE_INT | TYPE_STRING | TYPE_FLOAT => VariableEnum::Pointer(t[1..].into()),
            //     _ => unimplemented!("Type {t} not known"),
            // },

            VarType::Int => VariableEnum::Number(Number::Integer(0)),
            VarType::Str => VariableEnum::String(String::from("")),
            VarType::Float => todo!(),
            VarType::Ptr(_) => todo!(),
        };
    }
}

impl AST for Variable {
    fn visit_com(&self, _x: &mut Variables, _: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        asm.gen_asm_for_var(&self, &call_stack);
    }

    fn visit(&self, _: &mut Variables, _: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
        todo!()
    }

    fn get_token(&self) -> &Token {
        return &self.token;
    }

    fn print(&self) {
        println!("{:#?}", self);
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, _f: Rc<RefCell<Functions>>) {
        if !call_stack.var_with_name_found(&self.var_name) {
            panic!("Variable with name '{}' not found in current scope", self.var_name);
        }
    }

    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::Variable(&self);
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::Variable(self);
    }
}
