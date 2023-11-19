use super::{
    lexer::{Lexer, Token},
    tokens::{Bracket, Comparators, Operations, TokenEnum},
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
                    self.col_number = 0;
                    self.line_number += 1;
                    continue;
                }

                '+' => TokenEnum::Op(Operations::Plus),

                // TODO: Handle negative integers
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

                ':' => TokenEnum::Colon,
                ',' => TokenEnum::Comma,

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
                            Comparators::LessThan
                        }
                    }
                }),

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
