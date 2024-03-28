use core::panic;
use std::{cell::RefCell, rc::Rc};

use crate::{
    asm::asm::ASM,
    helpers::compiler_error,
    interpreter::interpreter::{Functions, Variables},
    lexer::{
        lexer::Token,
        types::{StructMemberType, VarType},
    },
    semantic_analyzer::semantic_analyzer::CallStack,
    trace,
    types::ASTNode,
};

use super::{
    abstract_syntax_tree::{ASTNodeEnum, ASTNodeEnumMut, VisitResult, AST},
    variable::Variable,
};

#[derive(Debug)]
pub struct StructMember {
    pub var_token: Token,
    pub name: String,
    pub rhs: ASTNode,
}

#[derive(Debug)]
pub struct StructDecleration {
    pub name: String,
    pub members: Vec<StructMember>,
    result_type: VarType,
    token: Token,
}

impl StructDecleration {
    pub fn new(name: String, members: Vec<StructMember>, token: Token) -> Self {
        Self {
            name,
            members,
            token,
            result_type: VarType::Unknown,
        }
    }

    pub fn get_member_definition_order(&self) -> Vec<&String> {
        let mut member_order = vec![];

        for member in &self.members {
            member_order.push(&member.name);
        }

        return member_order;
    }
}

impl AST for StructDecleration {
    fn visit(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, call_stack: &mut CallStack) -> VisitResult {
        todo!()
    }

    fn visit_com(&self, v: &mut Variables, f: Rc<RefCell<Functions>>, asm: &mut ASM, call_stack: &mut CallStack) {
        // iterate in reverse order so that
        for member_type in self.members.iter().rev() {
            member_type.rhs.borrow().visit_com(v, f.clone(), asm, call_stack)
        }
    }

    fn semantic_visit(&mut self, call_stack: &mut CallStack, f: Rc<RefCell<Functions>>) {
        let mut member_types = vec![];

        let struct_type = call_stack.user_defined_types.iter().find(|x| x.name == self.name);

        if matches!(struct_type, None) {
            compiler_error(format!("Type '{}' not defined", self.name), self.get_token());
        }

        let struct_type = struct_type.unwrap();

        for member in &mut self.members {
            member.rhs.borrow_mut().semantic_visit(call_stack, f.clone());

            member_types.push(StructMemberType {
                name: member.name.clone(),
                member_type: member.rhs.borrow().get_type().0,
                offset: 0,
            });
        }

        match &struct_type.type_ {
            VarType::Struct(_, members) => {
                for (index, member_type) in member_types.iter().enumerate() {
                    let borrowed = members.borrow();
                    let found = borrowed.iter().find(|x| x.name == member_type.name);

                    // could not find the field
                    if found.is_none() {
                        compiler_error(
                            format!("No field '{}' defined for struct {}", member_type.name, self.name),
                            self.get_token(),
                        );
                    }

                    let found = found.unwrap();

                    if found.member_type != member_type.member_type {
                        compiler_error(
                            format!(
                                "Field '{}' for struct {} is defined as type {}, but got {}",
                                member_type.name, self.name, found.member_type, member_type.member_type
                            ),
                            self.members[index].rhs.borrow().get_token(),
                        );
                    }
                }
            }

            typ => {
                compiler_error(format!("Type {typ} is not defined"), self.get_token());
            }
        };

        self.result_type = VarType::Struct(self.name.clone(), Rc::new(RefCell::new(member_types)));
    }

    fn get_token(&self) -> &Token {
        &self.token
    }

    fn get_node(&self) -> ASTNodeEnum {
        ASTNodeEnum::Struct(self)
    }

    fn get_node_mut(&mut self) -> ASTNodeEnumMut {
        ASTNodeEnumMut::Struct(self)
    }

    fn print(&self) {
        todo!()
    }

    fn get_type(&self) -> (VarType, VarType) {
        (self.result_type.clone(), self.result_type.clone())
    }
}
