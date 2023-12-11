#![allow(dead_code, unused)]

use std::io;

use parser::parser::Parser;

use crate::interpreter::interpreter::Interpreter;

mod asm;
mod ast;
mod constants;
mod helpers;
mod interpreter;
mod lexer;
mod parser;
mod tests;

fn run_asm() -> io::Result<()> {
    std::env::set_current_dir("./generated")?;

    let mut nasm = std::process::Command::new("nasm");
    nasm.args(["-f", "elf64", "-o", "output.o", "output.asm"]);
    nasm.spawn()?;

    let mut linker = std::process::Command::new("ld");
    linker.args(["output.o", "-o" ,"output"]);
    linker.spawn()?;

    std::process::Command::new("./output").spawn()?;

    Ok(())
}

fn main() {
    let file = std::fs::read("test/first.txt").unwrap();

    let mut parser = Parser::new(&file);
    let ast = parser.parse_program();

    let mut interpreter = Interpreter::new(ast, parser.functions);
    let _result = interpreter.compile();

    match run_asm() {
        Ok(_) => {
            println!("Successful!");
        }

        Err(e) => {
            println!("{:?}", e);
            return;
        }
    }
}
