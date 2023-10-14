use std::fmt::{Debug, Display};

use crate::lexer::Token;

pub trait AST {
    fn visit(&mut self);
    fn get_token(&self) -> &Token;
}

impl Debug for dyn AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.get_token())
    }
}

impl Display for dyn AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.get_token())
    }
}
