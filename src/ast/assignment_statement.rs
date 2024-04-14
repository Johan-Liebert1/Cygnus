use crate::helpers::compiler_error;
use crate::lexer::types::VarType;
use crate::{helpers, trace};
use crate::{lexer::tokens::AssignmentTypes, types::ASTNode};

use crate::semantic_analyzer::semantic_analyzer::CallStack;

use core::panic;
use std::process::exit;
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
            helpers::compiler_error(
                format!(
                    "Cannot assign variable (LHS) of type {} to RHS {}",
                    self.left.result_type, rhs_type
                ),
                self.left.get_token(),
            );
        }
    }
}

impl AST for AssignmentStatement {
    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        self.right.borrow().visit_com(v, f.clone(), asm, call_stack);

        // Only this is required for arrays. Not actually calling the visit_com function for the
        // variable as that messes up everything as it always assumes it's for printing and not for
        // assignment
        // We have to check var_type here as result_type actually contains integer
        if let VarType::Array(..) = self.left.var_type {
            if let Some(array_index) = &self.left.array_aceess_index {
                array_index.borrow().visit_com(v, f, asm, call_stack);
            }
        }

        asm.variable_assignment(
            &self.left.var_name,
            &self.assignment_type,
            call_stack,
            self.left.times_dereferenced,
            if let ASTNodeEnum::FunctionCall(..) = self.right.borrow().get_node() {
                true
            } else {
                false
            },
            &self.left.array_aceess_index,
            None,
            &self.left,
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
        self.right.borrow_mut().semantic_visit(call_stack, f.clone());
        self.left.semantic_visit(call_stack, f);

        if self.left.is_const {
            compiler_error(
                format!("Cannot reassign constant variable {}", self.left.var_name),
                &self.left.get_token(),
            );
        }

        self.verify_type();
    }

    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::AssignmentStatement(&self);
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::AssignmentStatement(self);
    }

    fn get_type(&self) -> (VarType, VarType) {
        return (VarType::Unknown, VarType::Unknown);
    }
}
