use crate::{
    interpreter::interpreter::{Functions, Variables},
    lexer::tokens::TokenEnum, asm::asm::ASM,
};
use std::{cell::RefCell, rc::Rc};

use super::abstract_syntax_tree::{VisitResult, AST};

pub struct IfStatement {
    condition: Rc<Box<dyn AST>>,
    block: Rc<Box<dyn AST>>,
}

impl IfStatement {
    pub fn new(condition: Rc<Box<dyn AST>>, block: Rc<Box<dyn AST>>) -> Self {
        Self { condition, block }
    }
}

pub struct ElseStatement {
    block: Rc<Box<dyn AST>>,
}

impl ElseStatement {
    pub fn new(block: Rc<Box<dyn AST>>) -> Self {
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
    fn visit_com(&self, x: &mut Variables, _: Rc<RefCell<Functions>>, asm: &mut ASM) {
        todo!()
    }

    fn visit(&self, i: &mut Variables, f: Rc<RefCell<Functions>>) -> VisitResult {
        if let TokenEnum::Bool(value) = *self.if_statement.condition.visit(i, Rc::clone(&f)).token {
            if value {
                return self.if_statement.block.visit(i, Rc::clone(&f));
            }
        }

        for elif in &self.elif_ladder {
            if let TokenEnum::Bool(value) = *elif.condition.visit(i, Rc::clone(&f)).token {
                if value {
                    return elif.block.visit(i, Rc::clone(&f));
                }
            }
        }

        // TODO: Panic if not boolean
        if let Some(else_statement) = &self.else_statement {
            return else_statement.block.visit(i, f);
        }

        return VisitResult {
            // TODO: Think of a better token for unexecuted statements
            token: Box::new(TokenEnum::Unknown("".into())),
        };
    }

    fn get_token(&self) -> &crate::lexer::lexer::Token {
        todo!()
    }

    fn print(&self) {
        todo!()
    }
}
