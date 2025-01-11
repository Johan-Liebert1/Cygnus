use crate::{
    ast::{abstract_syntax_tree::ASTNodeEnumMut, array::Array, variable::Variable},
    helpers::{compiler_error, unexpected_token},
    lexer::{keywords::AS, lexer::Token, tokens::Operations, types::VarType},
    types::ASTNode,
};

use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::factor::Factor,
    lexer::tokens::{Bracket, TokenEnum},
};

use super::parser::Parser;

impl Parser {
    /// VARIABLE (as type)*
    fn parse_variable_factor(&mut self, var_token: &Token, var_name: &String) -> ASTNode {
        let mut variable = Variable::new(
            Box::new(var_token.clone()),
            VarType::Unknown,
            var_name.into(),
            false,
            false,
            0,
        );

        if let TokenEnum::Keyword(word) = self.peek_next_token().token {
            if word == AS {
                // consume 'as'
                self.get_next_token();

                // the next token HAS to be a type
                let type_cast = self.validate_token(TokenEnum::Type(VarType::Unknown));

                let var_type = if let TokenEnum::Type(var_type) = type_cast.token {
                    var_type
                } else {
                    unreachable!("Validate token failed")
                };

                variable.type_cast = Some(var_type);
            }
        }

        // check if this is an array access
        if let TokenEnum::Bracket(Bracket::LSquare) = self.peek_next_token().token {
            self.get_next_token();

            variable.array_aceess_index = Some(self.parse_logical_expression());

            self.validate_token(TokenEnum::Bracket(Bracket::RSquare));
        }

        let mut member_access = vec![];

        while let TokenEnum::Dot = self.peek_next_token().token {
            self.get_next_token();

            let next_token = self.peek_next_token();

            if let TokenEnum::Variable(member_name) = next_token.token {
                self.get_next_token();
                member_access.push(member_name);
            } else {
                unexpected_token(&next_token, Some(&TokenEnum::Variable("".into())))
            }
        }

        variable.member_access = member_access;

        return Rc::new(RefCell::new(Box::new(variable)));
    }

    /// FACTOR -> (*|&)* INTEGER | FLOAT | VARIABLE (as type)* | STRING_LITERAL | LPAREN EXPRESSION RPAREN | FUNCTION_CALL
    pub fn parse_factor(&mut self) -> ASTNode {
        let next_token = self.peek_next_token();

        match &next_token.token {
            TokenEnum::Number(..) | TokenEnum::StringLiteral(..) => {
                self.get_next_token();
                return Rc::new(RefCell::new(Box::new(Factor::new(Box::new(next_token)))));
            }

            // This could also be a function call
            TokenEnum::Variable(var_name) => {
                // this is not a variable declaration, only a variable
                // name so we don't have type information here
                // This is handled via the call stack
                // This is done in the assignment_statemetn

                let var_token = self.get_next_token();

                let peeked_token = self.peek_next_token();

                match &peeked_token.token {
                    TokenEnum::Bracket(Bracket::LParen) => {
                        // parse_function_call already pushes to the bracket_stack
                        // WE cannot check for other type of parenthesis here as
                        // write(variable) will result in error as there's a ')' following the
                        // variable, but it should be perfectly fine
                        //
                        // This will only be called from a declaration statement or an
                        // assignment_statement which means this is assigned to a value.
                        // simple function call without assignment is handled in parse statements
                        self.parse_function_call(var_name.into(), true)
                    }

                    _ => self.parse_variable_factor(&var_token, var_name),
                }
            }

            TokenEnum::Bracket(paren) => match paren {
                Bracket::LParen => {
                    let tok = self.get_next_token();
                    self.bracket_stack.push(tok);

                    let return_value = self.parse_logical_expression();

                    let peeked_token = self.peek_next_token();

                    match peeked_token.token {
                        TokenEnum::Bracket(Bracket::RParen) => {
                            self.get_next_token();
                            self.bracket_stack.pop();
                            return return_value;
                        }

                        _ => compiler_error("Unclosed (", &peeked_token),
                    };
                }

                Bracket::RParen => match self.bracket_stack.last() {
                    Some(bracket) => {
                        match bracket.token {
                            TokenEnum::Bracket(Bracket::LParen) => {
                                // all good. A left paren was closed
                                self.get_next_token();
                                self.bracket_stack.pop();
                                return Rc::new(RefCell::new(Box::new(Factor::new(Box::new(next_token)))));
                            }

                            TokenEnum::Bracket(Bracket::RParen) => compiler_error("Unclosed (", &bracket),

                            _ => unexpected_token(&next_token, None),
                        }
                    }

                    None => unexpected_token(&next_token, None),
                },

                // Array definition, the RHS bit
                // def a: int[3] = [1, 2, 3];
                Bracket::LSquare => {
                    let bracket_token = self.get_next_token();

                    let mut members = vec![];

                    loop {
                        members.push(self.parse_logical_expression());

                        let peeked_token = self.peek_next_token();

                        match peeked_token.token {
                            TokenEnum::Comma => {
                                self.get_next_token();
                                continue;
                            }

                            TokenEnum::Bracket(Bracket::RSquare) => {
                                self.get_next_token();
                                break;
                            }

                            _ => unexpected_token(&peeked_token, None),
                        }
                    }

                    return Rc::new(RefCell::new(Box::new(Array::new(members, bracket_token))));
                }

                _ => unexpected_token(&next_token, None),
            },

            TokenEnum::Op(Operations::Multiply) => {
                self.get_next_token();

                self.times_dereferenced = 1;

                while let TokenEnum::Op(Operations::Multiply) = self.peek_next_token().token {
                    self.times_dereferenced += 1;
                    self.get_next_token();
                }

                if let TokenEnum::Bracket(Bracket::LParen) = self.peek_next_token().token {
                    self.validate_token(TokenEnum::Bracket(Bracket::LParen));
                    let exp = self.parse_expression();
                    self.validate_token(TokenEnum::Bracket(Bracket::RParen));

                    match exp.borrow_mut().get_node_mut() {
                        ASTNodeEnumMut::Variable(ref mut var) => {
                            var.dereference = true;
                            var.times_dereferenced = self.times_dereferenced;
                        }

                        _ => {}
                    };

                    self.times_dereferenced = 0;

                    return exp;
                }

                // FIXME: Cannot have this accept self.times_dereferenced as the amount of
                // times it's been derefed as
                //
                // *(str as *char) + 3 will be counted as *(str as *char + 3) which is
                // incredibly wrong
                let exp = self.parse_factor();

                match exp.borrow_mut().get_node_mut() {
                    ASTNodeEnumMut::Variable(ref mut var) => {
                        var.dereference = true;
                        var.times_dereferenced = self.times_dereferenced;
                    }

                    _ => {}
                };

                self.times_dereferenced = 0;

                return exp;
            }

            TokenEnum::Ampersand => {
                // consume '&'
                self.get_next_token();

                let next_next_token = self.peek_next_token();

                // the next token has to be a variable, else this is a syntax error
                match next_next_token.token {
                    TokenEnum::Variable(var_name) => {
                        Rc::new(RefCell::new(Box::new(Variable::new(
                            Box::new(self.get_next_token()),
                            // this is not a variable declaration, only a variable
                            // name so we don't have type information here
                            // This is handled via the call stack
                            VarType::Unknown,
                            var_name,
                            false,
                            true,
                            0,
                        ))))
                    }

                    _ => {
                        unexpected_token(&next_next_token, Some(&TokenEnum::Variable("".into())));
                    }
                }
            }

            _ => {
                unexpected_token(&next_token, None);
            }
        }
    }
}
