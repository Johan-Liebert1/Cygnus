use std::fmt::Display;

use super::{
    keywords,
    tokens::{Number, Operations, TokenEnum},
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
    pub index: usize,
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

    pub fn construct_number(&mut self) -> TokenEnum {
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

    pub fn construct_word(&mut self) -> TokenEnum {
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

        if keywords::TYPES.contains(&word.as_str()) {
            return TokenEnum::Type(word);
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

    pub fn is_comment(&mut self) -> bool {
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
}
