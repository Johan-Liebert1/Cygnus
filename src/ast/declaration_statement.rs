use std::{cell::RefCell, rc::Rc};

use crate::{
    interpreter::interpreter::{Functions, Variables},
    lexer::{lexer::Token, tokens::TokenEnum},
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
    fn visit(&self, vars: &mut Variables, functions: Rc<RefCell<Functions>>) -> VisitResult {
        let right_visit = self.right.visit(vars, functions);

        // TODO: change this so that the expression is stored here and we need to visit the varible
        // to evaluate the value
        if let TokenEnum::Number(n) = &*right_visit.token {
            vars.insert(String::from(self.left.var_name.as_str()), n.clone());

            return VisitResult {
                token: right_visit.token,
            };
        }

        panic!("Variable value is not a Number");
    }

    fn get_token(&self) -> &Token {
        todo!()
    }

    fn print(&self) {
        println!("{:#?}", self);
    }
}
