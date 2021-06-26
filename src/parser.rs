use std::collections::HashMap;

//take a line, parse the relevant symbols/words, return the instruction it represents
//instructions are newline-seperated, and of format
// label: opcode operand ; comment
use crate::instruction::Instruction;

fn parseline(table: &mut HashMap<&str, u8>, line: &str, lineno: usize) -> Option<Instruction> {
    //copy the instruction so we have ownership of it
    let mut line: String = String::from(line);

    let mut label: Option<&str> = None;
    let mut operand: Option<u8> = None;

    //remove the comment from the line
    if let Some(i) = line.find(';') {
        line.truncate(i);
    }

    //store the label
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
    };
    return Some(instruction);
}

//the main assembler function
//takes a vec of strings (the file) and returns a vec of instructions
pub fn parse_file(lines: Vec<&str>) -> Vec<u8> {
    //keeps track of symbols and their names/locations
    let mut symbol_table: HashMap<&str, u8> = HashMap::new();
    let mut binary: Vec<u8> = Vec::new();

    let mut instructions = lines.iter().enumerate();

    while let Some((lineno, line)) = instructions.next() {
        binary.push(
            parseline(&mut symbol_table, line, lineno)
                .unwrap()
                .assemble(),
        );
    }
    return binary;
}
