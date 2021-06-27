use super::*;

//test basic opcode operand lines
#[test]
fn parse_line_basic() {
    assert_eq!(
        parse_line("CLEAR", 0, &mut HashMap::new()),
        Ok(Instruction::CLEAR)
    );

    assert_eq!(
        parse_line("ADD 12", 0, &mut HashMap::new()),
        Ok(Instruction::ADD(12))
    );
    assert_eq!(
        parse_line("BUZ 30", 0, &mut HashMap::new()),
        Ok(Instruction::BNZ(30))
    );
    assert_eq!(
        parse_line("STORE 18", 0, &mut HashMap::new()),
        Ok(Instruction::STORE(18))
    );
}

//test lines with comments and some weird whitespacing
#[test]
fn parse_line_comments() {
    assert_eq!(
        parse_line("CLEAR;", 0, &mut HashMap::new()),
        Ok(Instruction::CLEAR)
    );
    assert_eq!(
        parse_line("DEC       ; test comment  ", 0, &mut HashMap::new()),
        Ok(Instruction::DEC)
    );
    assert_eq!(
        parse_line("ADD  100     ; test comment  ", 0, &mut HashMap::new()),
        Ok(Instruction::ADD(100))
    );
    assert_eq!(
        parse_line(
            "   LOAD  40     ; test comment DEC  ",
            0,
            &mut HashMap::new()
        ),
        Ok(Instruction::LOAD(40))
    );
    assert_eq!(
        parse_line("   LOAD  40     ;; test;  ", 0, &mut HashMap::new()),
        Ok(Instruction::LOAD(40))
    );
}

//some lines that should return blanks
#[test]
fn parse_line_blanks() {
    assert_eq!(
        parse_line(";     ", 0, &mut HashMap::new()),
        Err(ParseError::Blank)
    );
    assert_eq!(
        parse_line("            ;     ", 0, &mut HashMap::new()),
        Err(ParseError::Blank)
    );
    assert_eq!(
        parse_line("                 ", 0, &mut HashMap::new()),
        Err(ParseError::Blank)
    );
    assert_eq!(
        parse_line(" ;;           ;;  ;   ;;   ", 0, &mut HashMap::new()),
        Err(ParseError::Blank)
    );
    assert_eq!(
        parse_line("            ; test comment    ", 0, &mut HashMap::new()),
        Err(ParseError::Blank)
    );
    assert_eq!(
        parse_line(";test comment     ", 0, &mut HashMap::new()),
        Err(ParseError::Blank)
    );
    assert_eq!(
        parse_line(";     ", 0, &mut HashMap::new()),
        Err(ParseError::Blank)
    );
}
