use crate::{helpers::unexpected_token, types::ASTNode};

use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::conditionals::{ConditionalStatement, ElseStatement, IfStatement},
    lexer::{
        keywords::{ELIF_STATEMENT, ELSE_STATEMENT},
        tokens::{Bracket, TokenEnum},
    },
};

use super::parser::Parser;

impl Parser {
    /// CONDITIONAL_STATEMENT -> if LPAREN* COMPARISON_EXPRESSION RPAREN* LCURLY STATEMENT[]* RCURLY ELSE_STATEMENT*
    pub fn parse_conditional_statement(&mut self) -> ASTNode {
        // we get here after 'if' has been consumed

        // parse the if statements
        let if_statement = self.parse_if_elif();

        let mut elif_ladder: Vec<IfStatement> = vec![];

        // parse the elif ladder
        loop {
            let token = self.peek_next_token();

            if let TokenEnum::Keyword(keyword) = token.token {
                if keyword == ELIF_STATEMENT {
                    self.consume_token();

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
                self.consume_token();

                else_statement = Some(self.parse_else());
            }
        }

        return Rc::new(RefCell::new(Box::new(ConditionalStatement::new(
            if_statement,
            elif_ladder,
            else_statement,
        ))));
    }

    /// we get here after 'if' has been consumed
    pub fn parse_if_elif(&mut self) -> IfStatement {
        // Parse if statements and any and all elif and else statements
        // store them all in one AST
        let condition = self.parse_logical_expression();

        let token = self.peek_next_token();

        match &token.token {
            TokenEnum::Bracket(b) => match b {
                Bracket::LCurly => {
                    self.consume_token();

                    self.inside_if_else_depth += 1;
                    let statements = self.parse_program();
                    self.inside_if_else_depth -= 1;

                    self.validate_and_consume_token(TokenEnum::Bracket(Bracket::RCurly));

                    return IfStatement::new(condition, statements);
                }

                _ => {
                    unexpected_token(&token, Some(&TokenEnum::Bracket(Bracket::LCurly)));
                }
            },

            _ => {
                unexpected_token(&token, Some(&TokenEnum::Bracket(Bracket::LCurly)));
            }
        };
    }

    /// we get here after 'else' has been consumed
    pub fn parse_else(&mut self) -> ElseStatement {
        let token = self.peek_next_token();

        match &token.token {
            TokenEnum::Bracket(b) => match b {
                Bracket::LCurly => {
                    self.consume_token();

                    self.inside_if_else_depth += 1;
                    let statements = self.parse_program();
                    self.inside_if_else_depth -= 1;

                    self.validate_and_consume_token(TokenEnum::Bracket(Bracket::RCurly));

                    return ElseStatement::new(statements);
                }

                _ => {
                    unexpected_token(&token, Some(&TokenEnum::Bracket(Bracket::LCurly)));
                }
            },

            _ => {
                unexpected_token(&token, Some(&TokenEnum::Bracket(Bracket::LCurly)));
            }
        };
    }
}
