use std::rc::Rc;

use super::abstract_syntax_tree::AST;

pub struct LogicalExpression {
    left: Rc<Box<dyn AST>>,
}
