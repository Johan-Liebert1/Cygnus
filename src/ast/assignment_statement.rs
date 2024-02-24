use crate::trace;
use crate::{lexer::tokens::AssignmentTypes, types::ASTNode};

use crate::semantic_analyzer::semantic_analyzer::CallStack;

use core::panic;
use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::tokens::{TokenEnum, VariableEnum},
};

use super::abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut, VisitResult, AST};
use super::variable::Variable;

#[derive(Debug)]
pub struct AssignmentStatement {
    var_name: String,
    assignment_type: AssignmentTypes,
    right: ASTNode,
    times_dereferenced: usize,
}

impl AssignmentStatement {
    pub fn new(var_name: String, assignment_type: AssignmentTypes, right: ASTNode, times_dereferenced: usize) -> Self {
        Self {
            var_name,
            assignment_type,
            right,
            times_dereferenced,
        }
    }

    fn verify_type(&self, variable: &Variable) {
        let node_borrow = self.right.borrow();
        let node = node_borrow.get_node();

        let (is_assignment_okay, rhs_type) = node.is_var_assignment_okay(variable);

        if !is_assignment_okay {
            panic!(
                "Cannot assign variable (LHS) of type {} to RHS {}",
                variable.result_type, rhs_type
            )
        }
    }
}

impl AST for AssignmentStatement {
    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        self.right.borrow().visit_com(v, f, asm, call_stack);
        asm.variable_assignment(
            &self.var_name,
            &self.assignment_type,
            call_stack,
            self.times_dereferenced,
        );
    }

    // TODO: change this so that the expression is stored here and we need to visit the varible
    // to evaluate the value
    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
        let right_visit = self.right.borrow().visit(v, f, call_stack);

        match &*right_visit.token {
            TokenEnum::StringLiteral(s) => {
                v.insert(self.var_name.clone(), VariableEnum::String(s.into()));
            }

            TokenEnum::Number(n) => {
                v.insert(self.var_name.clone(), VariableEnum::Number(n.clone()));
            }

            TokenEnum::Variable(_) => todo!(),

            _ => {
                panic!("Variable value is not a Number, String or Variable");
            }
        }

        return VisitResult {
            token: right_visit.token,
        };
    }

    fn get_token(&self) -> &crate::lexer::lexer::Token {
        todo!()
    }

    fn print(&self) {
        println!("{:#?}", self)
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        self.right.borrow_mut().semantic_visit(call_stack, f);

        let (variable_opt, _) = call_stack.get_var_with_name(&self.var_name);

        if let Some(variable) = variable_opt {
            self.verify_type(variable);
        } else {
            panic!("Variable '{}' not found in current scope", &self.var_name);
        }
    }

    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::AssignmentStatement(&self);
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::AssignmentStatement(self);
    }
}
