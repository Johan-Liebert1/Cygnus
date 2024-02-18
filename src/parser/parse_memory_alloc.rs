use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::variable::Variable,
    lexer::{
        tokens::{Number, TokenEnum},
        types::VarType,
    },
    trace,
    types::ASTNode,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /// MEMORY_BLOCK -> mem VAR_NAME (size in bytes)
    pub fn parse_memory_alloc(&mut self) -> ASTNode {
        // we get here after consuming the 'mem' token
        let var_token = self.validate_token(TokenEnum::Variable("".into()));

        let memory_size = self.validate_token(TokenEnum::Number(Number::Integer(0)));

        trace!("var_token {:#?}", var_token);

        if let TokenEnum::Variable(var_name) = &var_token.token {
            let mut variable = Variable::new(
                Box::new(var_token.clone()),
                VarType::Ptr(Box::new(VarType::Int)),
                var_name.clone(),
                false,
                false,
                0,
            );

            variable.is_memory_block = true;

            return Rc::new(RefCell::new(Box::new(variable)));
        }

        unreachable!()
    }
}
