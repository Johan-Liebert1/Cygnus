use crate::{
    ast::{abstract_syntax_tree::AST, program::Program},
    lexer::{
        keywords::{ELIF_STATEMENT, ELSE_STATEMENT, IF_STATEMENT, LOOP, VAR_DEFINE},
        lexer::{Lexer, Token},
        tokens::{Bracket, TokenEnum},
    },
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

    /// Validates the current token with expected token and consumes the token
    /// panics if current token is not the same as expected token
    pub fn validate_token(&mut self, token_expected: TokenEnum) {
        let token = self.get_next_token();

        if token.token != token_expected {
            panic!("Expected {:?}, got {:?}", token_expected, token);
        }
    }

    /// STATEMENT -> VARIABLE_DECLARATION | CONDITIONAL_STATEMENT | COMPARISON_EXPRESSION | LPAREN COMPARISON_EXPRESSION RPAREN
    pub fn parse_statements(&mut self) -> Box<dyn AST> {
        let current_token = self.peek_next_token();

        match &current_token.token {
            TokenEnum::Keyword(keyword) => {
                self.get_next_token();

                match keyword as &str {
                    VAR_DEFINE => self.parse_assignment_statement(),

                    IF_STATEMENT => self.parse_conditional_statement(),

                    LOOP => self.parse_loop(),

                    ELSE_STATEMENT => {
                        panic!("Found 'else' without an 'if' {:?}", current_token)
                    }
                    ELIF_STATEMENT => {
                        panic!("Found 'elif' without an 'if' {:?}", current_token)
                    }

                    _ => {
                        panic!("Keyword '{}' not recognised", keyword);
                    }
                }
            }

            // FIXME: This cannot be any bracket, example { is not correct
            TokenEnum::Number(..) | TokenEnum::Bracket(..) => self.parse_comparison_expression(),

            TokenEnum::Variable(var) => {
                // 2 here as we haven't consumed the `var` token
                let nth_token = self.peek_nth_token(2);

                println!("found var {}. nth token {:#?}", var, nth_token);

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

                    _ => {
                        // println!("In the variable thing. Token {:?}", a);
                        self.parse_comparison_expression()
                    }
                }
            }

            TokenEnum::Op(_) => todo!(),
            TokenEnum::Equals => todo!(),
            TokenEnum::Comparator(_) => todo!(),
            TokenEnum::Bool(_) => todo!(),
            TokenEnum::Type(_) => todo!(),
            TokenEnum::Colon => todo!(),
            TokenEnum::Comma => todo!(),

            TokenEnum::Unknown(..) => {
                panic!("Unknown token: {:?}", &current_token);
            }

            TokenEnum::EOF => {
                unreachable!("Reached EOF");
            }
        }
    }

    pub fn parse_program(&mut self) -> Box<dyn AST> {
        let mut statements: Vec<Box<dyn AST>> = vec![];

        loop {
            let current_token = self.peek_next_token();

            match &current_token.token {
                TokenEnum::EOF => {
                    break;
                }

                _ => {
                    statements.push(self.parse_statements());
                }
            }
        }

        return Box::new(Program::new(statements));
    }
}
