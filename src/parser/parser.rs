use crate::{
    ast::abstract_syntax_tree::AST,
    lexer::{Lexer, Token},
    tokens::{Bracket, TokenEnum},
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

    /// STATEMENT -> VARIABLE_DECLARATION | COMPARISON_EXPRESSION | LPAREN COMPARISON_EXPRESSION RPAREN
    pub fn parse_statements(&mut self) -> Box<dyn AST> {
        let current_token = self.peek_next_token();

        match current_token.token {
            TokenEnum::Keyword(_) => {
                self.get_next_token();
                self.parse_assignment_statement()
            },

            TokenEnum::Number(_) => todo!(),
            TokenEnum::Op(_) => todo!(),
            TokenEnum::Equals => todo!(),
            TokenEnum::Bracket(_) => todo!(),
            TokenEnum::Comparator(_) => todo!(),
            TokenEnum::Bool(_) => todo!(),
            TokenEnum::Variable(_) => todo!(),
            TokenEnum::Unknown => todo!(),
            TokenEnum::EOF => todo!(),
        }
    }

    pub fn parse(&mut self) -> Box<dyn AST> {
        return self.parse_statements();
    }
}
