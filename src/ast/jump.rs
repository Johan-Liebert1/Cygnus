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
}

#[derive(Debug)]
pub struct Jump {
    typ: JumpType,
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

    fn visit_com(&self, _v: &mut Variables, _f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        match self.typ {
            JumpType::Return => {
                // Since we break out of a loop or return from a function, we need to pop the call stack
                // call_stack.pop_special(PopTypes::EarlyReturn);
                asm.function_return()
            }

            JumpType::Break => {
                // Since we break out of a loop or return from a function, we need to pop the call stack
                // call_stack.pop_special(PopTypes::LoopBreak);
                asm.loop_break(self.loop_number)
            }
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
        trace!("Retrun in func name: {:#?}", self.function_name);

        if let JumpType::Return = self.typ {
            // Handle return

            // This unwrap should be fine as the function definition will have checked for it
            let name: String = self.function_name.as_ref().unwrap().into();

            // FIXME: This will always panic as semantic_visit needs a mutable reference 
            // and this is called by semantic_visit of function_def which holds the mutable ref
            match f.borrow().get(&name).unwrap().to_owned().borrow().get_node() {
                ASTNodeEnum::FunctionDef(fd) => {
                    let mut ast_node_type = VarType::Unknown;

                    if let Some(ast_node) = &self.return_node {
                        ast_node.borrow_mut().semantic_visit(call_stack, f.clone());

                        ast_node_type = ast_node.borrow().get_node().get_result_type().clone();

                        if fd.return_type != ast_node_type {
                            compiler_error(
                                format!(
                                    "Function '{}' return type is defined as '{}', this return expression evaluates to\
                                '{}'",
                                    name, fd.return_type, ast_node_type
                                ),
                                &self.token,
                            );
                            exit(1);
                        }
                    } else {
                        return;
                    }
                }

                _ => unreachable!("Value in functions map was not a function definition"),
            };
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
}
