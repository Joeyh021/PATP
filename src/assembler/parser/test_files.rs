use super::*;

#[test]
fn parse_file_empty() {
    assert_eq!(parse_file(""), Ok(Vec::new()));
    assert_eq!(parse_file("         "), Ok(Vec::new()));
    assert_eq!(parse_file("       \n  "), Ok(Vec::new()));
    assert_eq!(parse_file("       \n  ;   \n"), Ok(Vec::new()));
}

#[test]
fn parse_file_one() {
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

#[test]
fn parse_file_errors() {
    assert_eq!(
        parse_file(" CLEAR \n ADD x"),
        Err(ParseError::OperandParseError(1))
    );
    assert_eq!(
        parse_file(" CLEAR \n SUB 12"),
        Err(ParseError::InvalidOpcode(1))
    );
}
