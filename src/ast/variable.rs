use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        keywords::{TYPE_FLOAT, TYPE_INT, TYPE_STRING},
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
}

impl Variable {
    pub fn new(token: Box<Token>, var_type: String, var_name: String) -> Self {
        Self {
            token,
            var_type,
            var_name,
        }
    }

    pub fn get_var_enum_from_type(&self) -> VariableEnum {
        return match self.var_type.as_str() {
            TYPE_STRING => VariableEnum::String(String::from("")),
            TYPE_INT => VariableEnum::Number(Number::Integer(0)),

            t => unimplemented!("Type {t} not known"),
        };
    }
}

impl AST for Variable {
    fn visit_com(&self, x: &mut Variables, _: Rc<RefCell<Functions>>, asm: &mut ASM) {
        todo!()
    }

    fn visit(&self, _: &mut Variables, _: Rc<RefCell<Functions>>) -> VisitResult {
        todo!()
    }

    fn get_token(&self) -> &Token {
        return &self.token;
    }

    fn print(&self) {
        println!("{:#?}", self);
    }
}
