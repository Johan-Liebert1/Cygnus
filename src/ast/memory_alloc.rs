use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    semantic_analyzer::semantic_analyzer::CallStack, trace, lexer::tokens::{TokenEnum, Number},
};

use super::{
    abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut, VisitResult, AST},
    variable::Variable,
};

pub struct MemoryAlloc {
    variable: Variable,
    size: Rc<RefCell<Box<dyn AST>>>,
}

impl MemoryAlloc {
    pub fn new(variable: Variable, size: Rc<RefCell<Box<dyn AST>>>) -> Self {
        Self { variable, size }
    }
}

impl AST for MemoryAlloc {
    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
        todo!()
    }

    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        call_stack.insert_variable(self.variable.clone());

        // The size has to be known at compile time

        let result = self.size.borrow().visit(v, f, call_stack);

        let size = match *result.token {
            TokenEnum::Number(n) => match n {
                Number::Integer(i) => if i < 0 { panic!("Memory size must be a positive integer") } else { i },
                Number::Float(_) => panic!("Memory to be allocated has to be an integer"),
            },

            _ => {
                panic!("Memory to be allocated has to be a number")
            }
        };

        asm.generate_memory_alloc(&self.variable.var_name, size as usize)
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        call_stack.insert_variable(self.variable.clone());

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
}
