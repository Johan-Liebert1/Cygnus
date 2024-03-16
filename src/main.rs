#![allow(dead_code, unused)]

use std::{
    char,
    io::{self, BufReader, Read},
    process::{ChildStdout, Stdio},
    rc::Rc,
    time::Duration,
};

use lexer::types::VarType;
use parser::parser::Parser;

use crate::{interpreter::interpreter::Interpreter, semantic_analyzer::semantic_analyzer::SemanticAnalyzer};

mod asm;
mod ast;
mod constants;
mod helpers;
mod interpreter;
mod lexer;
mod parser;
mod semantic_analyzer;
mod tests;
mod types;

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

pub fn parse_input_file(path: String, compile_mode: bool, run_asm: bool, is_test: bool) -> Option<ChildStdout> {
    println!("Parsing file {path}");

    let file = match std::fs::read(&path) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open `{path}` with err {:?}", err);
            return None;
        }
    };

    let mut parser = Parser::new(&file, &path);
    let ast = parser.parse_program();

    let mut semantic_analyzer = SemanticAnalyzer::new(ast.clone(), Rc::clone(&parser.functions));
    semantic_analyzer.analyze();

    let mut interpreter = Interpreter::new(ast.clone(), parser.functions.clone());

    let mut semantic_analyzer = SemanticAnalyzer::new(ast, parser.functions);

    if compile_mode {
        let _result = interpreter.compile(&mut semantic_analyzer.call_stack);

        let current_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir("./generated").unwrap();

        match generate_asm() {
            Ok(_) => {
                println!("Successful!{}", if run_asm { "" } else { " Not running the program" });
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

        match std::process::Command::new("./output").stdout(Stdio::piped()).spawn() {
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

    let result = interpreter.interpret(&mut semantic_analyzer.call_stack);

    println!("{:#?}", result);

    return None;
}

fn main() {
    #[allow(non_snake_case)]
    let mut COMPILE_MODE = true;
    #[allow(non_snake_case)]
    let mut RUN_PROGRAM = false;

    let cmd_args = std::env::args().collect::<Vec<String>>();

    let mut file_name_next = false;

    let mut file_name = "test/first.cberk";

    for arg in cmd_args.iter().skip(1) {
        match arg.as_str() {
            "com" => COMPILE_MODE = true,
            "int" => COMPILE_MODE = false,
            "-r" => RUN_PROGRAM = true,
            "-f" => file_name_next = true,

            e => {
                if !file_name_next {
                    println!("Unrecognised arg {e}");
                    break;
                }

                file_name = e;
            }
        };
    }

    if let Some(ref mut stdout) = parse_input_file(file_name.into(), COMPILE_MODE, RUN_PROGRAM, false) {
        let mut str = String::new();
        stdout.read_to_string(&mut str);
        println!("{:?}", str);
    }

    std::thread::sleep(Duration::new(1000000, 0))
}
