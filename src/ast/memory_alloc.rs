use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    helpers,
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        tokens::{Number, TokenEnum},
        types::VarType,
    },
    semantic_analyzer::semantic_analyzer::CallStack,
};

use super::{
    abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut, VisitResult, AST},
    variable::Variable,
};

#[derive(Debug)]
pub struct MemoryAlloc {
    variable: Rc<RefCell<Variable>>,
    size: Rc<RefCell<Box<dyn AST>>>,
    pub result_type: VarType,
}

impl MemoryAlloc {
    pub fn new(variable: Rc<RefCell<Variable>>, size: Rc<RefCell<Box<dyn AST>>>) -> Self {
        Self {
            variable,
            size,
            result_type: VarType::Ptr(Box::new(VarType::Int)),
        }
    }
}

impl AST for MemoryAlloc {
    fn visit(&self, _: &mut Variables, _: Rc<RefCell<Functions>>, _: &mut CallStack) -> VisitResult {
        todo!()
    }

    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        call_stack.insert_variable(Rc::clone(&self.variable));

        // The size has to be known at compile time
        let result = self.size.borrow().visit(v, f, call_stack);

        let size = match *result.token {
            TokenEnum::Number(n) => match n {
                Number::Integer(i) => {
                    if i < 0 {
                        helpers::compiler_error(
                            "Memory size must be a positive integer",
                            self.size.borrow().get_token(),
                        );
                    } else {
                        i
                    }
                }

                Number::Float(_) => {
                    helpers::compiler_error(
                        "Memory to be allocated has to be an integer",
                        self.size.borrow().get_token(),
                    );
                }
            },

            _ => {
                helpers::compiler_error(
                    "Memory to be allocated has to be a number",
                    self.size.borrow().get_token(),
                );
            }
        };

        asm.generate_memory_alloc(&self.variable.borrow().var_name, size as usize)
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        call_stack.insert_variable(Rc::clone(&self.variable));

        self.size.borrow_mut().semantic_visit(call_stack, f);
    }

    fn get_token(&self) -> &crate::lexer::lexer::Token {
        todo!()
    }

    fn get_node(&self) -> ASTNodeEnum {
        todo!()
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        todo!()
    }

    fn print(&self) {
        todo!()
    }

    fn get_type(&self) -> (VarType, VarType) {
        return (
            self.result_type.get_actual_type(0, self.get_token()),
            self.result_type.clone(),
        );
    }
}
