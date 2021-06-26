use std::collections::HashMap;

use crate::instruction::Instruction;

//take a line, parse the relevant symbols/words, return the instruction it represents
//instructions are newline-seperated, and of format
// label: opcode operand ; comment
// returns Some(Instruction), or None if there's some parse error
fn parse_line(line: &str, lineno: usize, table: &mut HashMap<&str, u8>) -> Option<Instruction> {
    //copy the instruction so we have ownership of it
    let mut line: String = String::from(line);

    let mut label: Option<&str> = None;
    let mut operand: Option<u8> = None;

    //remove the comment from the line, if one exists
    if let Some(i) = line.find(';') {
        line.truncate(i);
    }

    //get the label, if the line has one
    if let Some(i) = line.find(':') {
        label = Some(&line[0..i]);
    }

    //just left with opcode operand at this point
    let mut split = line.trim().split_whitespace();
    let opcode = split.next()?;

    if let Some(operand_str) = split.next() {
        operand = operand_str.parse::<u8>().ok()
    }

    let instruction = match opcode {
        "CLEAR" => Instruction::CLEAR,
        "INC" => Instruction::INC,
        "ADD" => Instruction::ADD(operand?),
        "DEC" => Instruction::DEC,
        "JMP" => Instruction::JMP(operand?),
        "BUZ" | "BNZ" | "BZC" | "BNE" => Instruction::BNZ(operand?),
        "LOAD" => Instruction::LOAD(operand?),
        "STORE" => Instruction::STORE(operand?),
        _ => return None,
    };

    return Some(instruction);
}

//the main assembler function
//takes a vec of strings (the file) and returns a vec of instructions
pub fn parse_file(file: &str) -> Vec<u8> {
    //keeps track of symbols and their names/locations
    let mut symbol_table: HashMap<&str, u8> = HashMap::new();
    let mut binary: Vec<u8> = Vec::new();

    let mut instructions = file.lines().enumerate();

    while let Some((lineno, line)) = instructions.next() {
        binary.push(
            parse_line(&line, lineno, &mut symbol_table)
                .unwrap()
                .assemble(),
        );
    }
    return binary;
}
