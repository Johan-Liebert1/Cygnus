use crate::{lexer::keywords::TYPE_FLOAT, semantic_analyzer::semantic_analyzer::CallStack, trace};

use core::panic;
use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        keywords::{TYPE_INT, TYPE_STRING},
        lexer::Token,
        tokens::{Number, VariableEnum},
    },
};

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct Variable {
    token: Box<Token>,
    pub var_name: String,
    pub var_type: String,
    pub dereference: bool,
    pub store_address: bool,
    pub string_number: usize,
}

impl Variable {
    pub fn new(
        token: Box<Token>,
        var_type: String,
        var_name: String,
        dereference: bool,
        store_address: bool,
    ) -> Self {
        Self {
            token,
            var_type,
            var_name,
            dereference,
            store_address,
            string_number: 0,
        }
    }

    pub fn get_var_enum_from_type(&self) -> VariableEnum {
        return match self.var_type.as_str() {
            TYPE_STRING => VariableEnum::String(String::from("")),
            TYPE_INT => VariableEnum::Number(Number::Integer(0)),

            t => match &t[1..] {
                TYPE_INT | TYPE_STRING | TYPE_FLOAT => VariableEnum::Pointer(t[1..].into()),
                _ => unimplemented!("Type {t} not known"),
            },
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
}
