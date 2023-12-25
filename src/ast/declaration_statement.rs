use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        lexer::Token,
        tokens::{TokenEnum, VariableEnum},
    },
};

use super::{
    abstract_syntax_tree::{VisitResult, AST},
    variable::Variable,
};

#[derive(Debug)]
pub struct DeclarationStatement {
    left: Variable,
    right: Rc<Box<dyn AST>>,
}

impl DeclarationStatement {
    pub fn new(left: Variable, right: Rc<Box<dyn AST>>) -> Self {
        Self { left, right }
    }
}

impl AST for DeclarationStatement {
    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM) {
        asm.variable_declaration(&self.left.var_name);
        self.right.visit_com(v, f, asm);
        asm.variable_assignment(&self.left.var_name);
    }

    fn visit(&self, vars: &mut Variables, functions: Rc<RefCell<Functions>>) -> VisitResult {
        let right_visit = self.right.visit(vars, functions);

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
}
