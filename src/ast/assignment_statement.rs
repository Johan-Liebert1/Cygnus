use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, VarScope, VariableHashMap, VariableHashMapValue},
    lexer::tokens::{TokenEnum, VariableEnum},
    trace,
};

use super::abstract_syntax_tree::{VisitResult, AST};

#[derive(Debug)]
pub struct AssignmentStatement {
    var_name: String,
    var_value: Option<VariableHashMapValue>,
    right: Rc<Box<dyn AST>>,
}

impl AssignmentStatement {
    pub fn new(var_name: String, var_value: Option<VariableHashMapValue>, right: Rc<Box<dyn AST>>) -> Self {
        Self { var_name, var_value, right }
    }
}

impl AST for AssignmentStatement {
    fn visit_com(&self, v: &mut VariableHashMap, f: Rc<RefCell<Functions>>, asm: &mut ASM) {
        self.right.visit_com(v, f, asm);
        asm.variable_assignment(&self.var_name);
    }

    // TODO: change this so that the expression is stored here and we need to visit the varible
    // to evaluate the value
    fn visit(&self, vars: &mut VariableHashMap, f: Rc<RefCell<Functions>>) -> VisitResult {
        let right_visit = self.right.visit(vars, f);

        match &*right_visit.token {
            TokenEnum::StringLiteral(s) => {
                vars.insert(
                    self.var_name.clone(),
                    VariableHashMapValue {
                        var: VariableEnum::String(s.into()),
                        scope: VarScope::Global,
                        index: 0,
                    },
                );
            }

            TokenEnum::Number(n) => {
                vars.insert(
                    self.var_name.clone(),
                    VariableHashMapValue {
                        var: VariableEnum::Number(n.clone()),
                        scope: VarScope::Global,
                        index: 0,
                    },
                );
            }

            TokenEnum::Variable(_) => todo!(),

            _ => {
                panic!("Variable value is not a Number, String or Variable");
            }
        }

        return VisitResult {
            token: right_visit.token,
        };
    }

    fn get_token(&self) -> &crate::lexer::lexer::Token {
        todo!()
    }

    fn print(&self) {
        trace!("{:#?}", self)
    }

    fn type_check(&self, call_stack: &crate::semantic::semantic_analyzer::CallStackRecord) {
        todo!()
    }
}
