use std::{
    fs::{self, File},
    io::{BufReader, Read, Write},
};

use crate::{generate_asm, parse_input_file};

#[test]
fn arithmetic() {
    let mut stdout_str = String::new();

    if let Some(ref mut stdout) = parse_input_file("./examples/arithmetic.cberk".into(), true, true, true) {
        stdout.read_to_string(&mut stdout_str);
    }

    let mut file = File::create("output.sfdf").unwrap();
    file.write_all(stdout_str.as_bytes());

    let file_result = fs::read_to_string("./examples/output/arithmetic").unwrap();
    let file_result_left = fs::read_to_string("./output.sfdf").unwrap();

    assert_eq!(file_result_left, file_result);
}
