#[allow(unused_imports)]

use crate::{
    ast::abstract_syntax_tree::VisitResult,
    lexer::tokens::{Number, TokenEnum as TE},
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
        ("3 + 2", TE::Number(Number::Integer(5))),
        ("4 + (2 + 3)", TE::Number(Number::Integer(9))),
        ("(5 + 3) + 2", TE::Number(Number::Integer(10))),
        ("1 + 1", TE::Number(Number::Integer(2))),
        ("0 + (3 + 2)", TE::Number(Number::Integer(5))),
    ];

    for (index, file) in files.iter().enumerate() {
        assert_eq!(
            *run_the_test(&file.0.as_bytes().to_vec()).token,
            file.1,
            "\nFailed at index: {}",
            index
        );
    }
}

#[test]
fn subtraction() {
    let files = [
        ("9 - 5", TE::Number(Number::Integer(4))),
        ("10 - (3 + 2)", TE::Number(Number::Integer(5))),
        ("(15 - 5) - 3", TE::Number(Number::Integer(7))),
        ("6 - 1", TE::Number(Number::Integer(5))),
        ("8 - (3 + 1)", TE::Number(Number::Integer(4))),
    ];

    for (index, file) in files.iter().enumerate() {
        assert_eq!(
            *run_the_test(&file.0.as_bytes().to_vec()).token,
            file.1,
            "\nFailed at index: {}",
            index
        );
    }
}

#[test]
fn multiplicaiton() {
    let files = [
        ("3 * 2", TE::Number(Number::Integer(6))),
        ("2 * (3 * 2)", TE::Number(Number::Integer(12))),
        ("(5 * 3) * 2", TE::Number(Number::Integer(30))),
        ("4 * 3", TE::Number(Number::Integer(12))),
        ("5 * (2 * 2)", TE::Number(Number::Integer(20))),
    ];
    for (index, file) in files.iter().enumerate() {
        assert_eq!(
            *run_the_test(&file.0.as_bytes().to_vec()).token,
            file.1,
            "\nFailed at index: {}",
            index
        );
    }
}

#[test]
fn division() {
    let files = [
        ("8 / 2", TE::Number(Number::Integer(4))),
        ("18 / (2 + 1)", TE::Number(Number::Integer(6))),
        ("(12 / 3) / 2", TE::Number(Number::Integer(2))),
        ("15 / 5", TE::Number(Number::Integer(3))),
        ("20 / (5 + 1)", TE::Number(Number::Integer(3))),
    ];

    for (index, file) in files.iter().enumerate() {
        assert_eq!(
            *run_the_test(&file.0.as_bytes().to_vec()).token,
            file.1,
            "\nFailed at index: {}",
            index
        );
    }
}

#[test]
fn arithmetic() {
    let files = [
        ("5 + 3 * 2", TE::Number(Number::Integer(11))),
        ("(6 + 2) * 3 - 6 / 2", TE::Number(Number::Integer(21))),
        ("(12 + 3) / 3 * 2", TE::Number(Number::Integer(10))),
        ("5 + 2 * (3 - 1)", TE::Number(Number::Integer(9))),
        ("(5 + 3) * 2 / 4", TE::Number(Number::Integer(4))),
        ("(3. + (4. * (5. - 6.))) * ((7. + (8. / (9. + 10.))) / 11.) - (12. * 13.) + (14. / (15. + (16. - 17.)))", TE::Number(Number::Float(-155.67464)))
    ];

    for (index, file) in files.iter().enumerate() {
        assert_eq!(
            *run_the_test(&file.0.as_bytes().to_vec()).token,
            file.1,
            "\nFailed at index: {}",
            index
        );
    }
}
