use crate::{
    helpers::compiler_error,
    lexer::{lexer::Token, types::VarType},
    semantic_analyzer::semantic_analyzer::{CallStack, PopTypes},
    trace,
    types::ASTNode,
};

use std::{cell::RefCell, process::exit, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::tokens::TokenEnum,
};

use super::abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut, VisitResult, AST};

#[derive(Debug)]
pub enum JumpType {
    Return,
    Break,
    Continue,
}

#[derive(Debug)]
pub struct Jump {
    pub typ: JumpType,
    loop_number: usize,
    function_name: Option<String>,
    return_node: Option<ASTNode>,
    token: Token,
    pub result_type: VarType,
}

impl Jump {
    pub fn new(
        typ: JumpType,
        loop_number: usize,
        function_name: Option<String>,
        return_node: Option<ASTNode>,
        token: Token,
    ) -> Self {
        Self {
            typ,
            loop_number,
            function_name,
            return_node,
            token,
            result_type: VarType::Unknown,
        }
    }
}

impl AST for Jump {
    fn visit(&self, _v: &mut Variables, _f: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
        todo!();

        // this is pretty straightforward. We simply return
        return VisitResult {
            token: Box::new(TokenEnum::Unknown("".into())),
        };
    }

    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        let mut return_value_exists = false;

        if let Some(ast_node) = &self.return_node {
            ast_node.borrow_mut().visit_com(v, f, asm, call_stack);
            return_value_exists = true;
        }

        match self.typ {
            JumpType::Return => {
                // Since we break out of a loop or return from a function, we need to pop the call stack
                // call_stack.pop_special(PopTypes::EarlyReturn);
                asm.function_return(return_value_exists)
            }

            JumpType::Break => {
                // Since we break out of a loop or return from a function, we need to pop the call stack
                // call_stack.pop_special(PopTypes::LoopBreak);
                asm.loop_break(self.loop_number)
            }

            JumpType::Continue => asm.loop_continue(self.loop_number),
        }
    }

    fn get_token(&self) -> &crate::lexer::lexer::Token {
        todo!()
    }

    fn print(&self) {
        todo!()
    }

    // TODO: Figure out if this matters
    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        if let JumpType::Return = self.typ {
            // Handle return

            // This unwrap should be fine as the function definition will have checked for it
            let name: String = self.function_name.as_ref().unwrap().into();

            if let Some(ast_node) = &self.return_node {
                let func_struct = f.borrow();
                let func_struct = func_struct.get(&name).unwrap();

                ast_node.borrow_mut().semantic_visit(call_stack, f.clone());

                let ast_node_type = ast_node.borrow().get_node().get_result_type().clone();

                if func_struct.return_type != ast_node_type {
                    compiler_error(
                        format!(
                            "Function '{}' return type is defined as '{}', this return expression evaluates to\
                                '{}'",
                            name, func_struct.return_type, ast_node_type
                        ),
                        &self.token,
                    );
                    exit(1);
                }
            } else {
                return;
            }
        }

        // Since we break out of a loop or return from a function, we need to pop the call stack
        // match self.typ {
        //     JumpType::Return => {
        //         // Since we break out of a loop or return from a function, we need to pop the call stack
        //         call_stack.pop_special(PopTypes::EarlyReturn);
        //     }

        //     JumpType::Break => {
        //         // Since we break out of a loop or return from a function, we need to pop the call stack
        //         call_stack.pop_special(PopTypes::LoopBreak);
        //     }
        // }
    }

    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::Jump(&self);
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::Jump(self);
    }

    fn get_type(&self) -> (VarType, VarType) {
        return (
            self.result_type.get_actual_type(0, &self.token),
            self.result_type.clone(),
        );
    }
}
