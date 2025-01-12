use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::lexer::types::{TYPE_FLOAT, TYPE_STRING};

use super::{
    keywords::{self, LOGICAL_AND, LOGICAL_NOT, LOGICAL_OR},
    tokens::{LogicalOps, Number, TokenEnum},
    types::{VarType, PREDEFINED_TYPES, TYPE_CHAR, TYPE_INT, TYPE_INT16, TYPE_INT32, TYPE_INT8},
};

#[derive(Clone)]
pub struct Token {
    pub token: TokenEnum,
    pub line_number: usize,
    pub col_number: usize,
    pub index: usize,
    pub file: Rc<String>,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "< {} {}:{}:{} {:?} >",
            self.index, self.file, self.line_number, self.col_number, self.token
        )
    }
}

#[derive(Debug)]
pub struct Lexer {
    pub line_number: usize,
    pub col_number: usize,
    pub file: Vec<u8>,
    pub index: usize,
    pub file_name: Rc<String>,
}

impl Lexer {
    pub fn new(file: Vec<u8>, file_name: String) -> Self {
        Lexer {
            line_number: 1,
            col_number: 1,
            index: 0,
            file,
            file_name: Rc::new(file_name),
        }
    }

    pub fn construct_number(&mut self) -> TokenEnum {
        if self.index + 1 > self.file.len() {
            return TokenEnum::EOF;
        }

        let is_hex = self.file[self.index + 1] == 'x' as u8;

        let mut int_string = String::new();
        let mut is_float = false;

        if is_hex {
            self.index += 2
        }

        while self.index < self.file.len() {
            let char = self.file[self.index] as char;

            if !is_hex && !char.is_numeric() && char != '.' {
                break;
            }

            if is_hex && !char.is_ascii_hexdigit() {
                break;
            }

            if char == '.' {
                is_float = true;
            }

            int_string += &char.to_string();

            // self.col_number += 1;
            self.index += 1;
        }

        self.index -= 1;

        if !is_float {
            TokenEnum::Number(Number::Integer(
                i32::from_str_radix(&int_string, if is_hex { 16 } else { 10 }).unwrap(),
            ))
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

            if word.as_str() == LOGICAL_NOT {
                return TokenEnum::LogicalOp(LogicalOps::Not);
            }

            return TokenEnum::Keyword(word);
        }

        if PREDEFINED_TYPES.contains(&word.as_str()) {
            let tok = TokenEnum::Type(match word.as_str() {
                TYPE_INT => VarType::Int,
                TYPE_INT8 => VarType::Int8,
                TYPE_INT16 => VarType::Int16,
                TYPE_INT32 => VarType::Int32,
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
        while self.index < self.file.len() && self.file[self.index] != b'\n' {
            self.index += 1;
        }
    }

    pub fn is_comment(&mut self) -> bool {
        self.index += 1;

        if self.file[self.index] == b'-' {
            self.index += 1;

            // sleep(Duration::from_millis(100));

            // we have found a comment
            self.parse_comment();

            return true;
        }

        self.index -= 1;
        return false;
    }
}
