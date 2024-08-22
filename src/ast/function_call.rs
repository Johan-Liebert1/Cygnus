use crate::asm::functions::FUNCTION_ARGS_REGS;
use crate::ast::function_def::FunctionDefinition;
use crate::helpers::{self, compiler_error};
use crate::lexer::keywords::FUNC_SYSCALL;
use crate::lexer::types::VarType;
use crate::{trace, types::ASTNode};

use crate::semantic_analyzer::semantic_analyzer::{ActivationRecordType, CallStack};

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

use super::abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut, VisitResult, AST};

#[derive(Debug)]
pub struct FunctionCall {
    name: String,
    token: Token,
    arguments: Vec<ASTNode>,
    /// This is basically the return type for this function
    pub result_type: VarType,
}

impl FunctionCall {
    pub fn new(name: String, token: Token, arguments: Vec<ASTNode>) -> Self {
        Self {
            name,
            token,
            arguments,
            result_type: VarType::Unknown,
        }
    }
}

impl AST for FunctionCall {
    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        match self.name.as_str() {
            FUNC_WRITE => {
                for arg in &self.arguments {
                    // this will generate everything and put in rax
                    arg.borrow().visit_com(v, Rc::clone(&f), asm, call_stack);

                    match arg.borrow().get_node() {
                        ASTNodeEnum::Variable(v) => {
                            println!("token: {:?}", arg.borrow().get_token());
                            asm.func_write_var(v, call_stack);
                        }

                        ASTNodeEnum::BinaryOp(bo) => match &bo.result_type {
                            VarType::Int => asm.func_write_number(),

                            VarType::Str => asm.func_write_string(),

                            VarType::Float => asm.func_write_number(),

                            VarType::Ptr(ptr_type) => {
                                asm.func_write_pointer(&ptr_type, bo.times_dereferenced, &call_stack, None)
                            }

                            VarType::Int8 => todo!(),
                            VarType::Int16 => todo!(),
                            VarType::Int32 => todo!(),

                            VarType::Char => todo!(),

                            VarType::Array(..) => todo!(),
                            VarType::Struct(_, _) => todo!(),
                            VarType::Unknown => todo!(),
                            VarType::Function(_, _, _) => todo!(),
                        },

                        ASTNodeEnum::Factor(f) => match &f.get_token().token {
                            TokenEnum::Number(_) => asm.func_write_number(),
                            TokenEnum::StringLiteral(_) => asm.func_write_string(),

                            tok => unreachable!("This should be unreachable"),
                        },

                        // This will always be an integer
                        ASTNodeEnum::LogicalExp(lo) => asm.func_write_number(),

                        // This will always be an integer
                        ASTNodeEnum::ComparisonExp(..) => asm.func_write_number(),

                        ASTNodeEnum::FunctionCall(fc) => {
                            // if the function returns anything, then that will be in rax

                            let borrow = f.borrow();
                            // this will most definitely exist here
                            let func_def = borrow.get(&fc.name).unwrap();

                            match func_def.return_type {
                                VarType::Int | VarType::Int8 | VarType::Int16 | VarType::Int32 => {
                                    asm.func_write_number()
                                }
                                _ => unimplemented!(),
                            };
                        }

                        node => {
                            trace!("{:#?}", node);
                            todo!();
                        }
                    };
                }
            }

            FUNC_EXIT => {
                for arg in &self.arguments {
                    arg.borrow().visit_com(v, Rc::clone(&f), asm, call_stack);
                }

                asm.func_exit();
            }

            FUNC_SYSCALL => {
                for arg in self.arguments.iter().rev() {
                    arg.borrow().visit_com(v, Rc::clone(&f), asm, call_stack);
                }

                asm.func_syscall(self.arguments.len());
            }

            name => match f.borrow().get(name) {
                // args -> rax, rdi, rsi, rdx, r10, r8, r9
                Some(func) => {
                    // we reverse here as we want to push into the stack backwards
                    for argument in self.arguments.iter().rev() {
                        argument.borrow().visit_com(v, f.clone(), asm, call_stack);
                    }

                    asm.function_call(&String::from(name), self.arguments.len(), &func.return_type);
                }

                None => compiler_error(format!("Function {} unimplemented", self.name), &self.token),
            },
        }
    }

    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
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
                    helpers::compiler_error(format!("exit needs one argument"), &self.token);
                }

                for arg in &self.arguments {
                    // println!("Visiting func write. Arg {:?}", arg);
                    // println!("{:?}", arg.visit(v, Rc::clone(&f)), call_stack);

                    let arg = arg.borrow().visit(v, Rc::clone(&f), call_stack);

                    match *arg.token {
                        TokenEnum::Number(n) => match n {
                            Number::Integer(i) => exit(i),
                            Number::Float(_) => {
                                helpers::compiler_error(
                                    format!("exit needs an integer argument. Received float"),
                                    &self.token,
                                );
                            }
                        },

                        t => {
                            helpers::compiler_error(
                                format!("exit needs an integer argument. Received {:?}", t),
                                &self.token,
                            );
                        }
                    }
                }

                exit(1);
            }

            name => match f.borrow().get(name) {
                Some(function_ast) => function_ast.func.borrow().visit(v, Rc::clone(&f), call_stack),

                None => unimplemented!("Function {} unimplemented", self.name),
            },
        }
    }

    fn get_token(&self) -> &Token {
        return &self.token;
    }

    fn print(&self) {
        println!("{:?}", &self);
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        // need to do this first to compute resulting types
        for arg in &self.arguments {
            arg.borrow_mut().semantic_visit(call_stack, Rc::clone(&f));
        }

        let binding = f.borrow();

        match self.name.as_str() {
            FUNC_WRITE => {}
            FUNC_EXIT => {}

            FUNC_SYSCALL => {
                // not type checked
                self.result_type = VarType::Int
            }

            _ => {
                if self.arguments.len() > FUNCTION_ARGS_REGS.len() {
                    todo!("Functions with more than {} args not handled", FUNCTION_ARGS_REGS.len())
                }

                let function_definition = match binding.get(&self.name) {
                    Some(f) => f,
                    None => {
                        compiler_error(format!("Function '{}' is not defined", &self.name), &self.token);
                        exit(1);
                    }
                };

                if let ASTNodeEnum::FunctionDef(fd) = function_definition.func.borrow().get_node() {
                    if fd.parameters.len() != self.arguments.len() {
                        compiler_error(
                            format!(
                                "Function '{}' expects {} arguments but got {}",
                                &self.name,
                                fd.parameters.len(),
                                self.arguments.len()
                            ),
                            &self.token,
                        );

                        exit(1);
                    }

                    self.result_type = fd.return_type.clone();

                    for (actual_param, formal_param) in fd.parameters.iter().zip(&self.arguments) {
                        let binding = formal_param.borrow();
                        let binding = binding.get_node();

                        let borrowed_actual_param = actual_param.borrow();

                        let (is_var_assignment_okay, rhs_type) = binding.is_var_assignment_okay(&borrowed_actual_param);

                        if !is_var_assignment_okay {
                            compiler_error(
                                format!(
                                    "Cannot assign param of type {} to '{}', as '{}' is defined as type {}",
                                    rhs_type,
                                    borrowed_actual_param.var_name,
                                    borrowed_actual_param.var_name,
                                    borrowed_actual_param.result_type
                                ),
                                &self.token,
                            )
                        }
                    }
                } else {
                    unreachable!("Found non function_definition node inside functions hash map")
                }
            }
        };
    }

    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::FunctionCall(&self);
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::FunctionCall(self);
    }

    fn get_type(&self) -> (VarType, VarType) {
        return (
            self.result_type.get_actual_type(0, &self.token),
            self.result_type.clone(),
        );
    }
}
