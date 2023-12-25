use std::rc::Rc;

use crate::{
    ast::{
        abstract_syntax_tree::AST,
        conditionals::{ConditionalStatement, ElseStatement, IfStatement},
    },
    lexer::{
        keywords::{ELIF_STATEMENT, ELSE_STATEMENT},
        tokens::{Bracket, TokenEnum},
    },
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /// CONDITIONAL_STATEMENT -> if LPAREN* COMPARISON_EXPRESSION RPAREN* LCURLY STATEMENT[]* RCURLY ELSE_STATEMENT*
    pub fn parse_conditional_statement(&mut self) -> Rc<Box<dyn AST>> {
        // we get here after 'if' has been consumed

        // parse the if statements
        let if_statement = self.parse_if_elif();

        let mut elif_ladder: Vec<IfStatement> = vec![];

        // parse the elif ladder
        loop {
            let token = self.peek_next_token();

            if let TokenEnum::Keyword(keyword) = token.token {
                if keyword == ELIF_STATEMENT {
                    self.get_next_token();

                    elif_ladder.push(self.parse_if_elif());
                    continue;
                }

                break;
            }

            break;
        }

        // parse the final else, if any
        let token = self.peek_next_token();

        let mut else_statement: Option<ElseStatement> = None;

        if let TokenEnum::Keyword(keyword) = token.token {
            if keyword == ELSE_STATEMENT {
                self.get_next_token();

                else_statement = Some(self.parse_else());
            }
        }

        return Rc::new(Box::new(ConditionalStatement::new(
            if_statement,
            elif_ladder,
            else_statement,
        )));
    }

    /// we get here after 'if' has been consumed
    pub fn parse_if_elif(&mut self) -> IfStatement {
        // Parse if statements and any and all elif and else statements
        // store them all in one AST
        let condition = self.parse_comparison_expression();

        match self.peek_next_token().token {
            TokenEnum::Bracket(b) => match b {
                Bracket::LCurly => {
                    self.get_next_token();

                    let statements = self.parse_program(true);

                    self.validate_token(TokenEnum::Bracket(Bracket::RCurly));

                    return IfStatement::new(condition, statements);
                }

                token => {
                    panic!("Expected {{, got {:?}", token);
                }
            },

            token => {
                panic!("Expected {{, got {:?}", token);
            }
        };
    }

    /// we get here after 'else' has been consumed
    pub fn parse_else(&mut self) -> ElseStatement {
        match self.peek_next_token().token {
            TokenEnum::Bracket(b) => match b {
                Bracket::LCurly => {
                    self.get_next_token();

                    let statements = self.parse_statements();

                    self.validate_token(TokenEnum::Bracket(Bracket::RCurly));

                    return ElseStatement::new(statements);
                }

                token => {
                    panic!("Expected {{, got {:?}", token);
                }
            },

            token => {
                panic!("Expected {{, got {:?}", token);
            }
        };
    }
}
