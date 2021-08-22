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
    assert_eq!(parse_file("\n CLEAR \n"), Ok(vec![0]));
    assert_eq!(
        parse_file("CLEAR \n ADD 15 \n STORE 0"),
        Ok(vec![0, 0b0100_1111, 0b1110_0000])
    );
}

#[test]
fn parse_file_errors() {
    assert_eq!(
        parse_file(" CLEAR \n ADD 78"),
        Err(ParseError::AssemblyError(1))
    );
    assert_eq!(
        parse_file(" CLEAR \n ADD x"),
        Err(ParseError::OperandParseError(1))
    );
    assert_eq!(
        parse_file(" CLEAR \n SUB 12"),
        Err(ParseError::InvalidOpcode(1))
    );
}
