use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, VarScope, VariableHashMap, VariableHashMapValue},
    lexer::{
        lexer::Token,
        tokens::{Number, TokenEnum, VariableEnum},
    },
    trace,
};

use super::{
    abstract_syntax_tree::{VisitResult, AST},
    variable::VariableAST,
};

#[derive(Debug)]
pub struct DeclarationStatement {
    left: VariableAST,
    right: Rc<Box<dyn AST>>,
}

impl DeclarationStatement {
    pub fn new(left: VariableAST, right: Rc<Box<dyn AST>>) -> Self {
        Self { left, right }
    }
}

impl AST for DeclarationStatement {
    fn visit_com(&self, vars: &mut VariableHashMap, f: Rc<RefCell<Functions>>, asm: &mut ASM) {
        vars.insert(
            self.left.var_name.clone(),
            VariableHashMapValue {
                var: self.left.get_var_enum_from_type(),
                scope: VarScope::Global,
                index: 0,
            },
        );

        asm.variable_declaration(&self.left.var_name);
        self.right.visit_com(vars, f, asm);
        asm.variable_assignment(&self.left.var_name);
    }

    fn visit(&self, vars: &mut VariableHashMap, functions: Rc<RefCell<Functions>>) -> VisitResult {
        let right_visit = self.right.visit(vars, functions);

        let var_name = String::from(self.left.var_name.as_str());

        // TODO: change this so that the expression is stored here and we need to visit the varible
        // to evaluate the value
        match &*right_visit.token {
            TokenEnum::StringLiteral(s) => {
                vars.insert(
                    var_name.clone(),
                    VariableHashMapValue {
                        var: VariableEnum::String(s.into()),
                        scope: VarScope::Global,
                        index: 0,
                    },
                );
            }

            TokenEnum::Number(n) => {
                vars.insert(
                    var_name.clone(),
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

    fn get_token(&self) -> &Token {
        todo!()
    }

    fn print(&self) {
        trace!("{:#?}", self);
    }

    fn type_check(&self, call_stack: &crate::semantic::semantic_analyzer::CallStackRecord) {
        todo!()
    }
}
