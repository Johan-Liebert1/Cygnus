use std::{
    fs,
    io::{BufReader, Read},
};

use crate::{generate_asm, parse_input_file};

#[test]
fn arithmetic() {
    let mut buf = vec![];

    if let Some(stdout) = parse_input_file("./examples/arithmetic".into(), true, true, true) {
        let mut reader = BufReader::new(stdout);
        reader.read_to_end(&mut buf);
    }

    let file_result = fs::read_to_string("./examples/output/arithmetic").unwrap();

    let stdout = String::from_utf8(buf.clone()).unwrap();

    assert_eq!(stdout, file_result);
}
