use crate::ast::abstract_syntax_tree::AST;

use super::parser::Parser;

impl<'a> Parser<'a> {
    pub fn parse_conditionals(&mut self) -> Box<dyn AST> {
        todo!()
    }
}
