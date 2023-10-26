use crate::ast::abstract_syntax_tree::AST;

use super::parser::Parser;

impl<'a> Parser<'a> {
    /// CONDITIONAL_STATEMENT -> if LPAREN* COMPARISON_EXPRESSION RPAREN* LCURLY STATEMENT[]* RCURLY ELSE_STATEMENT*
    pub fn parse_conditionals(&mut self) -> Box<dyn AST> {
        // we get here after 'if' has been consumed

        // Parse if statements and any and all elif and else statements
        // store them all in one AST
        let condition = self.parse_comparison_expression();

        todo!()
    }
}
