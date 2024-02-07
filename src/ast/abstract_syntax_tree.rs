use core::fmt;
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
    fn get_node_mut(&mut self) -> ASTNodeEnumMut;
    fn print(&self);
}

impl Debug for dyn AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.print())
    }
}

pub enum ASTNodeEnumMut<'a> {
  AssignmentStatement(&'a mut AssignmentStatement),
  Loop(&'a mut Loop),
  BinaryOp(&'a mut BinaryOP),
  ComparisonExp(&'a mut ComparisonExp),
  Conditionals(&'a mut ConditionalStatement),
  DeclarationStatement(&'a mut DeclarationStatement),
  Factor(&'a mut Factor),
  FunctionCall(&'a mut FunctionCall),
  FunctionDef(&'a mut FunctionDefinition),
  Jump(&'a mut Jump),
  LogicalExp(&'a mut LogicalExpression),
  Program(&'a mut Program),
  Variable(&'a mut Variable),
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

impl<'a> Display for ASTNodeEnumMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ASTNodeEnumMut::AssignmentStatement(_) => "AssignmentStatementMut",
            ASTNodeEnumMut::Loop(_) => "LoopMut",
            ASTNodeEnumMut::BinaryOp(_) => "BinaryOpMut",
            ASTNodeEnumMut::ComparisonExp(_) => "ComparisonExpMut",
            ASTNodeEnumMut::Conditionals(_) => "ConditionalsMut",
            ASTNodeEnumMut::DeclarationStatement(_) => "DeclarationStatementMut",
            ASTNodeEnumMut::Factor(_) => "FactorMut",
            ASTNodeEnumMut::FunctionCall(_) => "FunctionCallMut",
            ASTNodeEnumMut::FunctionDef(_) => "FunctionDefMut",
            ASTNodeEnumMut::Jump(_) => "JumpMut",
            ASTNodeEnumMut::LogicalExp(_) => "LogicalExpMut",
            ASTNodeEnumMut::Program(_) => "ProgramMut",
            ASTNodeEnumMut::Variable(_) => "VariableMut",
        };

        write!(f, "{}", name)
    }
}

impl<'a> Debug for ASTNodeEnumMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}


impl<'a> Display for ASTNodeEnum<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ASTNodeEnum::AssignmentStatement(_) => "AssignmentStatement",
            ASTNodeEnum::Loop(_) => "Loop",
            ASTNodeEnum::BinaryOp(_) => "BinaryOp",
            ASTNodeEnum::ComparisonExp(_) => "ComparisonExp",
            ASTNodeEnum::Conditionals(_) => "Conditionals",
            ASTNodeEnum::DeclarationStatement(_) => "DeclarationStatement",
            ASTNodeEnum::Factor(_) => "Factor",
            ASTNodeEnum::FunctionCall(_) => "FunctionCall",
            ASTNodeEnum::FunctionDef(_) => "FunctionDef",
            ASTNodeEnum::Jump(_) => "Jump",
            ASTNodeEnum::LogicalExp(_) => "LogicalExp",
            ASTNodeEnum::Program(_) => "Program",
            ASTNodeEnum::Variable(_) => "Variable",
        };

        write!(f, "{}", name)
    }
}

impl<'a> Debug for ASTNodeEnum<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

