use crate::{
    ast::variable::Variable,
    helpers::{compiler_error, unexpected_keyword, unexpected_token},
    lexer::{keywords::WITH, types::VarType},
    types::ASTNode,
};

use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{ast_loop::Loop, factor::Factor},
    lexer::{
        keywords::{FROM, STEP, TO},
        lexer::Token,
        tokens::{Bracket, Number, TokenEnum},
    },
};

use super::parser::Parser;

impl Parser {
    /// LOOP -> loop (from LPAREN* EXPRESSION to EXPRESSION (step EXPRESSION)* RPAREN* (with VAR_NAME)*)* LCURLY STATEMENT[] RCURLY
    pub fn parse_loop(&mut self) -> ASTNode {
        // we get here after consuming the 'loop' keyword
        if self.inside_function_depth == 0 {
            compiler_error("Loop cannot be outside a function", &self.peek_next_token());
        }

        // to not mess up nested loops
        let current_loop_number = self.num_loops;
        self.inside_current_loop_number = current_loop_number as i32;

        self.num_loops += 1;

        // Infinite loop
        if matches!(self.peek_next_token().token, TokenEnum::Bracket(Bracket::LCurly)) {
            self.validate_and_consume_token(TokenEnum::Bracket(Bracket::LCurly));

            self.inside_loop_depth += 1;
            let block = self.parse_program();
            self.inside_loop_depth -= 1;

            self.validate_and_consume_token(TokenEnum::Bracket(Bracket::RCurly));

            self.inside_current_loop_number -= 1;

            return Rc::new(RefCell::new(Box::new(Loop::new(
                None,
                None,
                None,
                None,
                block,
                current_loop_number,
            ))));
        };

        self.validate_and_consume_token(TokenEnum::Keyword(FROM.to_string()));

        let from_range = match self.peek_next_token().token {
            TokenEnum::Bracket(..) => {
                // if there is a bracket, it has to be a left paren
                self.validate_and_consume_token(TokenEnum::Bracket(Bracket::LParen));
                let exp = self.parse_expression();
                self.validate_and_consume_token(TokenEnum::Bracket(Bracket::RParen));

                exp
            }

            _ => self.parse_expression(),
        };

        self.validate_and_consume_token(TokenEnum::Keyword(TO.to_string()));

        let to_range = match self.peek_next_token().token {
            TokenEnum::Bracket(..) => {
                // if there is a bracket, it has to be a left paren
                self.validate_and_consume_token(TokenEnum::Bracket(Bracket::LParen));
                let exp = self.parse_expression();
                self.validate_and_consume_token(TokenEnum::Bracket(Bracket::RParen));

                exp
            }

            _ => self.parse_expression(),
        };

        let default_step: ASTNode = Rc::new(RefCell::new(Box::new(Factor::new(Token {
            token: TokenEnum::Number(Number::Integer(1)),
            line_number: 0,
            index: 0,
            col_number: 0,
            file: Rc::new(String::new()),
        }))));

        let step = match self.peek_next_token().token {
            TokenEnum::Keyword(keyword) => {
                match keyword.as_str() {
                    STEP => {
                        // consume 'step'
                        self.consume_token();

                        self.parse_expression()
                    }

                    _ => default_step,
                }
            }

            _ => default_step,
        };

        let next_token = self.peek_next_token();

        let with_var = match &next_token.token {
            TokenEnum::Keyword(keyword) => {
                match keyword.as_str() {
                    WITH => {
                        // consume 'with'
                        self.consume_token();

                        let peek_next_token = self.peek_next_token();

                        // the next token has to be a variable
                        match peek_next_token.token {
                            TokenEnum::Variable(var_name) => Some(Rc::new(RefCell::new(Variable::new(
                                self.consume_token(),
                                VarType::Int,
                                var_name,
                                false,
                                false,
                                0,
                            )))),

                            _ => {
                                unexpected_token(&peek_next_token, Some(&TokenEnum::Variable("".into())));
                            }
                        }
                    }

                    word => {
                        unexpected_keyword(&next_token, word, Some(WITH));
                    }
                }
            }

            _ => {
                self.validate_and_consume_token(TokenEnum::Bracket(Bracket::LCurly));
                None
            }
        };

        if with_var.is_some() {
            self.validate_and_consume_token(TokenEnum::Bracket(Bracket::LCurly));
        }

        self.inside_loop_depth += 1;
        let block = self.parse_program();
        self.inside_loop_depth -= 1;

        self.validate_and_consume_token(TokenEnum::Bracket(Bracket::RCurly));

        self.inside_current_loop_number -= 1;

        return Rc::new(RefCell::new(Box::new(Loop::new(
            Some(from_range),
            Some(to_range),
            Some(step),
            with_var,
            block,
            current_loop_number,
        ))));
    }
}
