use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{lexer::Token, tokens::TokenEnum},
    semantic_analyzer::semantic_analyzer::CallStack,
};

use super::{assignment_statement::AssignmentStatement, variable::Variable, program::Program, logical_exp::LogicalExpression, jump::Jump, function_def::FunctionDefinition, function_call::FunctionCall, factor::Factor, declaration_statement::DeclarationStatement, conditionals::ConditionalStatement, comparison_exp::ComparisonExp, binary_op::BinaryOP, ast_loop::Loop};

#[derive(Debug)]
pub struct VisitResult {
    pub token: Box<TokenEnum>,
}

pub trait AST {
    fn visit(
        &self,
        v: &mut Variables,
        f: Rc<RefCell<Functions>>,
        call_stack: &mut CallStack,
    ) -> VisitResult;
    fn visit_com(
        &self,
        v: &mut Variables,
        f: Rc<RefCell<Functions>>,
        asm: &mut ASM,
        call_stack: &mut CallStack,
    );
    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>);
    fn get_token(&self) -> &Token;
    fn get_node(&self) -> ASTNodeEnum;
    fn print(&self);
}

impl Debug for dyn AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.print())
    }
}

pub enum ASTNodeEnum<'a> {
  AssignmentStatement(&'a AssignmentStatement),
  Loop(&'a Loop),
  BinaryOp(&'a BinaryOP),
  ComparisonExp(&'a ComparisonExp),
  Conditionals(&'a ConditionalStatement),
  DeclarationStatement(&'a DeclarationStatement),
  Factor(&'a Factor),
  FunctionCall(&'a FunctionCall),
  FunctionDef(&'a FunctionDefinition),
  Jump(&'a Jump),
  LogicalExp(&'a LogicalExpression),
  Program(&'a Program),
  Variable(&'a Variable),
}
