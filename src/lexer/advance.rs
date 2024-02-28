use crate::trace;

use super::{
    lexer::{Lexer, Token},
    tokens::{Bracket, Comparators, Operations, TokenEnum},
    types::VarType,
};

impl<'a> Lexer<'a> {
    pub fn peek_next_token(&mut self) -> Token {
        return self.peek_nth_token(1);
    }

    pub fn get_next_token(&mut self) -> Token {
        let token = self.advance_to_next_token(false);
        return token;
    }

    pub fn peek_nth_token(&mut self, n: usize) -> Token {
        let index = self.index;
        let col_number = self.col_number;
        let line_number = self.line_number;

        let mut token: Token = Token {
            token: TokenEnum::EOF,
            line_number: 0,
            col_number: 0,
            file: self.file_name.to_string(),
        };

        for _ in 0..n {
            token = self.advance_to_next_token(false);
        }

        self.index = index;
        self.col_number = col_number;
        self.line_number = line_number;

        return token;
    }

    fn advance_to_next_token(&mut self, peek: bool) -> Token {
        let index = self.index;
        let col_number = self.col_number;
        let line_number = self.line_number;

        while self.index < self.file.len() {
            let char = self.file[self.index] as char;

            // if !peek {
            //     println!("Current char '{}' Peek: {}", char, peek);
            // }

            let token = match char {
                ' ' | '\t' => {
                    self.index += 1;
                    self.col_number += 1;
                    continue;
                }

                '\n' => {
                    self.index += 1;
                    self.col_number = 1;
                    self.line_number += 1;

                    continue;
                }

                '+' => {
                    self.index += 1;

                    match self.peek_next_token().token {
                        TokenEnum::Equals => TokenEnum::PlusEquals,
                        _ => TokenEnum::Op(Operations::Plus),
                    }
                }

                // TODO: Handle negative integers
                '-' => {
                    if self.is_comment() {
                        continue;
                    } else {
                        self.index += 1;

                        match self.peek_next_token().token {
                            TokenEnum::Equals => TokenEnum::MinusEquals,
                            TokenEnum::Comparator(Comparators::GreaterThan) => TokenEnum::FunctionReturnIndicator,
                            _ => TokenEnum::Op(Operations::Minus),
                        }
                    }
                }

                '*' => {
                    // this could also be a pointer type
                    // Ex: def a: *int = 5;

                    self.index += 1;

                    // FIXME: Every peek inside of advance should decrement the index after it's
                    // done with it
                    match self.peek_next_token().token {
                        TokenEnum::Type(type_) => {
                            // consume the 'type_' token
                            self.get_next_token();
                            TokenEnum::Type(VarType::Ptr(Box::new(type_)))
                        }

                        _ => {
                            self.index -= 1;
                            TokenEnum::Op(Operations::Multiply)
                        }
                    }
                }

                '/' => TokenEnum::Op(Operations::Divide),
                '%' => TokenEnum::Op(Operations::Modulo),

                '&' => TokenEnum::Ampersand,

                '=' => {
                    self.index += 1;

                    let tok = match self.peek_next_token().token {
                        TokenEnum::Equals => TokenEnum::Comparator(Comparators::DoubleEquals),

                        _ => TokenEnum::Equals,
                    };

                    tok
                }

                '!' => {
                    self.index += 1;

                    let tok = match self.peek_next_token().token {
                        TokenEnum::Equals => TokenEnum::Comparator(Comparators::NotEquals),

                        _ => TokenEnum::Unknown("!".to_string()),
                    };

                    tok
                }

                '(' => TokenEnum::Bracket(Bracket::LParen),
                ')' => TokenEnum::Bracket(Bracket::RParen),

                '{' => TokenEnum::Bracket(Bracket::LCurly),
                '}' => TokenEnum::Bracket(Bracket::RCurly),

                ':' => TokenEnum::Colon,
                ';' => TokenEnum::SemiColon,
                ',' => TokenEnum::Comma,

                '"' => self.construct_string(),

                // TODO: This messes up the column number in the final output
                '>' => {
                    self.index += 1;

                    match self.peek_next_token().token {
                        TokenEnum::Equals => {
                            self.get_next_token();
                            self.index -= 1;
                            TokenEnum::Comparator(Comparators::GreaterThanEq)
                        }

                        TokenEnum::Comparator(com) => match com {
                            Comparators::GreaterThan => TokenEnum::Op(Operations::ShiftRight),

                            e => TokenEnum::Unknown(format!("<{:?}", e)),
                        },

                        _ => {
                            self.index -= 1;
                            TokenEnum::Comparator(Comparators::GreaterThan)
                        }
                    }
                }

                '<' => {
                    self.index += 1;

                    let tok = match self.peek_next_token().token {
                        TokenEnum::Equals => {
                            self.get_next_token();
                            self.index -= 1;
                            TokenEnum::Comparator(Comparators::LessThanEq)
                        }

                        TokenEnum::Comparator(com) => match com {
                            Comparators::LessThan => TokenEnum::Op(Operations::ShiftLeft),

                            e => TokenEnum::Unknown(format!("<{:?}", e)),
                        },

                        _ => {
                            self.index -= 1;
                            TokenEnum::Comparator(Comparators::LessThan)
                        }
                    };

                    tok
                }

                // only handle ASCII for now
                _ => match self.file[self.index] {
                    65..=90 | 97..=122 => self.construct_word(),

                    48..=57 => self.construct_number(),

                    _ => TokenEnum::Unknown(char.to_string()),
                },
            };

            self.index += 1;

            let token = Token {
                token,
                line_number: self.line_number,
                col_number: self.col_number,
                file: self.file_name.into(),
            };

            if peek {
                self.index = index;
                self.col_number = col_number;
                self.line_number = line_number;
            }

            return token;
        }

        return Token {
            token: TokenEnum::EOF,
            line_number: self.line_number,
            col_number: self.col_number,
            file: self.file_name.into(),
        };
    }
}
