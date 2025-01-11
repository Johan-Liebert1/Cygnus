use std::{
    fs::{self},
    io::Read,
    process::exit,
};

use crate::parse_input_file;

pub fn get_file_result(file_name: &str) -> String {
    let file_name_wo_ext = file_name.split('.').collect::<Vec<&str>>();
    let file_result = fs::read_to_string(format!("./examples/output/{}", file_name_wo_ext[0]));

    match file_result {
        Ok(res) => res,
        Err(err) => {
            eprint!("Failed to read file {file_name}, with error: {err}");
            exit(1);
        }
    }
}

pub fn get_stdout_and_actual_result(file_name: &str) -> (String, String, String) {
    let mut stdout_str = String::new();
    let mut stderr_str = String::new();

    if let Some(ref mut stdout) = parse_input_file(format!("./examples/{}", file_name), true, true, true, &vec![]) {
        let _ = stdout.0.read_to_string(&mut stdout_str);
        let _ = stdout.1.read_to_string(&mut stderr_str);
    }

    return (stdout_str, stderr_str, get_file_result(file_name));
}

#[test]
fn arithmetic() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("arithmetic.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn float_arithmetic() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("float_arithmetic.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn if_elif_else() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("if-elif-else.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn fibonacci() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("fibonacci.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn loop_break() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("loop_break.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn loop_continue() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("loop_continue.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn early_return() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("early_return.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn logical() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("logical.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn euler_001() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("project_euler/001.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn euler_002() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("project_euler/002.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn euler_004() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("project_euler/004.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn euler_005() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("project_euler/005.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn euler_006() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("project_euler/006.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn euler_007() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("project_euler/007.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn rule_110() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("110.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn palindrome_number() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("palindrome.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn bubble_sort() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("bubble_sort.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn game_of_life() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("game_of_life.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn game_of_life_array() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("game_of_life_array.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn palindrome_str() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("palindrome_str.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn basic_struct() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("basic_struct.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn struct_assignment() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("struct_assign.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn assignment() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("assignment.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn count_char_occurances_in_file() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("count_char_occurances_in_file.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn loop_var() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("loop_var.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn strlen() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("strlen.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn string_ends_with() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("string_ends_with.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn string_starts_with() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("string_starts_with.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn ptr_to_struct() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("ptr_to_struct.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn function_pointers() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("func_ptrs.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn function_pointers_as_args() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("func_ptrs_as_args.cy");
    assert_eq!(stdout_str, file_result);
}

#[test]
fn decleration_only() {
    let (stdout_str, _, file_result) = get_stdout_and_actual_result("decleration_only.cy");
    assert_eq!(stdout_str, file_result);
}
