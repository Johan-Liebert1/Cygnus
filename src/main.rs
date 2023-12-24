#![allow(dead_code, unused)]

use std::{
    char,
    io::{self, BufReader, Read},
    process::{ChildStdout, Stdio},
};

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

pub fn generate_asm() -> io::Result<()> {
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

pub fn parse_input_file(
    path: String,
    compile_mode: bool,
    run_asm: bool,
    is_test: bool,
) -> Option<ChildStdout> {
    let file = match std::fs::read(&path) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open `{path}` with err {:?}", err);
            return None;
        },
    };

    let mut parser = Parser::new(&file);
    let ast = parser.parse_program(false);

    let mut interpreter = Interpreter::new(ast, parser.functions);

    if compile_mode {
        let _result = interpreter.compile();

        let current_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir("./generated").unwrap();

        match generate_asm() {
            Ok(_) => {
                println!(
                    "Successful!{}",
                    if run_asm {
                        ""
                    } else {
                        " Not running the program"
                    }
                );
            }

            Err(e) => {
                println!("Failed to generate asm: {:?}", e);
                std::env::set_current_dir(current_dir).unwrap();
                return None;
            }
        }

        if !run_asm {
            std::env::set_current_dir(current_dir).unwrap();
            return None;
        }

        match std::process::Command::new("./output")
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(ref mut child) => match child.wait() {
                Ok(exit_status) => {
                    if !is_test {
                        println!("Exited with status {exit_status}")
                    }

                    std::env::set_current_dir(current_dir).unwrap();
                    return Some(child.stdout.take().unwrap());
                }

                Err(err) => println!("Error while waiting for child {:?}", err),
            },

            Err(e) => {
                println!("Failed to spawn run process: {:?}", e);
            }
        }

        std::env::set_current_dir(current_dir).unwrap();
        return None;
    }

    let result = interpreter.interpret();

    println!("{:#?}", result);

    return None;
}

fn main() {
    #[allow(non_snake_case)]
    let mut COMPILE_MODE = false;

    let cmd_args = std::env::args().collect::<Vec<String>>();

    for arg in cmd_args.into_iter().skip(1) {
        match arg.as_str() {
            "com" => COMPILE_MODE = true,

            "int" => COMPILE_MODE = false,

            e => {
                println!("Unrecognised arg {e}")
            }
        };
    }

    if let Some(stdout) = parse_input_file("test/first.txt".into(), COMPILE_MODE, false, false) {
        let mut reader = BufReader::new(stdout);
        let mut buf = vec![];
        reader.read_to_end(&mut buf);
        println!("{:?}", buf);
    }
}
