use std::fmt::Display;

use crate::tokens::{Number, Operations, TokenEnum};

#[derive(Debug)]
pub struct Token {
    pub token: TokenEnum,
    pub line_number: usize,
    pub col_number: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "T: {:?}             , L: {:}, C: {}",
            self.token, self.line_number, self.col_number
        )
    }
}

pub struct Lexer<'a> {
    pub line_number: usize,
    pub col_number: usize,
    pub file: &'a Vec<u8>,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(file: &'a Vec<u8>) -> Self {
        Lexer {
            line_number: 0,
            col_number: 0,
            index: 0,
            file,
        }
    }

    fn construct_number(&mut self) -> TokenEnum {
        let mut int_string = String::new();

        let mut is_float = false;

        while self.index < self.file.len() {
            let char = self.file[self.index] as char;

            if !char.is_numeric() && char != '.' {
                break;
            }

            if char == '.' {
                is_float = true;
            }

            int_string += &char.to_string();

            self.col_number += 1;
            self.index += 1;
        }

        self.index -= 1;

        if !is_float {
            TokenEnum::Number(Number::Integer(int_string.parse::<i32>().unwrap()))
        } else {
            TokenEnum::Number(Number::Float(int_string.parse::<f32>().unwrap()))
        }
    }

    pub fn peek_next_token(&mut self) -> Token {
        return self.advance_to_next_token(true);
    }

    pub fn get_next_token(&mut self) -> Token {
        return self.advance_to_next_token(false);
    }

    pub fn advance_to_next_token(&mut self, peek: bool) -> Token {
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
                    self.col_number = 0;
                    self.line_number += 1;
                    continue;
                }

                '+' => TokenEnum::Op(Operations::Plus),
                '-' => TokenEnum::Op(Operations::Minus),
                '*' => TokenEnum::Op(Operations::Multiply),
                '/' => TokenEnum::Op(Operations::Divide),

                '=' => TokenEnum::Equals,
                '(' => TokenEnum::LParen,
                ')' => TokenEnum::RParen,

                _ => {
                    if char.is_numeric() {
                        self.construct_number()
                    } else {
                        TokenEnum::Unknown
                    }
                }
            };

            self.index += 1;

            let token = Token {
                token,
                line_number: self.line_number,
                col_number: self.col_number,
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
        };
    }
}
