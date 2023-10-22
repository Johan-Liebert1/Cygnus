use std::fmt::{Debug, Display};

use crate::{lexer::Token, tokens::TokenEnum};

#[derive(Debug)]
pub struct VisitResult {
    pub token: Box<TokenEnum>,
}

pub trait AST {
    fn visit(&self) -> VisitResult;
    fn get_token(&self) -> &Token;
    fn print(&self);
}

impl Debug for dyn AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.print())
    }
}

impl Display for dyn AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.get_token())
    }
}
