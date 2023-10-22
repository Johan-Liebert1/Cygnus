use crate::ast::{abstract_syntax_tree::AST, comparison_exp::ComparisonExp};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /// COMPARISON    -> EXPRESSION (> | < | >= | <=) EXPRESSION
    pub fn parse_comparison_expression(&mut self) -> Box<dyn AST> {
        let left_expression = self.parse_expression();

        loop {
            let token = self.peek_next_token();

            match token.token {
                crate::tokens::TokenEnum::Comparator(_) => {
                    self.get_next_token();

                    return Box::new(ComparisonExp::new(
                        left_expression,
                        Box::new(token),
                        self.parse_expression(),
                    ));
                }

                _ => {
                    return left_expression;
                }
            }
        }
    }
}
