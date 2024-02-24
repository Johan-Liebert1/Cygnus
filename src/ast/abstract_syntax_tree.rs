use core::fmt;
use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        lexer::Token,
        tokens::{AllOperations, TokenEnum},
        types::VarType,
    },
    semantic_analyzer::semantic_analyzer::CallStack,
    trace,
};

use super::{
    assignment_statement::AssignmentStatement, ast_loop::Loop, binary_op::BinaryOP, comparison_exp::ComparisonExp,
    conditionals::ConditionalStatement, declaration_statement::DeclarationStatement, factor::Factor,
    function_call::FunctionCall, function_def::FunctionDefinition, jump::Jump, logical_exp::LogicalExpression,
    memory_alloc::MemoryAlloc, program::Program, variable::Variable,
};

#[derive(Debug)]
pub struct VisitResult {
    pub token: Box<TokenEnum>,
}

pub trait AST {
    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult;
    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack);
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
    MemoryAlloc(&'a mut MemoryAlloc),
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
    MemoryAlloc(&'a MemoryAlloc),
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
            ASTNodeEnumMut::MemoryAlloc(_) => "MemoryAlloc",
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
            ASTNodeEnum::MemoryAlloc(_) => "MemoryAlloc",
        };

        write!(f, "{}", name)
    }
}

impl<'a> Debug for ASTNodeEnum<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl<'a> ASTNodeEnum<'a> {
    pub fn figure_out_type(&self, other: &ASTNodeEnum, op: AllOperations) -> VarType {
        match (self, other) {
            (ASTNodeEnum::BinaryOp(a), ASTNodeEnum::BinaryOp(b)) => a.result_type.figure_out_type(&b.result_type, op),
            (ASTNodeEnum::Factor(a), ASTNodeEnum::Factor(b)) => a.result_type.figure_out_type(&b.result_type, op),

            (ASTNodeEnum::FunctionCall(a), ASTNodeEnum::Variable(b)) => {
                a.result_type.figure_out_type(&b.result_type, op)
            }
            (ASTNodeEnum::LogicalExp(a), ASTNodeEnum::LogicalExp(b)) => {
                a.result_type.figure_out_type(&b.result_type, op)
            }
            (ASTNodeEnum::Variable(a), ASTNodeEnum::Variable(b)) => a.result_type.figure_out_type(&b.result_type, op),

            (ASTNodeEnum::BinaryOp(a), ASTNodeEnum::FunctionCall(b)) => {
                a.result_type.figure_out_type(&b.result_type, op)
            }
            (ASTNodeEnum::FunctionCall(a), ASTNodeEnum::BinaryOp(b)) => {
                a.result_type.figure_out_type(&b.result_type, op)
            }

            (ASTNodeEnum::BinaryOp(a), ASTNodeEnum::LogicalExp(b)) => a.result_type.figure_out_type(&b.result_type, op),
            (ASTNodeEnum::LogicalExp(a), ASTNodeEnum::BinaryOp(b)) => a.result_type.figure_out_type(&b.result_type, op),

            (ASTNodeEnum::BinaryOp(a), ASTNodeEnum::Variable(b)) => a.result_type.figure_out_type(&b.result_type, op),
            (ASTNodeEnum::Variable(a), ASTNodeEnum::BinaryOp(b)) => a.result_type.figure_out_type(&b.result_type, op),

            (ASTNodeEnum::BinaryOp(a), ASTNodeEnum::Factor(b)) => a.result_type.figure_out_type(&b.result_type, op),
            (ASTNodeEnum::Factor(a), ASTNodeEnum::BinaryOp(b)) => a.result_type.figure_out_type(&b.result_type, op),

            (ASTNodeEnum::Factor(a), ASTNodeEnum::FunctionCall(b)) => a.result_type.figure_out_type(&b.result_type, op),
            (ASTNodeEnum::FunctionCall(a), ASTNodeEnum::Factor(b)) => a.result_type.figure_out_type(&b.result_type, op),

            (ASTNodeEnum::Factor(a), ASTNodeEnum::LogicalExp(b)) => a.result_type.figure_out_type(&b.result_type, op),
            (ASTNodeEnum::LogicalExp(a), ASTNodeEnum::Factor(b)) => a.result_type.figure_out_type(&b.result_type, op),

            (ASTNodeEnum::Factor(a), ASTNodeEnum::Variable(b)) => a.result_type.figure_out_type(&b.result_type, op),
            (ASTNodeEnum::Variable(a), ASTNodeEnum::Factor(b)) => a.result_type.figure_out_type(&b.result_type, op),

            (ASTNodeEnum::FunctionCall(a), ASTNodeEnum::FunctionCall(b)) => {
                a.result_type.figure_out_type(&b.result_type, op)
            }
            (ASTNodeEnum::Variable(a), ASTNodeEnum::FunctionCall(b)) => {
                a.result_type.figure_out_type(&b.result_type, op)
            }

            (ASTNodeEnum::FunctionCall(a), ASTNodeEnum::LogicalExp(b)) => {
                a.result_type.figure_out_type(&b.result_type, op)
            }
            (ASTNodeEnum::LogicalExp(a), ASTNodeEnum::FunctionCall(b)) => {
                a.result_type.figure_out_type(&b.result_type, op)
            }

            (ASTNodeEnum::LogicalExp(a), ASTNodeEnum::Variable(b)) => a.result_type.figure_out_type(&b.result_type, op),
            (ASTNodeEnum::Variable(a), ASTNodeEnum::LogicalExp(b)) => a.result_type.figure_out_type(&b.result_type, op),

            (ASTNodeEnum::ComparisonExp(a), ASTNodeEnum::Factor(b)) => {
                a.result_type.figure_out_type(&b.result_type, op)
            }
            (ASTNodeEnum::Factor(a), ASTNodeEnum::ComparisonExp(b)) => {
                a.result_type.figure_out_type(&b.result_type, op)
            }

            (a, b) => unreachable!("This must be a bug in the parsing step. {a} and {b} not handled"),
        }
    }
}
