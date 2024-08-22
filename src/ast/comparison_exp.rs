use crate::lexer::tokens::AllOperations;
use crate::lexer::types::VarType;
use crate::types::ASTNode;

use crate::semantic_analyzer::semantic_analyzer::CallStack;

use crate::{
    asm::asm::ASM,
    constants,
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        lexer::Token,
        tokens::{Comparators, Number, Operand, TokenEnum, VariableEnum},
    },
};
use std::{cell::RefCell, rc::Rc};

use super::abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut, VisitResult, AST};

#[derive(Debug)]
pub struct ComparisonExp {
    left: ASTNode,
    comp_op: Box<Token>,
    right: ASTNode,
    pub result_type: VarType,
}

impl ComparisonExp {
    pub fn new(left: ASTNode, comp_op: Box<Token>, right: ASTNode) -> Self {
        Self {
            left,
            comp_op,
            right,
            result_type: VarType::Int,
        }
    }

    fn compare<T>(&self, l: T, r: T) -> TokenEnum
    where
        T: PartialOrd,
    {
        return TokenEnum::Bool(match &self.comp_op.token {
            TokenEnum::Comparator(comp) => match comp {
                Comparators::LessThan => l < r,
                Comparators::GreaterThan => l > r,
                Comparators::LessThanEq => l <= r,
                Comparators::GreaterThanEq => l >= r,
                Comparators::DoubleEquals => l == r,
                Comparators::NotEquals => l != r,
            },

            _ => {
                unreachable!("Found non comparator")
            }
        });
    }

    fn eval_number_number(&self, left_op: &Number, right_op: &Number) -> VisitResult {
        match (left_op, right_op) {
            (Number::Integer(l), Number::Integer(r)) => {
                return VisitResult {
                    token: Box::new(self.compare(*l, *r)),
                };
            }

            (Number::Float(l), Number::Float(r)) => {
                return VisitResult {
                    token: Box::new(self.compare(*l, *r)),
                };
            }

            _ => {
                panic!("Cannot compare Float and Integer");
            }
        }
    }

    fn eval_var_num(&self, number: &Number, variable: &String, i: &mut Variables) -> VisitResult {
        let result = i.get(variable);

        match result {
            Some(var_num) => match var_num {
                VariableEnum::Number(var_num) => self.eval_number_number(number, var_num),
                VariableEnum::String(_) => todo!(),
                VariableEnum::Pointer(_) => todo!(),
            },

            None => panic!("Variable {} is not defined", variable),
        }
    }

    fn eval_var_var(&self, var1: &String, var2: &String, i: &mut Variables) -> VisitResult {
        let r1 = i.get(var1);
        let r2 = i.get(var2);

        match (r1, r2) {
            (Some(var1), Some(var2)) => match (var1, var2) {
                (VariableEnum::Number(var1), VariableEnum::Number(var2)) => self.eval_number_number(var1, var2),

                (VariableEnum::Number(_), VariableEnum::String(_)) => todo!(),
                (VariableEnum::String(_), VariableEnum::Number(_)) => todo!(),
                (VariableEnum::String(_), VariableEnum::String(_)) => todo!(),
                (VariableEnum::Number(_), VariableEnum::Pointer(_)) => todo!(),
                (VariableEnum::String(_), VariableEnum::Pointer(_)) => todo!(),
                (VariableEnum::Pointer(_), VariableEnum::Number(_)) => todo!(),
                (VariableEnum::Pointer(_), VariableEnum::String(_)) => todo!(),
                (VariableEnum::Pointer(_), VariableEnum::Pointer(_)) => todo!(),
            },

            (None, Some(_)) => panic!("Variable {} is not defined", var1),
            (Some(_), None) => panic!("Variable {} is not defined", var2),
            (None, None) => panic!("Variable {} and {} is not defined", var1, var2),
        }
    }

    fn evaluate_operands(&self, left_op: &Operand, right_op: &Operand, i: &mut Variables) -> VisitResult {
        match (left_op, right_op) {
            (Operand::Number(left_op), Operand::Number(right_op)) => self.eval_number_number(left_op, right_op),

            (Operand::Number(n), Operand::Variable(v)) => self.eval_var_num(n, v, i),
            (Operand::Variable(v), Operand::Number(n)) => self.eval_var_num(n, v, i),

            (Operand::Variable(v1), Operand::Variable(v2)) => self.eval_var_var(v1, v2, i),
        }
    }
}

impl AST for ComparisonExp {
    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        self.left.borrow().visit_com(v, Rc::clone(&f), asm, call_stack);
        self.right.borrow().visit_com(v, Rc::clone(&f), asm, call_stack);

        match &self.comp_op.token {
            TokenEnum::Comparator(c) => {
                asm.compare_two_numbers(c.clone(), &self.left.borrow().get_type().1);
            }

            _ => unreachable!("Found non comparator for a Comparison Expression"),
        }
    }

    fn visit(&self, i: &mut Variables, f: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
        if constants::DEBUG_AST {
            println!("{:#?}", &self);
            println!("===============================================");
        }

        let visit_left = self.left.borrow().visit(i, Rc::clone(&f), call_stack);
        let visit_right = self.right.borrow().visit(i, Rc::clone(&f), call_stack);

        let left_operand = visit_left.token.get_operand();
        let right_operand = visit_right.token.get_operand();

        match (&left_operand, &right_operand) {
            (Ok(lop), Ok(rop)) => {
                // Handle the case where both operands are Ok
                return self.evaluate_operands(lop, rop, i);
            }

            (Err(err), _) => {
                // Handle the case where left_operand is an error
                panic!("{}", err);
            }

            (_, Err(err)) => {
                // Handle the case where right_operand is an error
                panic!("{}", err);
            }
        };
    }

    fn get_token(&self) -> &Token {
        return &self.comp_op;
    }

    fn print(&self) {
        println!("{:#?}", self);
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        self.left.borrow_mut().semantic_visit(call_stack, f.clone());
        self.right.borrow_mut().semantic_visit(call_stack, f);

        if let TokenEnum::Comparator(op) = &self.comp_op.token {
            // need to do this even though it's always going to be an int
            self.result_type = self
                .left
                .borrow()
                .get_node()
                .figure_out_type(&self.right.borrow().get_node(), AllOperations::Comparator(op.clone()));
        } else {
            unreachable!(
                "Found Operation '{:?}' which is not defined for a comparison operation.\
            This must be a bug in the parsing step",
                self.comp_op.token
            )
        }
    }

    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::ComparisonExp(&self);
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::ComparisonExp(self);
    }

    fn get_type(&self) -> (VarType, VarType) {
        return (
            self.result_type.get_actual_type(0, &self.comp_op),
            self.result_type.clone(),
        );
    }
}
