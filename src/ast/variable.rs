use crate::{
    helpers,
    lexer::types::{VarType, TYPE_FLOAT, TYPE_INT, TYPE_STRING},
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

#[derive(Debug, Clone)]
pub struct Variable {
    token: Box<Token>,
    pub var_name: String,
    pub var_type: VarType,
    pub result_type: VarType,
    pub dereference: bool,
    pub store_address: bool,
    pub times_dereferenced: usize,
    pub offset: usize,
    pub is_memory_block: bool,
    pub type_cast: Option<VarType>,
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
            result_type: var_type.clone(),
            var_type,
            var_name,
            dereference,
            store_address,
            times_dereferenced,
            offset: 0,
            is_memory_block: false,
            type_cast: None,
        }
    }

    pub fn size(&self) -> usize {
        return match self.var_type {
            // 64 bit integer
            VarType::Int => 8,
            // 8 bytes for length + 8 bytes for pointer to the start of the string
            VarType::Str => 16,
            VarType::Float => todo!(),
            // char is only 1 byte
            VarType::Char => 1,
            // Pointer will always consume 8 bytes
            VarType::Ptr(_) => 8,
            VarType::Unknown => todo!(),
        };
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
            VarType::Char => todo!(),
            VarType::Ptr(_) => todo!(),
            VarType::Unknown => todo!(),
        };
    }

    pub fn store_result_type(&mut self, var_type: &VarType, times_dereferenced: usize) {
        self.result_type = var_type.get_actual_type(times_dereferenced);
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
        let (variable_in_stack, _) = call_stack.get_var_with_name(&self.var_name);

        if let Some(variable_in_stack) = variable_in_stack {
            self.var_type = if let Some(casted_type) = &self.type_cast {
                casted_type.clone()
            } else {
                variable_in_stack.var_type.clone()
            };

            self.result_type = self.var_type.get_actual_type(self.times_dereferenced);
        } else {
            helpers::compiler_error(
                format!("Variable with name '{}' not found in current scope", self.var_name),
                &self.token,
            );
        }
    }

    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::Variable(&self);
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::Variable(self);
    }
}
