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

fn generate_asm() -> io::Result<()> {
    std::env::set_current_dir("./generated")?;

    let mut nasm = std::process::Command::new("nasm");
    nasm.args(["-f", "elf64", "-o", "output.o", "output.asm"]);
    let mut spawn = nasm.spawn()?;
    spawn.wait()?;

    let mut linker = std::process::Command::new("ld");
    linker.args(["output.o", "-o", "output"]);
    let mut spawn = linker.spawn()?;
    spawn.wait()?;

    Ok(())
}

fn main() {
    #[allow(non_snake_case)]
    let mut COMPILE_MODE = false;

    let cmd_args = std::env::args().collect::<Vec<String>>();

    for arg in cmd_args.into_iter().skip(1) {
        match arg.as_str() {
            "com" => {
                COMPILE_MODE = true
            },

            "int" => {
                COMPILE_MODE = false
            },

            e => {
                println!("Unrecognised arg {e}")
            }
        };
    }

    let file = std::fs::read("test/first.txt").unwrap();

    let mut parser = Parser::new(&file);
    let ast = parser.parse_program();

    let mut interpreter = Interpreter::new(ast, parser.functions);

    if COMPILE_MODE {
        let _result = interpreter.compile();

        match generate_asm() {
            Ok(_) => {
                println!("Successful! Not running the program");
            }

            Err(e) => {
                println!("Failed to generate asm: {:?}", e);
                return;
            }
        }

        // match std::process::Command::new("./output").spawn() {
        //     Ok(ref mut child) => match child.wait() {
        //         Ok(exit_status) => println!("Exited with status {exit_status}"),
        //         Err(err) => println!("Error while waiting for child {:?}", err),
        //     },

        //     Err(e) => {
        //         println!("Failed to spawn run process: {:?}", e);
        //     }
        // }

        return;
    }

    let result = interpreter.interpret();

    println!("{:#?}", result);
}
