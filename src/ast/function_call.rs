use crate::semantic_analyzer::semantic_analyzer::{
    ActivationRecord, ActivationRecordType, CallStack,
};

use std::{cell::RefCell, process::exit, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        keywords::{FUNC_EXIT, FUNC_STRLEN, FUNC_WRITE},
        lexer::Token,
        tokens::{Number, TokenEnum},
    },
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

                        TokenEnum::Variable(var_name) => asm.func_write_var(var_name, v),

                        _ => {
                            // TODO: Fix this thing as anything other than Sting will have a nonsense
                            // get_token function
                            asm.func_write_number();
                        }
                    }
                }
            }

            FUNC_EXIT => {
                if self.arguments.len() == 0 {
                    panic!("exit needs one argument");
                }

                for arg in &self.arguments {
                    arg.visit_com(v, Rc::clone(&f), asm);
                }

                asm.func_exit();
            }

            FUNC_STRLEN => {}

            name => match f.borrow().get(name) {
                Some(function_ast) => {
                    println!("Visiting func {name}");
                    asm.function_call(&String::from(name));
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

            FUNC_EXIT => {
                if self.arguments.len() == 0 {
                    panic!("exit needs one argument");
                }

                for arg in &self.arguments {
                    // println!("Visiting func write. Arg {:?}", arg);
                    // println!("{:?}", arg.visit(v, Rc::clone(&f)));

                    let arg = arg.visit(v, Rc::clone(&f));

                    match *arg.token {
                        TokenEnum::Number(n) => match n {
                            Number::Integer(i) => exit(i),
                            Number::Float(_) => {
                                panic!("exit needs an integer argument. Received float")
                            }
                        },

                        t => {
                            panic!("exit needs an integer argument. Received {:?}", t);
                        }
                    }
                }

                exit(1);
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

    fn semantic_visit(&self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        for arg in &self.arguments {
            arg.semantic_visit(call_stack, Rc::clone(&f));
        }
    }
}
