use crate::{
    interpreter::interpreter::Variables,
    lexer::{keywords::FUNC_OUTPUT, lexer::Token, tokens::TokenEnum},
};

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct FunctionCall {
    name: String,
    arguments: Vec<String>,
}

impl FunctionCall {
    pub fn new(name: String, arguments: Vec<String>) -> Self {
        Self { name, arguments }
    }
}

impl AST for FunctionCall {
    fn visit(&self, _: &mut Variables) -> VisitResult {
        match self.name.as_str() {
            FUNC_OUTPUT => {
                for arg in &self.arguments {
                    print!("{}", arg);
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
