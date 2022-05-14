#![cfg(test)]

use super::*;

//test basic opcode operand lines
#[test]
fn basic_operands() {
    assert_eq!(parse_file("CLEAR"), Ok(vec![Instruction::Clear(0)]));

    assert_eq!(parse_file("ADD 12"), Ok(vec![Instruction::Add(12)]));
    assert_eq!(
        parse_file("label: BUZ label  "),
        Ok(vec![Instruction::Bnz(0)])
    );
    assert_eq!(parse_file("STORE 18"), Ok(vec![Instruction::Store(18)]));
    assert_eq!(parse_file("STOP"), Ok(vec![Instruction::Clear(1)]));
}

//test lines with comments and some weird whitespacing
#[test]
fn comments() {
    assert_eq!(parse_file("CLEAR;"), Ok(vec![Instruction::Clear(0)]));
    assert_eq!(
        parse_file("DEC       ; test comment  "),
        Ok(vec![Instruction::Dec])
    );
    assert_eq!(
        parse_file("ADD  100     ; test comment  "),
        Ok(vec![Instruction::Add(100)])
    );
    assert_eq!(
        parse_file("   LOAD  40     ; test comment DEC  "),
        Ok(vec![Instruction::Load(40)])
    );
    assert_eq!(
        parse_file("   LOAD  40     ;; test;  "),
        Ok(vec![Instruction::Load(40)])
    );
}

//some lines that should return blanks
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

//make sure we get the right errors
#[test]
fn errors() {
    assert_eq!(parse_file("ABC"), Err(ParseError::InvalidOpcode(0)));
    assert_eq!(parse_file("DEC 12"), Err(ParseError::UnexpectedOperand(0)));
    assert_eq!(
        parse_file("DEC 12; INC"),
        Err(ParseError::UnexpectedOperand(0))
    );
    assert_eq!(
        parse_file("STORE 12 INC"),
        Err(ParseError::UnexpectedOperand(0))
    );
    assert_eq!(parse_file("ADD x ;"), Err(ParseError::OperandError(0)));
    assert_eq!(
        parse_file("STOP 14 ;"),
        Err(ParseError::UnexpectedOperand(0))
    );
    assert_eq!(
        parse_file("DEEZ NUTS ; haha"),
        Err(ParseError::InvalidOpcode(0))
    );
    assert_eq!(
        parse_file(" CLEAR \n ADD x"),
        Err(ParseError::OperandError(1))
    );
    assert_eq!(
        parse_file(" CLEAR \n SUB 12"),
        Err(ParseError::InvalidOpcode(1))
    );
}

#[test]
fn file_empty() {
    assert_eq!(parse_file(""), Ok(Vec::new()));
    assert_eq!(parse_file("         "), Ok(Vec::new()));
    assert_eq!(parse_file("       \n  "), Ok(Vec::new()));
    assert_eq!(parse_file("       \n  ;   \n"), Ok(Vec::new()));
}

#[test]
fn multiple_instructions() {
    assert_eq!(parse_file("\n CLEAR \n"), Ok(vec![Instruction::Clear(0)]));
    assert_eq!(
        parse_file("CLEAR \n ADD 15 \n STORE 0\nSTOP "),
        Ok(vec![
            Instruction::Clear(0),
            Instruction::Add(15),
            Instruction::Store(0),
            Instruction::Clear(1),
        ])
    );
}
