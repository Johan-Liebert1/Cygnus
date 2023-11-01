use crate::{
    interpreter::interpreter::Variables,
    lexer::{lexer::Token, tokens::TokenEnum},
};

use super::{
    abstract_syntax_tree::{VisitResult, AST},
    variable::Variable,
};

#[derive(Debug)]
pub struct AssignmentStatement {
    left: Variable,
    right: Box<dyn AST>,
}

impl AssignmentStatement {
    pub fn new(left: Variable, right: Box<dyn AST>) -> Self {
        Self { left, right }
    }
}

impl AST for AssignmentStatement {
    fn visit(&self, vars: &mut Variables) -> VisitResult {
        let right_visit = self.right.visit(vars);

        println!("{:?}", right_visit);

        if let TokenEnum::Number(n) = &*right_visit.token {
            vars.insert(String::from(self.left.var_name.as_str()), n.clone());

            return VisitResult{ token: right_visit.token };
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
