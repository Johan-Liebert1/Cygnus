use std::rc::Rc;

use crate::{
    ast::{abstract_syntax_tree::AST, function_def::FunctionDefinition, variable::Variable},
    lexer::tokens::{Bracket, TokenEnum},
};

use super::parser::{Parser, ParserFunctions};

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

        // println!("parameters {:?}", parameters);

        return parameters;
    }

    /// FUNCTION_DEF -> fun VAR_NAME LPAREN (VAR_NAME : VAR_TYPE (,*))* RPAREN LCURLY STATEMENT[] RCURLY
    pub fn parse_function_definition(&mut self, f: ParserFunctions) -> Rc<Box<dyn AST>> {
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

        let ff = function_name.clone();

        // Create an Rc from the Box
        let function_def = FunctionDefinition::new(function_name, parameters, block);

        let fdef: Rc<Box<dyn AST>> = Rc::new(Box::new(function_def));

        // Use Rc::clone to get a reference-counted clone of the Rc, not the inner value
        f.borrow_mut().insert(ff, Rc::clone(&fdef));

        // Convert Rc back to Box for the return value
        return Rc::clone(&fdef);
    }
}
