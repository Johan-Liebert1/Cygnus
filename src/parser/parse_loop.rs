use crate::types::ASTNode;

use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{abstract_syntax_tree::AST, ast_loop::Loop, factor::Factor},
    lexer::{
        keywords::{FROM, STEP, TO},
        lexer::Token,
        tokens::{Bracket, Number, TokenEnum},
    },
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /// LOOP -> loop from LPAREN* EXPRESSION to EXPRESSION (step EXPRESSION)* RPAREN* (with VAR_NAME)* LCURLY STATEMENT[] RCURLY
    pub fn parse_loop(&mut self) -> ASTNode {
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

        let default_step: ASTNode = Rc::new(RefCell::new(Box::new(Factor::new(Box::new(Token {
            token: TokenEnum::Number(Number::Integer(1)),
            line_number: 0,
            col_number: 0,
        })))));

        let step = match self.peek_next_token().token {
            TokenEnum::Keyword(keyword) => {
                match keyword.as_str() {
                    STEP => {
                        // consume 'step'
                        self.get_next_token();

                        self.parse_expression()
                    }

                    _ => default_step,
                }
            }

            _ => default_step,
        };

        // TODO: Handle (with VAR_NAME)* here
        self.validate_token(TokenEnum::Bracket(Bracket::LCurly));

        self.inside_loop_depth += 1;
        let block = self.parse_program();
        self.inside_loop_depth -= 1;

        self.validate_token(TokenEnum::Bracket(Bracket::RCurly));

        return Rc::new(RefCell::new(Box::new(Loop::new(
            from_range, to_range, step, block,
        ))));
    }
}
