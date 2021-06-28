use super::*;

//test basic opcode operand lines
#[test]
fn parse_line_basic() {
    assert_eq!(parse_line("CLEAR"), Ok(Instruction::CLEAR));

    assert_eq!(parse_line("ADD 12"), Ok(Instruction::ADD(12)));
    assert_eq!(parse_line("BUZ 30"), Ok(Instruction::BNZ(30)));
    assert_eq!(parse_line("STORE 18"), Ok(Instruction::STORE(18)));
}

//test lines with comments and some weird whitespacing
#[test]
fn parse_line_comments() {
    assert_eq!(parse_line("CLEAR;"), Ok(Instruction::CLEAR));
    assert_eq!(
        parse_line("DEC       ; test comment  "),
        Ok(Instruction::DEC)
    );
    assert_eq!(
        parse_line("ADD  100     ; test comment  "),
        Ok(Instruction::ADD(100))
    );
    assert_eq!(
        parse_line("   LOAD  40     ; test comment DEC  "),
        Ok(Instruction::LOAD(40))
    );
    assert_eq!(
        parse_line("   LOAD  40     ;; test;  "),
        Ok(Instruction::LOAD(40))
    );
}

//some lines that should return blanks
#[test]
fn parse_line_blanks() {
    assert_eq!(parse_line(";     "), Err(ParseError::Blank));
    assert_eq!(parse_line("            ;     "), Err(ParseError::Blank));
    assert_eq!(parse_line("                 "), Err(ParseError::Blank));
    assert_eq!(
        parse_line(" ;;           ;;  ;   ;;   "),
        Err(ParseError::Blank)
    );
    assert_eq!(
        parse_line("            ; test comment    "),
        Err(ParseError::Blank)
    );
    assert_eq!(parse_line(";test comment     "), Err(ParseError::Blank));
    assert_eq!(parse_line(";     "), Err(ParseError::Blank));
}

//make sure we get the right errors
#[test]
fn parse_line_errors() {
    assert_eq!(parse_line("ABC"), Err(ParseError::err("Invalid opcode")));
    assert_eq!(
        parse_line("DEC 12"),
        Err(ParseError::err("No operand expected here"))
    );
    assert_eq!(
        parse_line("DEC 12; INC"),
        Err(ParseError::err("No operand expected here"))
    );
    assert_eq!(
        parse_line("STORE 12 INC"),
        Err(ParseError::err("Too many operands"))
    );
    assert_eq!(
        parse_line("ADD x ;"),
        Err(ParseError::err("Could not parse operand"))
    );
    assert_eq!(
        parse_line("DEEZ NUTS ; haha"),
        Err(ParseError::err("Invalid opcode"))
    );
}

#[test]
fn parse_line_labels() {}
