#![allow(dead_code)]

use std::{collections::HashMap, rc::Rc, cell::RefCell};

use parser::parser::Parser;

use crate::interpreter::interpreter::Interpreter;

mod ast;
mod constants;
mod helpers;
mod interpreter;
mod lexer;
mod parser;
mod tests;

fn main() {
    let file = std::fs::read("test/first.txt").unwrap();

    let mut parser = Parser::new(&file);
    let ast = parser.parse_program();

    let mut interpreter = Interpreter::new(ast);
    let _result = interpreter.interpret();

    let h: HashMap<i32, i32> = HashMap::new();
    let c = Rc::new(RefCell::new(h));

    let mut map = c.borrow_mut();
    map.insert(2, 3);

    // println!("\n\nRESULT\n{:#?}", result);
    // println!("{:#?}", interpreter.variables)
}
