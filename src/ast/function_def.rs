use crate::lexer::types::VarType;
use crate::trace;
use crate::types::ASTNode;

use crate::semantic_analyzer::semantic_analyzer::{
    ActivationRecord, ActivationRecordType, CallStack,
};

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
use super::{
    abstract_syntax_tree::{VisitResult, AST},
    variable::Variable,
};

#[derive(Debug)]
pub struct FunctionDefinition {
    name: String,
    parameters: Vec<Variable>,
    block: ASTNode,
    /// How much to allocate on the stack to make room for local variables
    stack_var_size: usize,
}

impl FunctionDefinition {
    pub fn new(name: String, arguments: Vec<Variable>, block: ASTNode) -> Self {
        Self {
            name,
            parameters: arguments,
            block,
            stack_var_size: 0,
        }
    }
}

impl AST for FunctionDefinition {
    fn visit_com(
        &self,
        v: &mut Variables,
        f: Rc<RefCell<Functions>>,
        asm: &mut ASM,
        call_stack: &mut CallStack,
    ) {
        asm.function_def(&self.name, self.stack_var_size);

        call_stack.push(self.name.to_string(), ActivationRecordType::Function);

        for arg in &self.parameters {
            // params cannot be dereferenced
            call_stack.insert_variable(arg.clone());
        }

        self.block.borrow().visit_com(v, f, asm, call_stack);

        // pop the record here
        call_stack.pop();

        asm.function_def_end(&self.name);
    }

    // TODO: This function will be visited twice, once when the interpreter calls visit, and
    // another when the function is actually called
    fn visit(
        &self,
        v: &mut Variables,
        f: Rc<RefCell<Functions>>,
        call_stack: &mut CallStack,
    ) -> VisitResult {
        // TODO: handle global variables and function parameters with the same name
        for param in &self.parameters {
            let value = match param.var_type {
                VarType::Int => todo!(),
                VarType::Str => todo!(),
                VarType::Float => todo!(),
                VarType::Ptr(_) => todo!(),
                VarType::Unknown => todo!(),
                VarType::Char => todo!(),
            };

            v.insert(param.var_name.clone(), VariableEnum::Number(value));
        }

        self.block.borrow().visit(v, f, call_stack);

        for param in &self.parameters {
            v.remove(&param.var_name.clone());
        }

        return VisitResult {
            token: Box::new(TokenEnum::Unknown("".to_string())),
        };
    }

    fn get_token(&self) -> &Token {
        todo!()
    }

    fn print(&self) {
        println!("{:?}", &self);
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        call_stack.push(self.name.to_string(), ActivationRecordType::Function);

        for arg in &self.parameters {
            call_stack.insert_variable(arg.clone());
        }

        self.block
            .borrow_mut()
            .semantic_visit(call_stack, Rc::clone(&f));

        self.stack_var_size = call_stack.get_func_var_stack_size(&self.name);

        // pop the record here
        call_stack.pop();
    }

    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::FunctionDef(&self);
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::FunctionDef(self);
    }
}
