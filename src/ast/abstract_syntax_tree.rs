use core::{fmt, panic};
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
    array::Array, assignment_statement::AssignmentStatement, ast_loop::Loop, binary_op::BinaryOP,
    comparison_exp::ComparisonExp, conditionals::ConditionalStatement, declaration_statement::DeclarationStatement,
    factor::Factor, function_call::FunctionCall, function_def::FunctionDefinition, jump::Jump,
    logical_exp::LogicalExpression, memory_alloc::MemoryAlloc, program::Program, structs::StructDecleration,
    variable::Variable,
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
    /// Returns (actual_type, result_type)
    /// actual_type = type after all dereferences have been applied
    fn get_type(&self) -> (VarType, VarType);
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
    Array(&'a mut Array),
    Struct(&'a mut StructDecleration),
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
    Array(&'a Array),
    Struct(&'a StructDecleration),
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
            ASTNodeEnumMut::Array(_) => "Array",
            ASTNodeEnumMut::Struct(_) => "Struct",
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
            ASTNodeEnum::Array(_) => "Array",
            ASTNodeEnum::Struct(_) => "Struct",
        };

        write!(f, "{}", name)
    }
}

impl<'a> Debug for ASTNodeEnum<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNodeEnum::AssignmentStatement(a) => write!(f, "Name: AssignmentStatement {:#?}", a),
            ASTNodeEnum::Loop(a) => write!(f, "Name: Loop {:#?}", a),
            ASTNodeEnum::BinaryOp(a) => write!(f, "Name: BinaryOp {:#?}", a),
            ASTNodeEnum::ComparisonExp(a) => write!(f, "Name: ComparisonExp {:#?}", a),
            ASTNodeEnum::Conditionals(a) => write!(f, "Name: Conditionals {:#?}", a),
            ASTNodeEnum::DeclarationStatement(a) => write!(f, "Name: DeclarationStatement {:#?}", a),
            ASTNodeEnum::Factor(a) => write!(f, "Name: Factor {:#?}", a),
            ASTNodeEnum::FunctionCall(a) => write!(f, "Name: FunctionCall {:#?}", a),
            ASTNodeEnum::FunctionDef(a) => write!(f, "Name: FunctionDef {:#?}", a),
            ASTNodeEnum::Jump(a) => write!(f, "Name: Jump {:#?}", a),
            ASTNodeEnum::LogicalExp(a) => write!(f, "Name: LogicalExp {:#?}", a),
            ASTNodeEnum::Program(a) => write!(f, "Name: Program {:#?}", a),
            ASTNodeEnum::Variable(a) => write!(f, "Name: Variable {:#?}", a),
            ASTNodeEnum::MemoryAlloc(a) => write!(f, "Name: MemoryAlloc {:#?}", a),
            ASTNodeEnum::Array(a) => write!(f, "Name: Array {:#?}", a),
            ASTNodeEnum::Struct(a) => write!(f, "Name: Struct {:#?}", a),
        }
    }
}

