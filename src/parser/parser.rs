use crate::types::ASTNode;

use core::panic;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{
        abstract_syntax_tree::AST,
        jump::{Jump, JumpType},
        program::Program,
    },
    interpreter::interpreter::Functions,
    lexer::{
        keywords::{
            BREAK, ELIF_STATEMENT, ELSE_STATEMENT, FUNCTION_DEFINE, IF_STATEMENT, LOOP, RETURN,
            VAR_DEFINE,
        },
        lexer::{Lexer, Token},
        tokens::{Bracket, TokenEnum},
    },
};

pub type ParserFunctions = Rc<RefCell<Functions>>;

#[derive(Debug)]
pub struct Parser<'a> {
    pub lexer: Box<Lexer<'a>>,
    parsed_tokens: Vec<Token>,
    pub bracket_stack: Vec<Bracket>,
    pub functions: ParserFunctions,

    pub inside_loop_depth: usize,
    pub inside_function_depth: usize,
    pub inside_if_else_depth: usize,
}

impl<'a> Parser<'a> {
    pub fn new(file: &'a Vec<u8>) -> Self {
        let parser = Lexer::new(file);

        Self {
            lexer: Box::new(parser),
            parsed_tokens: vec![],
            bracket_stack: vec![],
            functions: Rc::new(RefCell::new(HashMap::new())),

            inside_loop_depth: 0,
            inside_function_depth: 0,
            inside_if_else_depth: 0,
        }
    }

    /// Validates the current token with expected token and consumes the token
    /// panics if current token is not the same as expected token
    pub fn validate_token(&mut self, token_expected: TokenEnum) {
        let token = self.get_next_token();

        if token.token != token_expected {
            panic!("Expected {:?}, got {:?}", token_expected, token);
        }
    }

    /// STATEMENT -> VARIABLE_DECLARATION | CONDITIONAL_STATEMENT | COMPARISON_EXPRESSION | LPAREN COMPARISON_EXPRESSION RPAREN
    pub fn parse_statements(&mut self) -> ASTNode {
        let current_token = self.peek_next_token();

        // println!("parse_statements current_token {:#?}", current_token);

        match &current_token.token {
            TokenEnum::Keyword(keyword) => {
                self.get_next_token();

                match keyword as &str {
                    VAR_DEFINE => self.parse_declaration_statement(),

                    IF_STATEMENT => self.parse_conditional_statement(),

                    LOOP => self.parse_loop(),

                    FUNCTION_DEFINE => {
                        if self.inside_function_depth != 0 {
                            // don't allow function in function definitions
                            panic!("Defining function inside functions is not allowed");
                        }

                        self.parse_function_definition(Rc::clone(&self.functions))
                    }

                    BREAK => {
                        if self.inside_loop_depth == 0 {
                            panic!("Found `break` outside of a loop");
                        }

                        Rc::new(RefCell::new(Box::new(Jump::new(JumpType::Break))))
                    }

                    RETURN => {
                        if self.inside_function_depth == 0 {
                            panic!("Found `return` outside of a function");
                        }

                        Rc::new(RefCell::new(Box::new(Jump::new(JumpType::Return))))
                    }

                    ELSE_STATEMENT => {
                        panic!("Found 'else' without an 'if' {:?}", current_token)
                    }

                    ELIF_STATEMENT => {
                        panic!("Found 'elif' without an 'if' {:?}", current_token)
                    }

                    _ => {
                        println!(
                            "loop {}, func {}, if {}",
                            self.inside_loop_depth,
                            self.inside_function_depth,
                            self.inside_if_else_depth
                        );
                        panic!("Keyword '{}' not recognised", keyword);
                    }
                }
            }

            // FIXME: This cannot be any bracket, example { is not correct
            TokenEnum::Number(..) | TokenEnum::Bracket(..) => self.parse_logical_expression(),

            TokenEnum::Variable(var) => {
                // 2 here as we haven't consumed the `var` token
                let nth_token = self.peek_nth_token(2);

                // println!("parse_statements variable nth_token {:#?}", current_token);

                match nth_token.token {
                    TokenEnum::Bracket(b) => {
                        match b {
                            Bracket::LParen => {
                                // function invocation
                                self.get_next_token();
                                self.parse_function_call(var.to_string())
                            }

                            Bracket::RParen => todo!(),
                            Bracket::LCurly => todo!(),
                            Bracket::RCurly => todo!(),
                        }
                    }

                    TokenEnum::Equals => {
                        // variable assignment
                        self.get_next_token();
                        self.parse_assignment_statement(var.to_string())
                    }

                    e => {
                        panic!("Expected `)` or `=` after {}, got {:?}", var, e)
                    }
                }
            }

            TokenEnum::LogicalOp(op) => {
                panic!("Expected statement, found {:?}", op)
            }

            TokenEnum::StringLiteral(_) => todo!(),

            TokenEnum::Op(_) => todo!(),
            TokenEnum::Equals => todo!(),
            TokenEnum::Comparator(_) => todo!(),
            TokenEnum::Bool(_) => todo!(),
            TokenEnum::Type(_) => todo!(),
            TokenEnum::Colon => todo!(),
            TokenEnum::Comma => todo!(),
            TokenEnum::SemiColon => todo!(),

            TokenEnum::Unknown(..) => {
                panic!("Unknown token: {:?}", &current_token);
            }

            TokenEnum::EOF => {
                unreachable!("Reached EOF");
            }
        }
    }

    pub fn parse_program(&mut self) -> ASTNode {
        let mut statements: Vec<ASTNode> = vec![];

        loop {
            let current_token = self.peek_next_token();

            match &current_token.token {
                TokenEnum::EOF => {
                    break;
                }

                TokenEnum::SemiColon => {
                    self.get_next_token();
                    continue;
                }

                TokenEnum::Bracket(b) => match b {
                    Bracket::RCurly => {
                        if self.inside_function_depth > 0
                            || self.inside_loop_depth > 0
                            || self.inside_if_else_depth > 0
                        {
                            return Rc::new(RefCell::new(Box::new(Program::new(statements))));
                        } else {
                            statements.push(self.parse_statements())
                        }
                    }

                    _ => statements.push(self.parse_statements()),
                },

                _ => {
                    statements.push(self.parse_statements());
                }
            }
        }

        return Rc::new(RefCell::new(Box::new(Program::new(statements))));
    }
}
