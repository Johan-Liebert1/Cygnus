use crate::helpers::compiler_error;
use crate::lexer::tokens::Number;
use crate::lexer::types::VarType;
use crate::semantic_analyzer::semantic_analyzer::CallStack;

use crate::trace;
use crate::{
    asm::asm::ASM,
    constants,
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        lexer::Token,
        tokens::{TokenEnum, VariableEnum},
    },
};
use std::process::exit;
use std::{cell::RefCell, rc::Rc};

use super::abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut, VisitResult, AST};

/// FACTOR -> INTEGER | FLOAT
#[derive(Debug)]
pub struct Factor {
    token: Box<Token>,
    pub result_type: VarType,
}

impl Factor {
    pub fn new(token: Box<Token>) -> Self {
        Self {
            token,
            result_type: VarType::Unknown,
        }
    }

    pub fn get_type_factor(&self) -> &VarType {
        return &self.result_type;
    }
}

impl AST for Factor {
    fn visit_com(&self, x: &mut Variables, _: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        asm.generate_asm_factor(&self.token.token, x, call_stack);
    }

    fn visit(&self, v: &mut Variables, _: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
        if constants::DEBUG_AST {
            println!("{:?}", &self);
        }

        let token_enum = match &self.token.token {
            TokenEnum::Variable(var_name) => {
                if let Some(n) = v.get(var_name) {
                    match n {
                        VariableEnum::Number(n) => TokenEnum::Number(n.clone()),
                        VariableEnum::String(s) => TokenEnum::StringLiteral(s.to_string()),
                        VariableEnum::Pointer(_) => todo!(),
                    }
                } else {
                    compiler_error(format!("Variable with name '{var_name}' not found in current scope"), &self.token);
                    exit(1);
                }
            }

            t => t.clone(),
        };

        VisitResult {
            token: Box::new(token_enum),
        }
    }

    fn get_token(&self) -> &Token {
        return &self.token;
    }

    fn print(&self) {
        println!("{:#?}", self);
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, _f: Rc<RefCell<Functions>>) {
        self.result_type = match &self.token.token {
            TokenEnum::Variable(v) => {
                let (variable, _) = call_stack.get_var_with_name(v);

                if let Some(var) = variable {
                    var.var_type.get_actual_type(var.times_dereferenced).clone()
                } else {
                    compiler_error(format!("Variable with name '{v}' not found in current scope"), &self.token);
                    exit(1);
                }
            }

            TokenEnum::Number(num) => match num {
                Number::Integer(_) => VarType::Int,
                Number::Float(_) => VarType::Float,
            },

            TokenEnum::StringLiteral(..) => VarType::Str,

            _ => VarType::Unknown,
        };
    }

    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::Factor(&self);
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::Factor(self);
    }
}
