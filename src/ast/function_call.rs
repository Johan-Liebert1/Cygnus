use std::{cell::RefCell, rc::Rc};

use crate::{
    interpreter::interpreter::{Functions, Variables},
    lexer::{keywords::FUNC_WRITE, lexer::Token, tokens::TokenEnum}, asm::asm::ASM,
};

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct FunctionCall {
    name: String,
    arguments: Vec<Rc<Box<dyn AST>>>,
}

impl FunctionCall {
    pub fn new(name: String, arguments: Vec<Rc<Box<dyn AST>>>) -> Self {
        Self { name, arguments }
    }
}

impl AST for FunctionCall {
    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM) {
        match self.name.as_str() {
            FUNC_WRITE => {
                for arg in &self.arguments {
                    // this will generate everything and put in rax
                    println!("{:?}", arg);

                    arg.visit_com(v, Rc::clone(&f), asm);
                    asm.func_write();
                }
            }

            name => match f.borrow().get(name) {
                Some(function_ast) => {
                    println!("Visiting func {name}");
                    function_ast.visit(v, Rc::clone(&f));
                }

                None => unimplemented!("Function {} unimplemented", self.name),
            },
        }
    }

    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>) -> VisitResult {
        match self.name.as_str() {
            FUNC_WRITE => {
                for arg in &self.arguments {
                    println!("{:?}", arg.visit(v, Rc::clone(&f)));
                }

                return VisitResult {
                    token: Box::new(TokenEnum::Unknown("".into())),
                };
            }

            name => match f.borrow().get(name) {
                Some(function_ast) => {
                    println!("Visiting func {name}");

                    function_ast.visit(v, Rc::clone(&f))
                }

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
