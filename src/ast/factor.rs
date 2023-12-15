use crate::{
    constants,
    interpreter::interpreter::{Functions, Variables},
    lexer::{lexer::Token, tokens::TokenEnum}, asm::asm::ASM,
};
use std::{cell::RefCell, rc::Rc};

use super::abstract_syntax_tree::{VisitResult, AST};

/// FACTOR -> INTEGER | FLOAT
#[derive(Debug)]
pub struct Factor {
    token: Box<Token>,
}

impl Factor {
    pub fn new(token: Box<Token>) -> Self {
        Self { token }
    }
}

impl AST for Factor {
    fn visit_com(&self, x: &mut Variables, _: Rc<RefCell<Functions>>, asm: &mut ASM) {
        asm.generate_asm_factor(&self.token.token);
    }

    fn visit(&self, v: &mut Variables, _: Rc<RefCell<Functions>>) -> VisitResult {
        if constants::DEBUG_AST {
            println!("{:?}", &self);
        }

        let token_enum = match &self.token.token {
            TokenEnum::Variable(var_name) => {
                if let Some(n) = v.get(var_name) {
                    TokenEnum::Number(n.clone())
                } else {
                    panic!("Variable {var_name} not defined");
                }
            }

            t => t.clone(),
        };

        VisitResult {
            token: Box::new(token_enum),
        }
    }

    fn get_token(&self) -> &Token {
        return &self.token;
    }

    fn print(&self) {
        println!("{:#?}", self);
    }
}

// fn thing() -> Box<Rc<dyn AST>> {
//     return Box::new(Factor::new(Box::new(Token {
//         token: crate::lexer::tokens::TokenEnum::EOF,
//         line_number: 1,
//         col_number: 1,
//     })));
// }
