use std::{cell::RefCell, rc::Rc};

use crate::ast::abstract_syntax_tree::AST;

pub type ASTNode = Rc<RefCell<Box<dyn AST>>>;
