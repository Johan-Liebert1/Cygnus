use crate::{
    asm::{asm::ASM, conditionals::ConditionalJumpTo},
    interpreter::interpreter::{Functions, Variables},
    lexer::tokens::TokenEnum, trace,
};
use std::{cell::RefCell, rc::Rc};

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct IfStatement {
    condition: Rc<Box<dyn AST>>,
    block: Rc<Box<dyn AST>>,
}

impl IfStatement {
    pub fn new(condition: Rc<Box<dyn AST>>, block: Rc<Box<dyn AST>>) -> Self {
        Self { condition, block }
    }
}

#[derive(Debug)]
pub struct ElseStatement {
    block: Rc<Box<dyn AST>>,
}

impl ElseStatement {
    pub fn new(block: Rc<Box<dyn AST>>) -> Self {
        Self { block }
    }
}

#[derive(Debug)]
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
    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM) {
        let current_num_if = asm.num_ifs;
        asm.inc_num_ifs();

        self.if_statement.condition.visit_com(v, Rc::clone(&f), asm);

        asm.if_start(
            if self.elif_ladder.len() > 0 {
                // we jump to IfEnd as IfEnd label contains the comparison expression for elif
                ConditionalJumpTo::IfEnd
            } else if let Some(_) = &self.else_statement {
                ConditionalJumpTo::Else
            } else {
                ConditionalJumpTo::IfEnd
            },
            current_num_if,
        );

        self.if_statement.block.visit_com(v, Rc::clone(&f), asm);

        asm.if_end(
            if let Some(_) = self.else_statement {
                ConditionalJumpTo::Else
            } else if self.elif_ladder.len() > 0 {
                ConditionalJumpTo::ElifEnd
            } else {
                ConditionalJumpTo::IfEnd
            },
            self.elif_ladder.len(),
            current_num_if,
        );

        for (index, elif) in self.elif_ladder.iter().enumerate() {
            elif.condition.visit_com(v, Rc::clone(&f), asm);

            // if it's the last index, then jump to else, else jump to the next elif
            asm.elif_start(
                index,
                if index < self.elif_ladder.len() - 1 {
                    ConditionalJumpTo::ElifEnd
                } else if let Some(_) = &self.else_statement {
                    ConditionalJumpTo::Else
                } else {
                    ConditionalJumpTo::ElifEnd
                },
                current_num_if,
            );

            elif.block.visit_com(v, Rc::clone(&f), asm);

            asm.elif_end(
                index,
                if let Some(_) = self.else_statement {
                    ConditionalJumpTo::Else
                } else if self.elif_ladder.len() > 0 {
                    ConditionalJumpTo::ElifEnd
                } else {
                    ConditionalJumpTo::Else
                },
                current_num_if,
            );
        }

        if let Some(e) = &self.else_statement {
            asm.else_start(current_num_if);

            e.block.visit_com(v, Rc::clone(&f), asm);

            asm.else_end(current_num_if);
        }
    }

    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>) -> VisitResult {
        if let TokenEnum::Bool(value) = *self.if_statement.condition.visit(v, Rc::clone(&f)).token {
            if value {
                return self.if_statement.block.visit(v, Rc::clone(&f));
            }
        }

        for elif in &self.elif_ladder {
            if let TokenEnum::Bool(value) = *elif.condition.visit(v, Rc::clone(&f)).token {
                if value {
                    return elif.block.visit(v, Rc::clone(&f));
                }
            }
        }

        // TODO: Panic if not boolean
        if let Some(else_statement) = &self.else_statement {
            return else_statement.block.visit(v, f);
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
        trace!("{:#?}", self);
    }

    fn type_check(&self, call_stack: &crate::semantic::semantic_analyzer::CallStackRecord) {
        todo!()
    }
}
