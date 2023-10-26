use crate::lexer::tokens::{Number, TokenEnum};

use super::abstract_syntax_tree::{VisitResult, AST};

pub struct Loop {
    /// an expression
    from_range: Box<dyn AST>,
    /// an expression
    to_range: Box<dyn AST>,
    block: Box<dyn AST>,
}

impl Loop {
    pub fn new(from_range: Box<dyn AST>, to_range: Box<dyn AST>, block: Box<dyn AST>) -> Self {
        Self {
            from_range,
            to_range,
            block,
        }
    }
}

impl AST for Loop {
    fn visit(&self) -> VisitResult {
        let from = self.from_range.visit();
        let to = self.to_range.visit();

        if !from.token.is_integer() || !to.token.is_integer() {
            panic!("Expected from and to expressions to be Integer");
        }

        let from = if let TokenEnum::Number(Number::Integer(i)) = *from.token {
            i
        } else {
            unreachable!("Somehow did not get integer even after performing Integer enum check")
        };

        let to = if let TokenEnum::Number(Number::Integer(i)) = *to.token {
            i
        } else {
            unreachable!("Somehow did not get integer even after performing Integer enum check")
        };

        for _ in from..to {
            // TODO: Remove this once print statements are implemented
            println!("{:?}", self.block.visit());
        }

        return VisitResult {
            token: Box::new(TokenEnum::Unknown),
        };
    }

    fn get_token(&self) -> &crate::lexer::lexer::Token {
        todo!()
    }

    fn print(&self) {
        todo!()
    }
}
