use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, VariableHashMap},
    lexer::tokens::{Number, TokenEnum},
    trace,
};
use std::{cell::RefCell, rc::Rc};

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct Loop {
    /// an expression
    from_range: Rc<Box<dyn AST>>,
    /// an expression
    to_range: Rc<Box<dyn AST>>,
    step_by: Rc<Box<dyn AST>>,
    block: Rc<Box<dyn AST>>,
}

impl Loop {
    pub fn new(
        from_range: Rc<Box<dyn AST>>,
        to_range: Rc<Box<dyn AST>>,
        step_by: Rc<Box<dyn AST>>,
        block: Rc<Box<dyn AST>>,
    ) -> Self {
        Self {
            from_range,
            to_range,
            step_by,
            block,
        }
    }
}

impl AST for Loop {
    fn visit_com(&self, v: &mut VariableHashMap, f: Rc<RefCell<Functions>>, asm: &mut ASM) {
        // 1. Visit the from expression, to expression and step expression if they exist. Push
        //    them onto the stack
        //
        // 2. On every loop iteration, we pop these into r0, r1 and r2 and perform the step
        //    operation
        //
        // 3. Compare if the current addition value is equal to the `to` value, and if they are
        //    equal break the loop

        let current_num_loop = asm.num_loops;
        asm.inc_num_loops();

        self.from_range.visit_com(v, Rc::clone(&f), asm);
        self.to_range.visit_com(v, Rc::clone(&f), asm);
        self.step_by.visit_com(v, Rc::clone(&f), asm);

        asm.gen_loop_start(current_num_loop);
        self.block.visit_com(v, Rc::clone(&f), asm);
        asm.gen_loop_end(current_num_loop);
    }

    fn visit(&self, v: &mut VariableHashMap, f: Rc<RefCell<Functions>>) -> VisitResult {
        let from = self.from_range.visit(v, Rc::clone(&f));
        let to = self.to_range.visit(v, Rc::clone(&f));
        let step_by = self.step_by.visit(v, Rc::clone(&f));

        if !from.token.is_integer() || !to.token.is_integer() || !step_by.token.is_integer() {
            panic!("Expected from, to and step expressions to be Integer");
        }

        let from = if let TokenEnum::Number(Number::Integer(i)) = *from.token {
            i
        } else {
            unreachable!("Somehow did not get integer even after performing Integer enum check")
        };

        let to = if let TokenEnum::Number(Number::Integer(i)) = *to.token {
            i
        } else {
            unreachable!("Somehow did not get integer even after performing Integer enum check")
        };

        let step_by = if let TokenEnum::Number(Number::Integer(i)) = *step_by.token {
            if i < 0 {
                panic!("Step cannot be negative");
            }

            i as usize
        } else {
            panic!("Step has to be a positive integer")
        };

        for _ in (from..to).step_by(step_by) {
            self.block.visit(v, Rc::clone(&f));
        }

        return VisitResult {
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
