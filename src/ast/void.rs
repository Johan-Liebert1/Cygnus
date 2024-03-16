use super::abstract_syntax_tree::AST;

pub struct Void;

impl AST for Void {
    fn visit(
        &self,
        v: &mut crate::interpreter::interpreter::Variables,
        f: std::rc::Rc<std::cell::RefCell<crate::interpreter::interpreter::Functions>>,
        call_stack: &mut crate::semantic_analyzer::semantic_analyzer::CallStack,
    ) -> super::abstract_syntax_tree::VisitResult {
        todo!()
    }

    fn visit_com(
        &self,
        v: &mut crate::interpreter::interpreter::Variables,
        f: std::rc::Rc<std::cell::RefCell<crate::interpreter::interpreter::Functions>>,
        asm: &mut crate::asm::asm::ASM,
        call_stack: &mut crate::semantic_analyzer::semantic_analyzer::CallStack,
    ) {
    }

    fn semantic_visit(
        &mut self,
        call_stack: &mut crate::semantic_analyzer::semantic_analyzer::CallStack,
        f: std::rc::Rc<std::cell::RefCell<crate::interpreter::interpreter::Functions>>,
    ) {
    }

    fn get_token(&self) -> &crate::lexer::lexer::Token {
        todo!()
    }

    fn get_node(&self) -> super::abstract_syntax_tree::ASTNodeEnum {
        todo!()
    }

    fn get_node_mut(&mut self) -> super::abstract_syntax_tree::ASTNodeEnumMut {
        todo!()
    }

    fn get_type(&self) -> (crate::lexer::types::VarType, crate::lexer::types::VarType) {
        todo!()
    }

    fn print(&self) {
        todo!()
    }
}
