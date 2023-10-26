use std::fmt::Display;

use super::{
    keywords,
    tokens::{Bracket, Comparators, Number, Operations, TokenEnum},
};

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

    fn construct_word(&mut self) -> TokenEnum {
        let mut word = String::new();

        while self.index < self.file.len() {
            let char = self.file[self.index] as char;

            if !char.is_alphanumeric() {
                break;
            }

            word += &char.to_string();

            self.col_number += 1;
            self.index += 1;
        }

        self.index -= 1;

        if keywords::KEYWORDS.contains(&word.as_str()) {
            return TokenEnum::Keyword(word);
        }

        return TokenEnum::Variable(word);
    }

    fn parse_comment(&mut self) -> String {
        let mut comment = String::new();

        while self.index < self.file.len() {
            match self.file[self.index] {
                b'\n' => {
                    self.line_number += 1;
                    self.col_number = 0;
                    break;
                }

                _ => {
                    comment += &(self.file[self.index] as char).to_string();
                    self.index += 1;
                }
            }
        }

        return comment;
    }

    fn is_comment(&mut self) -> bool {
        self.index += 1;

        match self.peek_next_token().token {
            TokenEnum::Op(op) => match op {
                Operations::Minus => {
                    self.get_next_token();

                    // we have found a comment
                    self.parse_comment();

                    return true;
                }

                _ => {
                    self.index -= 1;
                    return false;
                }
            },

            _ => {
                self.index -= 1;
                return false;
            }
        }
    }

    pub fn peek_next_token(&mut self) -> Token {
        return self.advance_to_next_token(true);
    }

    pub fn get_next_token(&mut self) -> Token {
        let token = self.advance_to_next_token(false);

        // println!("Lexer: {:#?}", token);

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
                    self.col_number = 0;
                    self.line_number += 1;
                    continue;
                }

                '+' => TokenEnum::Op(Operations::Plus),
                '-' => {
                    if self.is_comment() {
                        continue;
                    } else {
                        TokenEnum::Op(Operations::Minus)
                    }
                }
                '*' => TokenEnum::Op(Operations::Multiply),
                '/' => TokenEnum::Op(Operations::Divide),

                '=' => TokenEnum::Equals,

                '(' => TokenEnum::Bracket(Bracket::LParen),
                ')' => TokenEnum::Bracket(Bracket::RParen),

                '{' => TokenEnum::Bracket(Bracket::LCurly),
                '}' => TokenEnum::Bracket(Bracket::RCurly),

                // TODO: This messes up the column number in the final output
                '>' => TokenEnum::Comparator({
                    self.index += 1;

                    match self.peek_next_token().token {
                        TokenEnum::Equals => {
                            self.get_next_token();

                            self.index -= 1;
                            Comparators::GreaterThanEq
                        }

                        _ => {
                            self.index -= 1;
                            Comparators::GreaterThan
                        }
                    }
                }),

                '<' => TokenEnum::Comparator({
                    self.index += 1;

                    match self.peek_next_token().token {
                        TokenEnum::Equals => {
                            self.get_next_token();
                            self.index -= 1;
                            Comparators::LessThanEq
                        }

                        _ => {
                            self.index -= 1;
                            Comparators::LessThanEq
                        }
                    }
                }),

                // only handle ASCII for now
                _ => match self.file[self.index] {
                    65..=90 | 97..=122 => self.construct_word(),

                    48..=57 => self.construct_number(),

                    _ => TokenEnum::Unknown,
                },
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
