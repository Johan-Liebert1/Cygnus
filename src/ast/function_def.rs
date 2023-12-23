use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        keywords::{TYPE_FLOAT, TYPE_INT},
        lexer::Token,
        tokens::{Number, TokenEnum, VariableEnum},
    },
};
use std::{cell::RefCell, rc::Rc};

use super::{
    abstract_syntax_tree::{VisitResult, AST},
    variable::Variable,
};

#[derive(Debug)]
pub struct FunctionDefinition {
    name: String,
    parameters: Vec<Variable>,
    block: Rc<Box<dyn AST>>,
}

impl FunctionDefinition {
    pub fn new(name: String, arguments: Vec<Variable>, block: Rc<Box<dyn AST>>) -> Self {
        Self {
            name,
            parameters: arguments,
            block,
        }
    }
}

impl AST for FunctionDefinition {
    fn visit_com(&self, x: &mut Variables, _: Rc<RefCell<Functions>>, asm: &mut ASM) {
        todo!()
    }

    // TODO: This function will be visited twice, once when the interpreter calls visit, and
    // another when the function is actually called
    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>) -> VisitResult {
        // TODO: handle global variables and function parameters with the same name
        for param in &self.parameters {
            let value = match param.var_type.as_str() {
                TYPE_INT => Number::Integer(0),
                TYPE_FLOAT => Number::Float(0.0),
                t => unimplemented!("Variable type {t} not implemented"),
            };

            v.insert(param.var_name.clone(), VariableEnum::Number(value));
        }

        self.block.visit(v, f);

        for param in &self.parameters {
            v.remove(&param.var_name.clone());
        }

        return VisitResult {
            token: Box::new(TokenEnum::Unknown("".to_string())),
        };
    }

    fn get_token(&self) -> &Token {
        todo!()
    }

    fn print(&self) {
        println!("{:?}", &self);
    }
}
