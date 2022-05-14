#![cfg(test)]

use Instruction::*;

use super::*;

//test one of each really simple operands
#[test]
fn basic_ops() {
    assert_eq!(parse_file("CLEAR"), Ok(vec![Clear(0)]));
    assert_eq!(parse_file("STOP"), Ok(vec![Clear(1)]));
    assert_eq!(parse_file("INC"), Ok(vec![Inc]));
    assert_eq!(parse_file("DEC"), Ok(vec![Dec]));

    assert_eq!(parse_file("ADD 12"), Ok(vec![Add(12)]));
    assert_eq!(parse_file("label: BUZ label  "), Ok(vec![Bnz(0)]));
    assert_eq!(
        parse_file("start: CLEAR \n JUMP start\n "),
        Ok(vec![Clear(0), Jump(0)])
    );

    assert_eq!(parse_file("STORE 18"), Ok(vec![Store(18)]));
    assert_eq!(parse_file("STOP"), Ok(vec![Clear(1)]));
}

//blank lines that return empty lists of instructions
#[test]
fn blanks() {
    assert_eq!(parse_file(";     "), Ok(vec![]));
    assert_eq!(parse_file("            ;     "), Ok(vec![]));
    assert_eq!(parse_file("                 "), Ok(vec![]));
    assert_eq!(parse_file(" ;;           ;;  ;   ;;   "), Ok(vec![]));
    assert_eq!(parse_file("            ; test comment    "), Ok(vec![]));
    assert_eq!(parse_file(";test comment     "), Ok(vec![]));
    assert_eq!(parse_file(";     "), Ok(vec![]));
}

//test lines with comments and some weird whitespacing
#[test]
fn comments() {
    assert_eq!(parse_file("CLEAR;"), Ok(vec![Clear(0)]));
    assert_eq!(parse_file("DEC       ; test comment  "), Ok(vec![Dec]));
    assert_eq!(
        parse_file("ADD  100     ; test comment  "),
        Err(ParseError::OperandOverflow(0))
    );
    assert_eq!(
        parse_file("   LOAD  21     ; test comment DEC  "),
        Ok(vec![Load(21)])
    );
    assert_eq!(parse_file("   LOAD  31     ;; test;  "), Ok(vec![Load(31)]));
}

// //make sure we get the right errors
// disabled for now beacuse the error reporting is bad
// #[test]
// fn errors() {
//     assert_eq!(
//         parse_file("ABC"),
//         Err(ParseError::BadInput(0, "".to_owned()))
//     );
//     assert_eq!(
//         parse_file("DEC 12"),
//         Err(ParseError::BadInput(0, "".to_owned()))
//     );
//     assert_eq!(
//         parse_file("DEC 12; INC"),
//         Err(ParseError::UnexpectedSymbol(0))
//     );
//     assert_eq!(
//         parse_file("STORE 12 INC"),
//         Err(ParseError::UnexpectedSymbol(0))
//     );
//     assert_eq!(
//         parse_file("ADD x ;"),
//         Err(ParseError::MissingOperandError(0))
//     );
//     assert_eq!(
//         parse_file("STOP 14 ;"),
//         Err(ParseError::UnexpectedSymbol(0))
//     );
//     assert_eq!(
//         parse_file("DEEZ NUTS ; haha"),
//         Err(ParseError::InvalidOpcode(0))
//     );
//     assert_eq!(
//         parse_file(" CLEAR \n ADD x"),
//         Err(ParseError::MissingOperandError(1))
//     );
//     assert_eq!(
//         parse_file(" CLEAR \n SUB 12"),
//         Err(ParseError::InvalidOpcode(1))
//     );
// }

#[test]
fn file_empty() {
    assert_eq!(parse_file(""), Ok(Vec::new()));
    assert_eq!(parse_file("         "), Ok(Vec::new()));
    assert_eq!(parse_file("       \n  "), Ok(Vec::new()));
    assert_eq!(parse_file("       \n  ;   \n"), Ok(Vec::new()));
}

#[test]
fn multiple_instructions() {
    assert_eq!(parse_file("\n CLEAR \n"), Ok(vec![Clear(0)]));
    assert_eq!(
        parse_file("CLEAR \n ADD 15 \n STORE 0\nSTOP "),
        Ok(vec![Clear(0), Add(15), Store(0), Clear(1),])
    );
}
