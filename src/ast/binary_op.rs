use crate::{
    constants,
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        lexer::Token,
        tokens::{Number, Operand, Operations, TokenEnum},
    },
};
use std::{cell::RefCell, rc::Rc};

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct BinaryOP {
    left: Rc<Box<dyn AST>>,
    operator: Box<Token>,
    right: Rc<Box<dyn AST>>,
}

impl BinaryOP {
    pub fn new(left: Rc<Box<dyn AST>>, operator: Box<Token>, right: Rc<Box<dyn AST>>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }

    fn evaluate<T>(&self, l: T, r: T) -> T
    where
        T: std::ops::Add<Output = T>,
        T: std::ops::Sub<Output = T>,
        T: std::ops::Mul<Output = T>,
        T: std::ops::Div<Output = T>,
    {
        match &self.operator.token {
            TokenEnum::Op(op) => match op {
                Operations::Plus => l + r,
                Operations::Minus => l - r,
                Operations::Divide => l / r,
                Operations::Multiply => l * r,
            },

            _ => {
                unreachable!("WTF!!")
            }
        }
    }

    fn eval_number_number(&self, left_op: &Number, right_op: &Number) -> VisitResult {
        match (left_op, right_op) {
            (Number::Integer(l), Number::Integer(r)) => VisitResult {
                token: Box::new(TokenEnum::new_integer(self.evaluate(*l, *r))),
            },

            (Number::Float(l), Number::Float(r)) => VisitResult {
                token: Box::new(TokenEnum::new_float(self.evaluate(*l, *r))),
            },

            _ => {
                panic!("Cannot add Float and Integer");
            }
        }
    }

    fn eval_var_num(&self, number: &Number, variable: &String, i: &mut Variables) -> VisitResult {
        let result = i.get(variable);

        match result {
            Some(var_num) => self.eval_number_number(number, var_num),

            None => panic!("Variable {} is not defined", variable),
        }
    }

    fn eval_var_var(&self, var1: &String, var2: &String, i: &mut Variables) -> VisitResult {
        let r1 = i.get(var1);
        let r2 = i.get(var2);

        match (r1, r2) {
            (Some(var1), Some(var2)) => self.eval_number_number(var1, var2),

            (None, Some(_)) => panic!("Variable {} is not defined", var2),
            (Some(_), None) => panic!("Variable {} is not defined", var1),
            (None, None) => panic!("Variable {} and {} is not defined", var1, var2),
        }
    }

    fn evaluate_operands(
        &self,
        left_op: &Operand,
        right_op: &Operand,
        i: &mut Variables,
    ) -> VisitResult {
        match (left_op, right_op) {
            (Operand::Number(left_op), Operand::Number(right_op)) => {
                self.eval_number_number(left_op, right_op)
            }

            (Operand::Number(n), Operand::Variable(v)) => self.eval_var_num(n, v, i),
            (Operand::Variable(v), Operand::Number(n)) => self.eval_var_num(n, v, i),

            (Operand::Variable(v1), Operand::Variable(v2)) => self.eval_var_var(v1, v2, i),
        }
    }
}

impl AST for BinaryOP {
    fn visit(&self, i: &mut Variables, f: Rc<RefCell<Functions>>) -> VisitResult {
        if constants::DEBUG_AST {
            println!("{:#?}", &self);
            println!("===============================================");
        }

        let visit_left = self.left.visit(i, Rc::clone(&f));
        let visit_right = self.right.visit(i, Rc::clone(&f));

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
        return &self.operator;
    }

    fn print(&self) {
        println!("{:#?}", self);
    }
}
