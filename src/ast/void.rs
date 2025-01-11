use super::abstract_syntax_tree::AST;

pub struct Void;

impl AST for Void {
    fn visit(
        &self,
        _: &mut crate::interpreter::interpreter::Variables,
        _: std::rc::Rc<std::cell::RefCell<crate::interpreter::interpreter::Functions>>,
        _: &mut crate::semantic_analyzer::semantic_analyzer::CallStack,
    ) -> super::abstract_syntax_tree::VisitResult {
        todo!()
    }

    fn visit_com(
        &self,
        _: &mut crate::interpreter::interpreter::Variables,
        _: std::rc::Rc<std::cell::RefCell<crate::interpreter::interpreter::Functions>>,
        _: &mut crate::asm::asm::ASM,
        _: &mut crate::semantic_analyzer::semantic_analyzer::CallStack,
    ) {
    }

    fn semantic_visit(
        &mut self,
        _: &mut crate::semantic_analyzer::semantic_analyzer::CallStack,
        _: std::rc::Rc<std::cell::RefCell<crate::interpreter::interpreter::Functions>>,
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
