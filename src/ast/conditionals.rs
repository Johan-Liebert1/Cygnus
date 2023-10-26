use crate::lexer::{
    lexer::Token,
    tokens::{Boolean, TokenEnum},
};

use super::abstract_syntax_tree::{VisitResult, AST};

pub struct IfStatement {
    condition: Box<dyn AST>,
    block: Box<dyn AST>,
}

impl IfStatement {
    pub fn new(condition: Box<dyn AST>, block: Box<dyn AST>) -> Self {
        Self { condition, block }
    }
}

pub struct ElseStatement {
    block: Box<dyn AST>,
}

impl ElseStatement {
    pub fn new(block: Box<dyn AST>) -> Self {
        Self { block }
    }
}

pub struct ConditionalStatement {
    if_statement: IfStatement,
    elif_ladder: Vec<IfStatement>,
    else_statement: Option<ElseStatement>,
}

impl ConditionalStatement {
    pub fn new(
        if_statement: IfStatement,
        elif_ladder: Vec<IfStatement>,
        else_statement: Option<ElseStatement>,
    ) -> Self {
        Self {
            if_statement,
            elif_ladder,
            else_statement,
        }
    }
}

impl AST for ConditionalStatement {
    fn visit(&self) -> VisitResult {
        if let TokenEnum::Bool(value) = *self.if_statement.condition.visit().token {
            if value {
                return self.if_statement.block.visit();
            }
        }

        for elif in &self.elif_ladder {
            if let TokenEnum::Bool(value) = *elif.condition.visit().token {
                if value {
                    return self.if_statement.block.visit();
                }
            }
        }

        if let Some(else_statement) = &self.else_statement {
            return else_statement.block.visit();
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
