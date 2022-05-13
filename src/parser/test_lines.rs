use super::*;

//test basic opcode operand lines
#[test]
fn parse_line_basic() {
    assert_eq!(parse_line("CLEAR", 0), Ok(Instruction::Clear(0)));

    assert_eq!(parse_line("ADD 12", 0), Ok(Instruction::Add(12)));
    assert_eq!(parse_line("BUZ 30", 0), Ok(Instruction::Bnz(30)));
    assert_eq!(parse_line("STORE 18", 0), Ok(Instruction::Store(18)));
    assert_eq!(parse_line("STOP", 0), Ok(Instruction::Clear(1)));
}

//test lines with comments and some weird whitespacing
#[test]
fn parse_line_comments() {
    assert_eq!(parse_line("CLEAR;", 0), Ok(Instruction::Clear(0)));
    assert_eq!(
        parse_line("DEC       ; test comment  ", 0),
        Ok(Instruction::Dec)
    );
    assert_eq!(
        parse_line("ADD  100     ; test comment  ", 0),
        Ok(Instruction::Add(100))
    );
    assert_eq!(
        parse_line("   LOAD  40     ; test comment DEC  ", 0),
        Ok(Instruction::Load(40))
    );
    assert_eq!(
        parse_line("   LOAD  40     ;; test;  ", 0),
        Ok(Instruction::Load(40))
    );
}

//some lines that should return blanks
#[test]
fn parse_line_blanks() {
    assert_eq!(parse_line(";     ", 0), Err(ParseError::Blank(0)));
    assert_eq!(
        parse_line("            ;     ", 0),
        Err(ParseError::Blank(0))
    );
    assert_eq!(
        parse_line("                 ", 0),
        Err(ParseError::Blank(0))
    );
    assert_eq!(
        parse_line(" ;;           ;;  ;   ;;   ", 0),
        Err(ParseError::Blank(0))
    );
    assert_eq!(
        parse_line("            ; test comment    ", 0),
        Err(ParseError::Blank(0))
    );
    assert_eq!(
        parse_line(";test comment     ", 0),
        Err(ParseError::Blank(0))
    );
    assert_eq!(parse_line(";     ", 0), Err(ParseError::Blank(0)));
}

//make sure we get the right errors
#[test]
fn parse_line_errors() {
    assert_eq!(parse_line("ABC", 0), Err(ParseError::InvalidOpcode(0)));
    assert_eq!(
        parse_line("DEC 12", 0),
        Err(ParseError::UnexpectedOperand(0))
    );
    assert_eq!(
        parse_line("DEC 12; INC", 0),
        Err(ParseError::UnexpectedOperand(0))
    );
    assert_eq!(
        parse_line("STORE 12 INC", 0),
        Err(ParseError::UnexpectedOperand(0))
    );
    assert_eq!(
        parse_line("ADD x ;", 0),
        Err(ParseError::OperandParseError(0))
    );
    assert_eq!(
        parse_line("STOP 14 ;", 0),
        Err(ParseError::UnexpectedOperand(0))
    );
    assert_eq!(
        parse_line("DEEZ NUTS ; haha", 0),
        Err(ParseError::InvalidOpcode(0))
    );
}
