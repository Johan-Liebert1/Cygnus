use std::{cell::RefCell, rc::Rc};

use crate::{
    interpreter::interpreter::{Functions, Variables},
    lexer::tokens::TokenEnum,
};

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct AssignmentStatement {
    var_name: String,
    right: Rc<Box<dyn AST>>,
}

impl AssignmentStatement {
    pub fn new(var_name: String, right: Rc<Box<dyn AST>>) -> Self {
        Self { var_name, right }
    }
}

impl AST for AssignmentStatement {
    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>) -> VisitResult {
        let right_visit = self.right.visit(v, f);

        // TODO: change this so that the expression is stored here and we need to visit the varible
        // to evaluate the value
        if let TokenEnum::Number(n) = &*right_visit.token {
            v.insert(self.var_name.clone(), n.clone());

            return VisitResult {
                token: right_visit.token,
            };
        }

        panic!("Variable value is not a Number");
    }

    fn get_token(&self) -> &crate::lexer::lexer::Token {
        todo!()
    }

    fn print(&self) {
        println!("{:#?}", self)
    }
}
