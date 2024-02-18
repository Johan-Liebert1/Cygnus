use crate::trace;
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
    left: Variable,
    right: ASTNode,
}

impl DeclarationStatement {
    pub fn new(left: Variable, right: ASTNode) -> Self {
        Self { left, right }
    }
}

impl AST for DeclarationStatement {
    fn visit_com(&self, vars: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        call_stack.insert_variable(self.left.clone());

        asm.variable_declaration(&self.left.var_name, call_stack);

        self.right.borrow().visit_com(vars, f, asm, call_stack);

        asm.variable_assignment(&self.left.var_name, &AssignmentTypes::Equals, call_stack);
    }

    fn visit(
        &self,
        vars: &mut Variables,
        functions: Rc<RefCell<Functions>>,
        call_stack: &mut CallStack,
    ) -> VisitResult {
        let right_visit = self.right.borrow().visit(vars, functions, call_stack);

        let var_name = String::from(self.left.var_name.as_str());

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
                panic!("Variable value is not a Number, String or Variable");
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

    fn semantic_visit(&mut self, call_stack: &mut CallStack, _f: Rc<RefCell<Functions>>) {
        call_stack.insert_variable(self.left.clone());
    }

    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::DeclarationStatement(&self);
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::DeclarationStatement(self);
    }
}
