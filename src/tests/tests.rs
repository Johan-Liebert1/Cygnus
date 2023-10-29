use crate::{
    ast::abstract_syntax_tree::VisitResult,
    lexer::tokens::{Number, TokenEnum},
    parser::parser::Parser,
};

fn run_the_test(file: &Vec<u8>) -> VisitResult {
    let mut parser = Parser::new(&file);

    let ast = parser.parse_statements();

    return ast.visit();
}

#[test]
fn addition() {
    let files = [
        ("3 + 2", 5),
        ("4 + (2 + 3)", 9),
        ("(5 + 3) + 2", 10),
        ("1 + 1", 2),
        ("0 + (3 + 2)", 5),
    ];

    for (index, file) in files.iter().enumerate() {
        assert_eq!(
            *run_the_test(&file.0.as_bytes().to_vec()).token,
            TokenEnum::Number(Number::Integer(file.1)),
            "\nFailed at index: {}",
            index
        );
    }
}

#[test]
fn subtraction() {
    let files = [
        ("9 - 5", 4),
        ("10 - (3 + 2)", 5),
        ("(15 - 5) - 3", 7),
        ("6 - 1", 5),
        ("8 - (3 + 1)", 4),
    ];

    for (index, file) in files.iter().enumerate() {
        assert_eq!(
            *run_the_test(&file.0.as_bytes().to_vec()).token,
            TokenEnum::Number(Number::Integer(file.1)),
            "\nFailed at index: {}",
            index
        );
    }
}

#[test]
fn multiplicaiton() {
    let files = [
        ("3 * 2", 6),
        ("2 * (3 * 2)", 12),
        ("(5 * 3) * 2", 30),
        ("4 * 3", 12),
        ("5 * (2 * 2)", 20),
    ];
    for (index, file) in files.iter().enumerate() {
        assert_eq!(
            *run_the_test(&file.0.as_bytes().to_vec()).token,
            TokenEnum::Number(Number::Integer(file.1)),
            "\nFailed at index: {}",
            index
        );
    }
}

#[test]
fn division() {
    let files = [
        ("8 / 2", 4),
        ("18 / (2 + 1)", 6),
        ("(12 / 3) / 2", 2),
        ("15 / 5", 3),
        ("20 / (5 + 1)", 3),
    ];

    for (index, file) in files.iter().enumerate() {
        assert_eq!(
            *run_the_test(&file.0.as_bytes().to_vec()).token,
            TokenEnum::Number(Number::Integer(file.1)),
            "\nFailed at index: {}",
            index
        );
    }
}

#[test]
fn arithmetic() {
    let files = [
        ("5 + 3 * 2", 11),
        ("(6 + 2) * 3 - 6 / 2", 21),
        ("(12 + 3) / 3 * 2", 10),
        ("5 + 2 * (3 - 1)", 11),
        ("(5 + 3) * 2 / 4", 4),
    ];

    for (index, file) in files.iter().enumerate() {
        assert_eq!(
            *run_the_test(&file.0.as_bytes().to_vec()).token,
            TokenEnum::Number(Number::Integer(file.1)),
            "\nFailed at index: {}",
            index
        );
    }
}
