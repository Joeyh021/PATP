use std::collections::HashMap;

use crate::instruction::Instruction;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Blank,
    Error(String),
}

impl ParseError {
    fn err(err: &str) -> Self {
        ParseError::Error(String::from(err))
    }
}

//take a line, parse the relevant symbols/words, return the instruction it represents
//instructions are newline-seperated, and of format
// label: opcode operand ; comment
// returns Some(Instruction), or None if there's some parse error
fn parse_line(
    line: &str,
    _lineno: usize,
    _table: &mut HashMap<&str, u8>,
) -> Result<Instruction, ParseError> {
    //copy the instruction so we have ownership of it
    let mut line: String = String::from(line.trim());

    //if the line starts with a comment (;) or is entirely whitespace, then return a blank line
    if line.starts_with(';') || line == "" {
        return Err(ParseError::Blank);
    }

    let mut _label: Option<&str> = None;
    let mut operand: Option<u8> = None;

    //remove the comment from the line, if one exists
    if let Some(i) = line.find(';') {
        line.truncate(i);
    }

    //get the label, if the line has one
    if let Some(i) = line.find(':') {
        _label = Some(&line[0..i]);
    }

    //just left with opcode operand at this point
    let mut split = line.trim().split_whitespace();

    //get the opcode as the first item in the iterator, error if not possible
    let opcode = split
        .next()
        .ok_or(ParseError::err("Could not parse opcode"))?;

    //if theres a second item in the iterator and it can be parsed to a u8, store it as the operand
    if let Some(operand_str) = split.next() {
        operand = operand_str.parse::<u8>().ok()
    }

    //should be no opcode with clear inc or dec
    if (opcode == "CLEAR" || opcode == "DEC" || opcode == "INC") && operand != Option::None {
        return Err(ParseError::err("No operand expected here"));
    }

    //iterator should now be empty no matter what
    if let Some(_) = split.next() {
        return Err(ParseError::Error(String::from("Too many operands")));
    }

    //match opcodes to instructions
    //if we need an opcode and we cant get one, an error is returned here
    let instruction = match opcode {
        "CLEAR" => Instruction::CLEAR,
        "INC" => Instruction::INC,
        "ADD" => Instruction::ADD(operand.ok_or(ParseError::err("Could not parse operand"))?),
        "DEC" => Instruction::DEC,
        "JMP" => Instruction::JMP(operand.ok_or(ParseError::err("Could not parse operand"))?),
        "BUZ" | "BNZ" | "BZC" | "BNE" => {
            Instruction::BNZ(operand.ok_or(ParseError::err("Could not parse operand"))?)
        }
        "LOAD" => Instruction::LOAD(operand.ok_or(ParseError::err("Could not parse operand"))?),
        "STORE" => Instruction::STORE(operand.ok_or(ParseError::err("Could not parse operand"))?),
        _ => return Err(ParseError::err("Invalid opcode")),
    };

    return Ok(instruction);
}

//the main parser function
//takes a large string (the file) and returns a vec of instructions
pub fn parse_file(file: &str) -> Vec<u8> {
    //keeps track of symbols and their names/locations
    let mut symbol_table: HashMap<&str, u8> = HashMap::new();
    let mut binary: Vec<u8> = Vec::new();

    let mut instructions = file.lines().enumerate();

    while let Some((lineno, line)) = instructions.next() {
        match parse_line(&line, lineno, &mut symbol_table).map(|i| i.assemble()) {
            Err(ParseError::Blank) => (),
            Err(ParseError::Error(str)) => panic!("Error on line {}: {}", lineno, str),
            Ok(byte) => binary.push(byte),
        }
    }
    return binary;
}

#[cfg(test)]
mod test;