impl<'a> ASTNodeEnum<'a> {
    pub fn figure_out_type(&self, other: &ASTNodeEnum, op: AllOperations) -> VarType {
        use ASTNodeEnum::*;

        match (self, other) {
            (BinaryOp(a), BinaryOp(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),
            (Factor(a), Factor(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),

            (FunctionCall(a), Variable(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),
            (LogicalExp(a), LogicalExp(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),
            (Variable(a), Variable(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),

            (BinaryOp(a), FunctionCall(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),
            (FunctionCall(a), BinaryOp(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),

            (BinaryOp(a), LogicalExp(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),
            (LogicalExp(a), BinaryOp(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),

            (BinaryOp(a), Variable(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),
            (Variable(a), BinaryOp(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),

            (BinaryOp(a), Factor(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),
            (Factor(a), BinaryOp(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),

            (Factor(a), FunctionCall(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),
            (FunctionCall(a), Factor(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),

            (Factor(a), LogicalExp(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),
            (LogicalExp(a), Factor(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),

            (Factor(a), Variable(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),
            (Variable(a), Factor(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),

            (FunctionCall(a), FunctionCall(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),
            (Variable(a), FunctionCall(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),

            (FunctionCall(a), LogicalExp(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),
            (LogicalExp(a), FunctionCall(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),

            (LogicalExp(a), Variable(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),
            (Variable(a), LogicalExp(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),

            (ComparisonExp(a), Factor(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),
            (Factor(a), ComparisonExp(b)) => a.get_type().0.figure_out_type(&b.get_type().0, op),

            (a, b) => unreachable!("This must be a bug in the parsing step. {a} and {b} not handled"),
        }
    }

    pub fn is_var_assignment_okay(&self, variable: &Variable) -> (bool, VarType) {
        use ASTNodeEnum::*;

        return match self {
            Factor(f) => {
                let (actual_type, result_type) = f.get_type();
                (actual_type.can_assign(&variable.get_type().0), actual_type)
            }

            BinaryOp(f) => {
                let (actual_type, result_type) = f.get_type();
                (actual_type.can_assign(&variable.get_type().0), actual_type)
            }
            ComparisonExp(f) => {
                let (actual_type, result_type) = f.get_type();
                (actual_type.can_assign(&variable.get_type().0), actual_type)
            }
            FunctionCall(f) => {
                let (actual_type, result_type) = f.get_type();
                (actual_type.can_assign(&variable.get_type().0), actual_type)
            }
            LogicalExp(f) => {
                let (actual_type, result_type) = f.get_type();
                (actual_type.can_assign(&variable.get_type().0), actual_type)
            }
            Variable(f) => {
                let (actual_type, result_type) = f.get_type();

                // trace!("\n\n=======================================\nf's actual_type: {:#?}, result_type: {:#?}", actual_type, result_type);
                // trace!("variable's actual_type: {:#?}, result_type: {:#?}", variable.get_type().0, variable.get_type().1);

                (actual_type.can_assign(&variable.get_type().0), actual_type)
            }
            MemoryAlloc(f) => {
                let (actual_type, result_type) = f.get_type();
                (actual_type.can_assign(&variable.get_type().0), actual_type)
            }
            Array(f) => {
                let (actual_type, result_type) = f.get_type();
                (actual_type.can_assign(&variable.get_type().0), actual_type)
            }

            Struct(f) => {
                let (actual_type, result_type) = f.get_type();
                (actual_type.can_assign(&variable.get_type().0), actual_type)
            }

            node => unreachable!("Cannot assign a variable to {node}. This could a bug in the parsing stage"),
        };
    }

    pub fn get_result_type(&self) -> &VarType {
        match self {
            ASTNodeEnum::Factor(node) => &node.result_type,
            ASTNodeEnum::BinaryOp(node) => &node.result_type,
            ASTNodeEnum::ComparisonExp(node) => &node.result_type,
            ASTNodeEnum::FunctionCall(node) => &node.result_type,
            ASTNodeEnum::LogicalExp(node) => &node.result_type,
            ASTNodeEnum::Variable(node) => &node.result_type,
            ASTNodeEnum::MemoryAlloc(node) => &node.result_type,
            ASTNodeEnum::Jump(node) => &node.result_type,
            ASTNodeEnum::Array(node) => &node.result_type,

            ASTNodeEnum::Struct(_) => todo!(),
            ASTNodeEnum::AssignmentStatement(_) => todo!(),
            ASTNodeEnum::Loop(_) => todo!(),
            ASTNodeEnum::Conditionals(_) => todo!(),
            ASTNodeEnum::DeclarationStatement(_) => todo!(),
            ASTNodeEnum::FunctionDef(_) => todo!(),
            ASTNodeEnum::Program(_) => todo!(),
        }
    }
}
