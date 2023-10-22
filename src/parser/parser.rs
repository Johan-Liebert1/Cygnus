use crate::{
    ast::abstract_syntax_tree::AST,
    lexer::{Lexer, Token},
    tokens::Bracket,
};

pub struct Parser<'a> {
    pub lexer: Box<Lexer<'a>>,
    parsed_tokens: Vec<Token>,
    pub bracket_stack: Vec<Bracket>,
}

impl<'a> Parser<'a> {
    pub fn new(file: &'a Vec<u8>) -> Self {
        let parser = Lexer::new(file);

        Self {
            lexer: Box::new(parser),
            parsed_tokens: vec![],
            bracket_stack: vec![],
        }
    }

    /// STATEMENT -> EXPRESSION
    pub fn parse_statements(&mut self) -> Box<dyn AST> {
        return self.parse_comparison_expression();
    }

    pub fn parse(&mut self) -> Box<dyn AST> {
        return self.parse_statements();
    }
}
