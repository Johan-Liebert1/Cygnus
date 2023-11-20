use crate::{
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        keywords::{TYPE_FLOAT, TYPE_INT},
        lexer::Token,
        tokens::Number,
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
    block: Box<dyn AST>,
}

impl FunctionDefinition {
    pub fn new(name: String, arguments: Vec<Variable>, block: Box<dyn AST>) -> Self {
        Self {
            name,
            parameters: arguments,
            block,
        }
    }
}

impl AST for FunctionDefinition {
    fn visit(&self, i: &mut Variables, f: Rc<RefCell<&Functions>>) -> VisitResult {
        // TODO: handle global variables and function parameters with the same name
        for param in &self.parameters {
            let value = match param.var_type.as_str() {
                TYPE_INT => Number::Integer(0),
                TYPE_FLOAT => Number::Float(0.0),
                t => unimplemented!("Variable type {t} not implemented"),
            };

            i.insert(param.var_name.clone(), value);
        }

        let t = self.block.visit(i, f);

        for param in &self.parameters {
            i.remove(&param.var_name);
        }

        return t;
    }

    fn get_token(&self) -> &Token {
        todo!()
    }

    fn print(&self) {
        println!("{:?}", &self);
    }
}
