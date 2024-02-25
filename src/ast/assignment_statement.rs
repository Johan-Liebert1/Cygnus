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
    left: Variable,
    assignment_type: AssignmentTypes,
    right: ASTNode,
}

impl AssignmentStatement {
    pub fn new(left: Variable, assignment_type: AssignmentTypes, right: ASTNode) -> Self {
        Self {
            left,
            assignment_type,
            right,
        }
    }

    fn verify_type(&self) {
        let node_borrow = self.right.borrow();
        let node = node_borrow.get_node();

        let (is_assignment_okay, rhs_type) = node.is_var_assignment_okay(&self.left);

        if !is_assignment_okay {
            panic!(
                "Cannot assign variable (LHS) of type {} to RHS {} at {}:{}",
                self.left.result_type,
                rhs_type,
                self.left.get_token().line_number,
                self.left.get_token().col_number
            )
        }
    }
}

impl AST for AssignmentStatement {
    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        self.right.borrow().visit_com(v, f, asm, call_stack);
        asm.variable_assignment(
            &self.left.var_name,
            &self.assignment_type,
            call_stack,
            self.left.times_dereferenced,
        );
    }

    // TODO: change this so that the expression is stored here and we need to visit the varible
    // to evaluate the value
    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
        let right_visit = self.right.borrow().visit(v, f, call_stack);

        match &*right_visit.token {
            TokenEnum::StringLiteral(s) => {
                v.insert(self.left.var_name.clone(), VariableEnum::String(s.into()));
            }

            TokenEnum::Number(n) => {
                v.insert(self.left.var_name.clone(), VariableEnum::Number(n.clone()));
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

        let (variable_opt, _) = call_stack.get_var_with_name(&self.left.var_name);

        if let Some(variable) = variable_opt {
            // the variable in here has type unknown, we have to fill it in here

            self.left.var_type = variable.var_type.clone();

            // not passing &self.left.var_type because borrow checker cries
            self.left
                .store_result_type(&variable.var_type, self.left.times_dereferenced);

            self.verify_type();
        } else {
            panic!("Variable '{}' not found in current scope", &self.left.var_name);
        }
    }

    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::AssignmentStatement(&self);
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::AssignmentStatement(self);
    }
}
