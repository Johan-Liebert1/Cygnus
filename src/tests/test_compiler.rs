use std::{
    fs::{self, File},
    io::{BufReader, Read, Write},
};

use crate::{generate_asm, parse_input_file};

fn get_stdout_and_actual_result(file_name: &str) -> (String, String) {
    let mut stdout_str = String::new();

    if let Some(ref mut stdout) =
        parse_input_file(format!("./examples/{}", file_name), true, true, true)
    {
        stdout.read_to_string(&mut stdout_str);
    }

    let file_name_wo_ext = file_name.split('.').collect::<Vec<&str>>();

    println!("file_name_wo_ext {:?}", file_name_wo_ext);

    let file_result =
        fs::read_to_string(format!("./examples/output/{}", file_name_wo_ext[0])).unwrap();

    return (stdout_str, file_result);
}

#[test]
fn arithmetic() {
    let (stdout_str, file_result) = get_stdout_and_actual_result("arithmetic.cberk");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn if_elif_else() {
    let (stdout_str, file_result) = get_stdout_and_actual_result("if-elif-else.cberk");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn fibonacci() {
    let (stdout_str, file_result) = get_stdout_and_actual_result("fibonacci.cberk");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn loop_break() {
    let (stdout_str, file_result) = get_stdout_and_actual_result("loop_break.cberk");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn early_return() {
    let (stdout_str, file_result) = get_stdout_and_actual_result("early_return.cberk");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn logical() {
    let (stdout_str, file_result) = get_stdout_and_actual_result("logical.cberk");
    assert_eq!(stdout_str, file_result);
}
