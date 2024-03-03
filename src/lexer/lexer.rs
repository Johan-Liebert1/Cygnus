use std::fmt::Display;

use crate::{
    lexer::types::{TYPE_FLOAT, TYPE_STRING},
    trace,
};

use super::{
    keywords::{self, LOGICAL_AND, LOGICAL_OR},
    tokens::{Bracket, LogicalOps, Number, Operations, TokenEnum},
    types::{VarType, TYPES, TYPE_CHAR, TYPE_INT},
};

#[derive(Debug, Clone)]
pub struct Token {
    pub token: TokenEnum,
    pub line_number: usize,
    pub col_number: usize,
    pub file: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    pub line_number: usize,
    pub col_number: usize,
    pub file: &'a Vec<u8>,
    pub index: usize,
    pub file_name: &'a String,
}

impl<'a> Lexer<'a> {
    pub fn new(file: &'a Vec<u8>, file_name: &'a String) -> Self {
        Lexer {
            line_number: 1,
            col_number: 1,
            index: 0,
            file,
            file_name,
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

            if !char.is_alphanumeric() && char != '_' {
                break;
            }

            word += &char.to_string();

            self.col_number += 1;
            self.index += 1;
        }

        self.index -= 1;

        if keywords::KEYWORDS.contains(&word.as_str()) {
            if word.as_str() == LOGICAL_AND {
                return TokenEnum::LogicalOp(LogicalOps::And);
            }

            if word.as_str() == LOGICAL_OR {
                return TokenEnum::LogicalOp(LogicalOps::Or);
            }

            return TokenEnum::Keyword(word);
        }

        if TYPES.contains(&word.as_str()) {
            let tok = TokenEnum::Type(match word.as_str() {
                TYPE_INT => VarType::Int,
                TYPE_FLOAT => VarType::Float,
                TYPE_STRING => VarType::Str,
                TYPE_CHAR => VarType::Char,

                _ => {
                    panic!("Unknown type '{word}'")
                }
            });

            return tok;
        }

        return TokenEnum::Variable(word);
    }

    pub fn construct_string(&mut self) -> TokenEnum {
        // we get here when we encounter "

        // skip past the '"'
        self.index += 1;

        let mut string_literal = String::new();

        while self.index < self.file.len() {
            match self.file[self.index] {
                b'"' => {
                    break;
                }

                _ => {
                    string_literal += &(self.file[self.index] as char).to_string();
                    self.index += 1;
                    self.col_number += 1;
                }
            }
        }

        // don't inc index here as it'll be incremented in the lexer.advance func anyway
        return TokenEnum::StringLiteral(string_literal);
    }

    fn parse_comment(&mut self) {
        while self.index < self.file.len() {
            match self.file[self.index] {
                b'\n' => {
                    // not incrementing line number here as that's aready done inside of self.advance
                    self.col_number = 1;
                    break;
                }

                _ => {
                    self.index += 1;
                    self.col_number += 1;
                }
            }
        }
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
