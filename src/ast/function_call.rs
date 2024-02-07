use crate::{types::ASTNode, trace};

use crate::semantic_analyzer::semantic_analyzer::CallStack;

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

use super::abstract_syntax_tree::{VisitResult, AST, ASTNodeEnum, ASTNodeEnumMut};

#[derive(Debug)]
pub struct FunctionCall {
    name: String,
    arguments: Vec<ASTNode>,
}

impl FunctionCall {
    pub fn new(name: String, arguments: Vec<ASTNode>) -> Self {
        Self { name, arguments }
    }
}

impl AST for FunctionCall {
    fn visit_com(
        &self,
        v: &mut Variables,
        f: Rc<RefCell<Functions>>,
        asm: &mut ASM,
        call_stack: &mut CallStack,
    ) {
        match self.name.as_str() {
            FUNC_WRITE => {
                for arg in &self.arguments {
                    // this will generate everything and put in rax
                    arg.borrow().visit_com(v, Rc::clone(&f), asm, call_stack);

                    let arg_borrow = arg.borrow();

                    let arg_token = &arg_borrow.get_token().token;

                    match arg_token {
                        TokenEnum::StringLiteral(_) => asm.func_write_string(),

                        TokenEnum::Variable(var_name) => asm.func_write_var(var_name, call_stack),

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
                    arg.borrow().visit_com(v, Rc::clone(&f), asm, call_stack);
                }

                asm.func_exit();
            }

            FUNC_STRLEN => {}

            name => match f.borrow().get(name) {
                Some(_function_ast) => {
                    asm.function_call(&String::from(name));
                }

                None => unimplemented!("Function {} unimplemented", self.name),
            },
        }
    }

    fn visit(
        &self,
        v: &mut Variables,
        f: Rc<RefCell<Functions>>,
        call_stack: &mut CallStack,
    ) -> VisitResult {
        match self.name.as_str() {
            FUNC_WRITE => {
                for arg in &self.arguments {
                    // println!("Visiting func write. Arg {:?}", arg);
                    println!("{:?}", arg.borrow().visit(v, Rc::clone(&f), call_stack));
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
                    // println!("{:?}", arg.visit(v, Rc::clone(&f)), call_stack);

                    let arg = arg.borrow().visit(v, Rc::clone(&f), call_stack);

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
                    function_ast.borrow().visit(v, Rc::clone(&f), call_stack)
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

    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        for arg in &self.arguments {
            arg.borrow_mut().semantic_visit(call_stack, Rc::clone(&f));
        }
    }

    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::FunctionCall(&self);
    }


    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::FunctionCall(self);
    }
}
