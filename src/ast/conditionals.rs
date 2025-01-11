use crate::lexer::types::VarType;
use crate::types::ASTNode;

use crate::semantic_analyzer::semantic_analyzer::{ActivationRecordType, CallStack};

use crate::{
    asm::{asm::ASM, conditionals::ConditionalJumpTo},
    interpreter::interpreter::{Functions, Variables},
    lexer::tokens::TokenEnum,
};
use std::{cell::RefCell, rc::Rc};

use super::abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut, VisitResult, AST};

#[derive(Debug)]
pub struct IfStatement {
    condition: ASTNode,
    block: ASTNode,
}

impl IfStatement {
    pub fn new(condition: ASTNode, block: ASTNode) -> Self {
        Self { condition, block }
    }
}

#[derive(Debug)]
pub struct ElseStatement {
    block: ASTNode,
}

impl ElseStatement {
    pub fn new(block: ASTNode) -> Self {
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
    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        let current_num_if = asm.num_ifs;
        asm.inc_num_ifs();

        self.if_statement
            .condition
            .borrow()
            .visit_com(v, Rc::clone(&f), asm, call_stack);

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

        call_stack.push("".into(), ActivationRecordType::IfElse);
        self.if_statement
            .block
            .borrow()
            .visit_com(v, Rc::clone(&f), asm, call_stack);
        call_stack.pop();

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
            call_stack.push("".into(), ActivationRecordType::IfElse);
            elif.condition.borrow().visit_com(v, Rc::clone(&f), asm, call_stack);
            call_stack.pop();

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

            call_stack.push("".into(), ActivationRecordType::IfElse);
            elif.block.borrow().visit_com(v, Rc::clone(&f), asm, call_stack);
            call_stack.pop();

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

            e.block.borrow().visit_com(v, Rc::clone(&f), asm, call_stack);

            asm.else_end(current_num_if);
        }
    }

    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
        if let TokenEnum::Bool(value) = *self
            .if_statement
            .condition
            .borrow()
            .visit(v, Rc::clone(&f), call_stack)
            .token
        {
            if value {
                return self.if_statement.block.borrow().visit(v, Rc::clone(&f), call_stack);
            }
        }

        for elif in &self.elif_ladder {
            if let TokenEnum::Bool(value) = *elif.condition.borrow().visit(v, Rc::clone(&f), call_stack).token {
                if value {
                    return elif.block.borrow().visit(v, Rc::clone(&f), call_stack);
                }
            }
        }

        // TODO: Panic if not boolean
        if let Some(else_statement) = &self.else_statement {
            return else_statement.block.borrow().visit(v, f, call_stack);
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
        println!("{:#?}", self);
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        self.if_statement
            .condition
            .borrow_mut()
            .semantic_visit(call_stack, Rc::clone(&f));

        call_stack.push("".into(), ActivationRecordType::IfElse);

        self.if_statement
            .block
            .borrow_mut()
            .semantic_visit(call_stack, Rc::clone(&f));

        call_stack.pop();

        for elif in &self.elif_ladder {
            elif.condition.borrow_mut().semantic_visit(call_stack, Rc::clone(&f));

            call_stack.push("".into(), ActivationRecordType::IfElse);

            elif.block.borrow_mut().semantic_visit(call_stack, Rc::clone(&f));

            call_stack.pop();
        }

        if let Some(else_statement) = &self.else_statement {
            call_stack.push("".into(), ActivationRecordType::IfElse);

            else_statement
                .block
                .borrow_mut()
                .semantic_visit(call_stack, Rc::clone(&f));

            call_stack.pop();
        }
    }

    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::Conditionals(&self);
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::Conditionals(self);
    }

    fn get_type(&self) -> (VarType, VarType) {
        return (VarType::Unknown, VarType::Unknown);
    }
}
