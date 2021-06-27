use super::*;

#[test]
fn parse_file_empty() {
    assert_eq!(parse_file(""), Vec::new());
    assert_eq!(parse_file("         "), Vec::new());
    assert_eq!(parse_file("       \n  "), Vec::new());
    assert_eq!(parse_file("       \n  ;   \n"), Vec::new());
}

#[test]
fn parse_file_one() {
    assert_eq!(parse_file("\n CLEAR \n"), vec![0]);
    assert_eq!(
        parse_file("CLEAR \n ADD 15 \n STORE 0"),
        vec![0, 0b010_01111, 0b111_00000]
    );
}
