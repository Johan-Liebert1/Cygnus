use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{keywords::{FUNC_WRITE, FUNC_STRLEN}, lexer::Token, tokens::TokenEnum},
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
                    // println!("FunctionCall visit_com arg: {:#?}", arg);

                    // this will generate everything and put in rax
                    arg.visit_com(v, Rc::clone(&f), asm);

                    let arg_token = &arg.get_token().token;

                    match arg_token {
                        TokenEnum::StringLiteral(_) => asm.func_write_string(),

                        TokenEnum::Variable(var_name) => asm.func_write_var(var_name),

                        _ => {
                            // TODO: Fix this thing as anything other than Sting will have a nonsense
                            // get_token function
                            asm.func_write_number();
                        }
                    }
                }
            }

            FUNC_STRLEN => {
            }

            name => match f.borrow().get(name) {
                Some(function_ast) => {
                    println!("Visiting func {name}");
                    function_ast.visit_com(v, Rc::clone(&f), asm);
                }

                None => unimplemented!("Function {} unimplemented", self.name),
            },
        }
    }

    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>) -> VisitResult {
        match self.name.as_str() {
            FUNC_WRITE => {
                for arg in &self.arguments {
                    // println!("Visiting func write. Arg {:?}", arg);
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
