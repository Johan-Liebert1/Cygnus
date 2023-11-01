use std::process::exit;

use crate::{
    constants,
    interpreter::interpreter::Variables,
    lexer::{
        lexer::Token,
        tokens::{Comparators, Number, TokenEnum},
    },
};

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct ComparisonExp {
    left: Box<dyn AST>,
    comp_op: Box<Token>,
    right: Box<dyn AST>,
}

impl ComparisonExp {
    pub fn new(left: Box<dyn AST>, comp_op: Box<Token>, right: Box<dyn AST>) -> Self {
        Self {
            left,
            comp_op,
            right,
        }
    }

    fn get_left(&self, i: &mut Variables) -> Number {
        match *self.left.visit(i).token {
            TokenEnum::Number(number) => {
                return number;
            }

            _ => {
                exit(1);
            }
        }
    }

    fn get_right(&self, i: &mut Variables) -> Number {
        match *self.right.visit(i).token {
            TokenEnum::Number(number) => {
                return number;
            }

            _ => {
                exit(1);
            }
        }
    }

    fn compare<T>(&self, l: T, r: T) -> TokenEnum
    where
        T: PartialOrd,
    {
        return TokenEnum::Bool(match &self.comp_op.token {
            TokenEnum::Comparator(comp) => match comp {
                Comparators::LessThan => l < r,
                Comparators::GreaterThan => l > r,
                Comparators::LessThanEq => l <= r,
                Comparators::GreaterThanEq => l >= r,
            },

            _ => {
                unreachable!("Found non comparator")
            }
        });
    }
}

impl AST for ComparisonExp {
    fn visit(&self, i: &mut Variables) -> VisitResult {
        if constants::DEBUG_AST {
            println!("{:?}", &self);
        }

        if let Number::Integer(l) = self.get_left(i) {
            if let Number::Integer(r) = self.get_right(i) {
                return VisitResult {
                    token: Box::new(self.compare(l, r)),
                };
            }

            panic!("Cannot compare Float with Integer");
        };

        if let Number::Float(l) = self.get_left(i) {
            if let Number::Float(r) = self.get_right(i) {
                return VisitResult {
                    token: Box::new(self.compare(l, r)),
                };
            }

            panic!("Cannot compare Float with Integer");
        };

        panic!("wat add");
    }

    fn get_token(&self) -> &Token {
        return &self.comp_op;
    }

    fn print(&self) {
        println!("{:#?}", self);
    }
}
