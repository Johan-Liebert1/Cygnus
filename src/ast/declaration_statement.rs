use crate::lexer::types::VarType;
use crate::{helpers, trace};
use crate::{lexer::tokens::AssignmentTypes, types::ASTNode};

use crate::semantic_analyzer::semantic_analyzer::CallStack;

use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        lexer::Token,
        tokens::{TokenEnum, VariableEnum},
    },
};

use super::abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut};
use super::{
    abstract_syntax_tree::{VisitResult, AST},
    variable::Variable,
};

#[derive(Debug)]
pub struct DeclarationStatement {
    left: Rc<RefCell<Variable>>,
    right: ASTNode,
}

impl DeclarationStatement {
    pub fn new(left: Rc<RefCell<Variable>>, right: ASTNode) -> Self {
        Self { left, right }
    }

    fn verify_type(&self) {
        let node_borrow = self.right.borrow();
        let node = node_borrow.get_node();

        let (is_assignment_okay, rhs_type) = node.is_var_assignment_okay(&self.left.borrow());

        if !is_assignment_okay {
            trace!("Decleration statement: self.left: {:#?}", self.left);

            helpers::compiler_error(
                format!(
                    "Cannot assign variable (LHS) of type {} to RHS {}",
                    self.left.borrow().result_type,
                    rhs_type
                ),
                self.left.borrow().get_token(),
            );
        }
    }
}

impl AST for DeclarationStatement {
    fn visit_com(&self, vars: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        call_stack.insert_variable(Rc::clone(&self.left));

        asm.variable_declaration(&self.left.borrow().var_name, call_stack);

        self.right.borrow().visit_com(vars, f, asm, call_stack);

        let function_call_assign = if let ASTNodeEnum::FunctionCall(..) = self.right.borrow().get_node() {
            true
        } else {
            false
        };

        let borrow = self.right.borrow();

        let member_order = match borrow.get_node() {
            ASTNodeEnum::Struct(s) => {
                let binding = s.get_member_definition_order();
                Some(binding)
            }

            _ => None,
        };

        asm.variable_assignment(
            &self.left.borrow().var_name,
            &AssignmentTypes::Equals,
            call_stack,
            0,
            &None,
            member_order,
            &self.left.borrow(),
        );
    }

    fn visit(
        &self,
        vars: &mut Variables,
        functions: Rc<RefCell<Functions>>,
        call_stack: &mut CallStack,
    ) -> VisitResult {
        let right_visit = self.right.borrow().visit(vars, functions, call_stack);

        let var_name = String::from(self.left.borrow().var_name.as_str());

        // TODO: change this so that the expression is stored here and we need to visit the varible
        // to evaluate the value
        //
        match &*right_visit.token {
            TokenEnum::StringLiteral(s) => {
                vars.insert(var_name.clone(), VariableEnum::String(s.into()));
            }

            TokenEnum::Number(n) => {
                vars.insert(var_name.clone(), VariableEnum::Number(n.clone()));
            }

            TokenEnum::Variable(_) => todo!(),

            _ => {
                helpers::compiler_error(
                    "Variable value is not a Number, String or Variable",
                    self.left.borrow().get_token(),
                );
            }
        }

        return VisitResult {
            token: right_visit.token,
        };
    }

    fn get_token(&self) -> &Token {
        todo!()
    }

    fn print(&self) {
        println!("{:#?}", self);
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        self.right.borrow_mut().semantic_visit(call_stack, f.clone());

        // Before inserting in the call stack we need the type of the variable to calculate its
        // size
        {
            let left_mut = self.left.borrow_mut();
        }

        call_stack.insert_variable(Rc::clone(&self.left));

        self.left.borrow_mut().semantic_visit(call_stack, f);

        self.verify_type();
    }

    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::DeclarationStatement(&self);
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::DeclarationStatement(self);
    }

    fn get_type(&self) -> (VarType, VarType) {
        return (VarType::Unknown, VarType::Unknown);
    }
}
