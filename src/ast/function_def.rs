use crate::helpers::compiler_error;
use crate::lexer::types::VarType;
use crate::trace;
use crate::types::ASTNode;

use crate::semantic_analyzer::semantic_analyzer::{ActivationRecord, ActivationRecordType, CallStack};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        lexer::Token,
        tokens::{Number, TokenEnum, VariableEnum},
    },
};
use std::{cell::RefCell, rc::Rc};

use super::abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut};
use super::jump::JumpType;
use super::{
    abstract_syntax_tree::{VisitResult, AST},
    variable::Variable,
};

#[derive(Debug)]
pub struct FunctionDefinition {
    name: String,
    pub parameters: Vec<Rc<RefCell<Variable>>>,
    block: ASTNode,
    /// How much to allocate on the stack to make room for local variables
    stack_var_size: usize,
    pub return_type: VarType,
    token: Token,
    pub is_extern_func: bool,
}

impl FunctionDefinition {
    pub fn new(
        name: String,
        arguments: Vec<Rc<RefCell<Variable>>>,
        block: ASTNode,
        return_type: VarType,
        token: Token,
        is_extern_func: bool,
    ) -> Self {
        Self {
            name,
            parameters: arguments,
            block,
            stack_var_size: 0,
            return_type,
            token,
            is_extern_func,
        }
    }

    fn return_type_error(&self) {
        compiler_error(
            format!(
                "Function '{}' needs to return '{}' but nothing was returned",
                self.name, self.return_type
            ),
            &self.token,
        );
    }

    fn visit_com_external_func(&self, asm: &mut ASM) {
        asm.extern_function_def(&self.name);
    }
}

impl AST for FunctionDefinition {
    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        if self.is_extern_func {
            self.visit_com_external_func(asm);
            return;
        }

        call_stack.push(
            self.name.to_string(),
            // TODO: This used to be  ActivationRecordType::Function(self.stack_var_size)
            // but 0 also works
            ActivationRecordType::Function(0),
        );

        for arg in &self.parameters {
            // params cannot be dereferenced
            call_stack.insert_variable(Rc::clone(arg));
        }

        // args -> rax, rdi, rsi, rdx, r10, r8, r9
        asm.function_def(call_stack, &self.name, self.stack_var_size, &self.parameters);

        self.block.borrow().visit_com(v, f, asm, call_stack);

        // pop the record here
        call_stack.pop();

        asm.function_def_end(&self.name);
    }

    // TODO: This function will be visited twice, once when the interpreter calls visit, and
    // another when the function is actually called
    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
        // TODO: handle global variables and function parameters with the same name
        for param in &self.parameters {
            let value = match param.borrow().var_type {
                VarType::Int => todo!(),
                VarType::Int8 => todo!(),
                VarType::Int16 => todo!(),
                VarType::Int32 => todo!(),
                VarType::Array(..) => todo!(),
                VarType::Str => todo!(),
                VarType::Float => todo!(),
                VarType::Ptr(_) => todo!(),
                VarType::Unknown => todo!(),
                VarType::Char => todo!(),
                VarType::Struct(_, _) => todo!(),
                VarType::Function(_, _, _) => todo!(),
            };

            v.insert(param.borrow().var_name.clone(), VariableEnum::Number(value));
        }

        self.block.borrow().visit(v, f, call_stack);

        for param in &self.parameters {
            v.remove(&param.borrow().var_name.clone());
        }

        return VisitResult {
            token: Box::new(TokenEnum::Unknown("".to_string())),
        };
    }

    fn get_token(&self) -> &Token {
        &self.token
    }

    fn print(&self) {
        println!("{:?}", &self);
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        if self.is_extern_func {
            return
        }

        call_stack.push(self.name.to_string(), ActivationRecordType::Function(0));

        for arg in &self.parameters {
            call_stack.insert_variable(Rc::clone(arg));
        }

        self.block.borrow_mut().semantic_visit(call_stack, Rc::clone(&f));

        self.stack_var_size = call_stack.get_func_var_stack_size(&self.name);

        // In the AST of return statement we check the type of the returned value
        // But here we also need to check whether the function even returns something

        if !matches!(self.return_type, VarType::Unknown) {
            // The very last statement needs to be a return statement
            // Else, nothing is being returned from a function that doesn't have void return type

            match self.block.borrow().get_node() {
                ASTNodeEnum::Program(program) => {
                    match program.get_statements().last() {
                        Some(last_statement) => match last_statement.borrow().get_node() {
                            ASTNodeEnum::Jump(jump) => match jump.typ {
                                JumpType::Return => {}

                                JumpType::Break => self.return_type_error(),
                            },

                            _ => self.return_type_error(),
                        },

                        None => {
                            // function has a return type but is empty
                            self.return_type_error();
                        }
                    }
                }

                t => {
                    unreachable!("Block inside function was of type {}, where 'Program' was expected.", t)
                }
            };
        }

        // pop the record here
        call_stack.pop();
    }

    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::FunctionDef(&self);
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::FunctionDef(self);
    }

    fn get_type(&self) -> (VarType, VarType) {
        return (VarType::Unknown, VarType::Unknown);
    }
}
