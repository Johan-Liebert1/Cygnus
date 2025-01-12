use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::abstract_syntax_tree::AST,
    lexer::{lexer::Token, types::VarType},
};

pub type ASTNode = Rc<RefCell<Box<dyn AST>>>;

pub type TypeCast = Option<(Token, VarType)>;
