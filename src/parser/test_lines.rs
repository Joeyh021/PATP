#![cfg(test)]

use super::*;

//test basic opcode operand lines
#[test]
fn parse_line_basic() {
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
fn parse_line_comments() {
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
fn parse_line_blanks() {
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
fn parse_line_errors() {
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
}
