use crate::{
    ast::{abstract_syntax_tree::AST, function_def::FunctionDefinition, variable::Variable},
    lexer::tokens::{Bracket, TokenEnum},
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    fn parse_function_definition_parameters(&mut self) -> Vec<Variable> {
        let mut parameters = vec![];

        loop {
            let token = self.peek_next_token();

            match &token.token {
                TokenEnum::Bracket(b) => match b {
                    Bracket::RParen => {
                        self.get_next_token();
                        break;
                    }

                    _ => panic!("Unexpected token {:#?}", token),
                },

                TokenEnum::Comma => {
                    self.get_next_token();
                    continue;
                }

                _ => {
                    let variable = self.parse_variable();
                    parameters.push(variable);
                }
            };
        }

        println!("parameters {:?}", parameters);

        return parameters;
    }

    /// FUNCTION_DEF -> fun VAR_NAME LPAREN (VAR_NAME : VAR_TYPE (,*))* RPAREN LCURLY STATEMENT[] RCURLY
    pub fn parse_function_definition(&mut self) -> Box<dyn AST> {
        // we get here after consuming 'fn'

        let function_name = match self.get_next_token().token {
            TokenEnum::Variable(n) => n,

            _ => {
                panic!("Expected function name")
            }
        };

        match self.get_next_token().token {
            TokenEnum::Bracket(b) => match b {
                Bracket::LParen => (),
                _ => panic!("Expected LParen"),
            },

            _ => {
                panic!("Expected LParen")
            }
        };

        let parameters = self.parse_function_definition_parameters();

        match self.get_next_token().token {
            TokenEnum::Bracket(b) => match b {
                Bracket::LCurly => (),
                _ => panic!("Expected LCurly"),
            },

            _ => {
                panic!("Expected LCurly")
            }
        };

        let block = self.parse_statements();

        match self.get_next_token().token {
            TokenEnum::Bracket(b) => match b {
                Bracket::RCurly => (),
                _ => panic!("Expected RCurly"),
            },

            _ => {
                panic!("Expected RCurly")
            }
        };

        return Box::new(FunctionDefinition::new(function_name, parameters, block));
    }
}
