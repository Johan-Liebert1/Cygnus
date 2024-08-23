use crate::{
    helpers::{self, compiler_error},
    lexer::types::{VarType, TYPE_FLOAT, TYPE_INT, TYPE_STRING},
    semantic_analyzer::semantic_analyzer::CallStack,
    trace,
    types::ASTNode,
};

use core::panic;
use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        lexer::Token,
        tokens::{Number, VariableEnum},
    },
};

use super::abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut, VisitResult, AST};

#[derive(Debug, Clone)]
pub struct Variable {
    token: Box<Token>,

    pub var_name: String,

    /// The actual var type. Example if
    /// def a: Array<int>; then var_type is Array<int>
    /// def a: my_struct; then var_type is my_struct
    /// a.c; then var_type is still my_struct
    pub var_type: VarType,

    /// The resulting var type. Example if
    /// def a: Array<int>; then result_type is int
    /// def a: my_struct; then result_type is my_struct
    /// a.c; then result_type is the type of member 'c'
    pub result_type: VarType,

    pub dereference: bool,
    pub store_address: bool,
    pub times_dereferenced: usize,
    pub offset: usize,
    pub is_memory_block: bool,
    pub type_cast: Option<VarType>,
    pub array_aceess_index: Option<ASTNode>,

    /// if it's a.b.c.d then the var_name is 'a'
    /// and member_access contains ['b', 'c']
    pub member_access: Vec<String>,

    pub is_const: bool,
}

impl Variable {
    pub fn new(
        token: Box<Token>,
        var_type: VarType,
        var_name: String,
        dereference: bool,
        store_address: bool,
        times_dereferenced: usize,
    ) -> Self {
        Self {
            token,
            result_type: var_type.clone(),
            var_type,
            var_name,
            dereference,
            store_address,
            times_dereferenced,
            offset: 0,
            is_memory_block: false,
            type_cast: None,
            array_aceess_index: None,
            member_access: vec![],
            is_const: false,
        }
    }

    pub fn size(&self) -> usize {
        self.result_type.get_size()
    }

    pub fn get_var_enum_from_type(&self) -> VariableEnum {
        return match self.var_type {
            // TYPE_STRING => VariableEnum::String(String::from("")),
            // TYPE_INT => VariableEnum::Number(Number::Integer(0)),

            // t => match &t[1..] {
            //     TYPE_INT | TYPE_STRING | TYPE_FLOAT => VariableEnum::Pointer(t[1..].into()),
            //     _ => unimplemented!("Type {t} not known"),
            // },
            VarType::Int => VariableEnum::Number(Number::Integer(0)),
            VarType::Int8 => todo!(),
            VarType::Int16 => todo!(),
            VarType::Int32 => todo!(),
            VarType::Str => VariableEnum::String(String::from("")),
            VarType::Float => todo!(),
            VarType::Char => todo!(),
            VarType::Ptr(_) => todo!(),
            VarType::Unknown => todo!(),
            VarType::Array(..) => todo!(),
            VarType::Struct(_, _) => todo!(),
            VarType::Function(_, _, _) => todo!(),
        };
    }

    pub fn store_result_type(&mut self) {
        self.result_type = self.result_type.clone();
    }
}

impl AST for Variable {
    fn visit_com(&self, x: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        if let Some(ast_node) = &self.array_aceess_index {
            ast_node.borrow().visit_com(x, f, asm, call_stack);
        }

        asm.gen_asm_for_var(&self, &call_stack);
    }

    fn visit(&self, v: &mut Variables, _: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
        todo!()
    }

    fn get_token(&self) -> &Token {
        return &self.token;
    }

    fn print(&self) {
        println!("{:#?}", self);
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        let (variable_in_stack, _) = call_stack.get_var_with_name(&self.var_name);

        if let Some(variable_in_stack) = variable_in_stack {
            if let Some(casted_type) = &self.type_cast {
                self.var_type = casted_type.clone();
            } else {
                // actually need this as we don't have type information for the variable all
                // the time. We only have it at time of decleration and store it in the call
                // stack
                //
                // have to do this here as the variable_in_stack comes from variable definition, which
                // is the only AST node where we have the information about this variable

                //
                // In decleration statement we borrow_mut the current variable to call
                // semantic_visit on the variable. So borrowing again will panic with
                // AlreadyBorrowed mutable error

                match variable_in_stack.try_borrow() {
                    Ok(variable_in_stack_borrowed) => {
                        self.var_type = variable_in_stack_borrowed.var_type.clone();
                        self.is_const = variable_in_stack_borrowed.is_const;
                        self.offset = variable_in_stack_borrowed.offset;
                    }

                    Err(_) => { /* do nothing here */ }
                };
            };

            self.result_type = self.var_type.clone();

            if let Some(ast_node) = &self.array_aceess_index {
                ast_node.borrow_mut().semantic_visit(call_stack, f);

                if let VarType::Array(type_, _) = &self.result_type {
                    // if an index is being accessed, then we have to get the underlying type
                    self.result_type = *type_.clone();
                } else {
                    helpers::compiler_error(
                        format!("Cannot index into a variable of type {}", self.result_type),
                        &self.token,
                    );
                }
            }

            if self.member_access.len() > 0 {
                match call_stack
                    .user_defined_types
                    .iter()
                    .find(|x| x.name == format!("{}", self.var_type.get_pointer_type()))
                {
                    Some(user_defined_type) => match &user_defined_type.type_ {
                        // TODO: Handle struct -> struct -> member_access here
                        VarType::Struct(_, members) => {
                            for m in members.borrow().iter() {
                                if m.name == self.member_access[0] {
                                    self.result_type = m.member_type.clone();
                                    break;
                                }
                            }
                        }

                        tt => {
                            compiler_error(
                                format!("Cannot access '{}' on type '{}'", self.member_access[0], tt),
                                self.get_token(),
                            );
                        }
                    },

                    None => match &self.var_type {
                        VarType::Unknown => {
                            compiler_error(format!("Type '{}' not defined", self.var_type), self.get_token())
                        }

                        tt => {
                            compiler_error(
                                format!("Cannot access '{}' on type '{}'", self.member_access[0], tt),
                                self.get_token(),
                            );
                        }
                    },
                }
            }
        } else {
            // This might be a function pointer
            if let Some(function) = f.borrow().get(&self.var_name) {
                if let ASTNodeEnum::FunctionDef(fd) = function.func.borrow().get_node() {
                    self.var_type = VarType::Function(
                        self.var_name.clone(),
                        fd.parameters.iter().map(|param| param.borrow().var_type.clone()).collect(),
                        Box::new(function.return_type.clone()),
                    );
                } else {
                    unreachable!("Found non function_definition node inside functions hash map")
                }
            } else {
                helpers::compiler_error(
                    format!("Variable with name '{}' not found in current scope", self.var_name),
                    &self.token,
                );
            }
        }

        trace!("Variable self.var_type: {}", self.var_type);

        if self.store_address {
            self.result_type = VarType::Ptr(Box::new(self.var_type.clone()))
        }
    }

    fn get_node(&self) -> ASTNodeEnum {
        return ASTNodeEnum::Variable(&self);
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        return ASTNodeEnumMut::Variable(self);
    }

    fn get_type(&self) -> (VarType, VarType) {
        return (
            self.result_type.get_actual_type(self.times_dereferenced, &self.token),
            self.result_type.clone(),
        );
    }
}
