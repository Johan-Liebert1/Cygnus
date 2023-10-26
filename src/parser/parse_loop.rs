use crate::{
    ast::{abstract_syntax_tree::AST, ast_loop::Loop},
    lexer::{
        keywords::{FROM, TO},
        tokens::{Bracket, TokenEnum},
    },
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    // TODO: Also consider step keyword
    // 
    /// LOOP -> loop from LPAREN* EXPRESSION to EXPRESSION RPAREN* (with VAR_NAME)* LCURLY STATEMENT[] RCURLY
    pub fn parse_loop(&mut self) -> Box<dyn AST> {
        // we get here after consuming the 'loop' keyword

        self.validate_token(TokenEnum::Keyword(FROM.to_string()));

        let from_range = match self.peek_next_token().token {
            TokenEnum::Bracket(..) => {
                // if there is a bracket, it has to be a left paren
                self.validate_token(TokenEnum::Bracket(Bracket::LParen));
                let exp = self.parse_expression();
                self.validate_token(TokenEnum::Bracket(Bracket::RParen));

                exp
            }

            _ => self.parse_expression(),
        };

        self.validate_token(TokenEnum::Keyword(TO.to_string()));

        let to_range = match self.peek_next_token().token {
            TokenEnum::Bracket(..) => {
                // if there is a bracket, it has to be a left paren
                self.validate_token(TokenEnum::Bracket(Bracket::LParen));
                let exp = self.parse_expression();
                self.validate_token(TokenEnum::Bracket(Bracket::RParen));

                exp
            }

            _ => self.parse_expression(),
        };

        // TODO: Handle (with VAR_NAME)* here

        self.validate_token(TokenEnum::Bracket(Bracket::LCurly));
        let block = self.parse_statements();
        self.validate_token(TokenEnum::Bracket(Bracket::RCurly));

        return Box::new(Loop::new(from_range, to_range, block));
    }
}
