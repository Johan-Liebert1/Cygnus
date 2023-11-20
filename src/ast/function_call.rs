use crate::{
    interpreter::interpreter::{Variables, Functions},
    lexer::{keywords::FUNC_OUTPUT, lexer::Token, tokens::TokenEnum},
};

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct FunctionCall {
    name: String,
    arguments: Vec<Box<dyn AST>>,
}

impl FunctionCall {
    pub fn new(name: String, arguments: Vec<Box<dyn AST>>) -> Self {
        Self { name, arguments }
    }
}

impl AST for FunctionCall {
    fn visit(&self, i: &mut Variables, f: &mut Functions) -> VisitResult {
        match self.name.as_str() {
            FUNC_OUTPUT => {
                for arg in &self.arguments {
                    println!("{:?} {:?}", arg, arg.visit(i, f));
                }

                return VisitResult {
                    token: Box::new(TokenEnum::Unknown("".into())),
                };
            }

            _ => {
                unimplemented!("Function {} unimplemented", self.name)
            }
        }
    }

    fn get_token(&self) -> &Token {
        todo!()
    }

    fn print(&self) {
        println!("{:?}", &self);
    }
}
