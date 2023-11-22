use std::{cell::RefCell, rc::Rc};

use crate::{
    interpreter::interpreter::{Functions, Variables},
    lexer::{keywords::FUNC_OUTPUT, lexer::Token, tokens::TokenEnum},
};

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct FunctionCall {
    name: String,
    arguments: Vec<Box<dyn AST>>,
}

impl FunctionCall {
    pub fn new(name: String, arguments: Vec<Box<dyn AST>>) -> Self {
        Self { name, arguments }
    }
}

impl AST for FunctionCall {
    fn visit(&self, i: &mut Variables, f: Rc<RefCell<Functions>>) -> VisitResult {
        match self.name.as_str() {
            FUNC_OUTPUT => {
                return VisitResult {
                    token: Box::new(TokenEnum::Unknown("".into())),
                };
            }

            name => match f.borrow().get(name) {
                Some(function_ast) => function_ast.visit(i, Rc::clone(&f)),

                None => unimplemented!("Function {} unimplemented", self.name),
            },
        }
    }

    fn get_token(&self) -> &Token {
        todo!()
    }

    fn print(&self) {
        println!("{:?}", &self);
    }
}
