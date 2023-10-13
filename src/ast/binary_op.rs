use crate::{parser::Token, tokens::Operations};

pub struct BinaryOP<'a> {
    left: &'a Token,
    operand: &'a Operations,
    right: &'a Token,
}

impl<'a> BinaryOP<'a> {
    pub fn new( left: &'a Token, operand: &'a Operations, right: &'a Token) -> Self {
        Self { left, operand, right }
    }
}
