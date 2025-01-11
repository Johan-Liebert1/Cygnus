use std::{
    io::Read,
    process::{Command, Stdio},
    sync::Once,
};

use super::test_compiler::get_file_result;

static COMPILE_ONCE: Once = Once::new();

fn compile_binary() {
    COMPILE_ONCE.call_once(|| {
        Command::new("cargo")
            .args(["build"])
            .status()
            .expect("Failed to compile binary");
    });
}

fn try_to_compile_example(file_name: &str) -> (String, String) {
    let command = Command::new("./target/debug/lang")
        .args(["-f".into(), format!("./examples/{}", file_name)])
        .stderr(Stdio::piped())
        .spawn();

    let mut stderr = String::new();

    match command {
        Ok(mut child) => {
            let _ = child.stderr.take().unwrap().read_to_string(&mut stderr);
        }

        Err(err) => println!("Failed to spawn process err {:?}", err),
    }

    return (stderr, get_file_result(file_name));
}

#[test]
fn const_reassign() {
    compile_binary();
    let (stderr, file_result) = try_to_compile_example("errors/const_reassign.cy");

    assert_eq!(stderr, file_result);
}

#[test]
fn function_pointer_incorrect_arg() {
    compile_binary();
    let (stderr, file_result) = try_to_compile_example("errors/func_ptr_incorrect_arg.cy");

    assert_eq!(stderr, file_result);
}

#[test]
fn function_pointer_non_function() {
    compile_binary();
    let (stderr, file_result) = try_to_compile_example("errors/func_ptr_non_func.cy");

    assert_eq!(stderr, file_result);
}

#[test]
fn function_no_return() {
    compile_binary();
    let (stderr, file_result) = try_to_compile_example("errors/func_no_ret.cy");

    assert_eq!(stderr, file_result);
}

#[test]
fn function_no_statements() {
    compile_binary();
    let (stderr, file_result) = try_to_compile_example("errors/func_no_statements.cy");

    assert_eq!(stderr, file_result);
}
