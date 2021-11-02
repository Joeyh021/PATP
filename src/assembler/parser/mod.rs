use crate::emulator::Instruction;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Could not parse opcode on line {0}")]
    OpcodeParseError(usize),

    #[error("Could not parse operand on line {0}")]
    OperandParseError(usize),

    #[error("Unexpected operand found on line {0}")]
    UnexpectedOperand(usize),

    #[error("Invalid opcode on line {0}")]
    InvalidOpcode(usize),

    #[error("Unknown parse error on line {0}")]
    #[allow(dead_code)]
    Unknown(usize),

    #[error("Line {0} is blank")]
    Blank(usize),
}

//take a line, parse the relevant symbols/words, return the instruction it represents
//instructions are newline-seperated, and of format
// opcode operand ; comment
// returns Some(Instruction), or None if there's some parse error
fn parse_line(line: &str, lineno: usize) -> Result<Instruction, ParseError> {
    //copy the instruction so we have ownership of it
    let mut line_slice: &str = line.trim();

    //if the line starts with a comment (;) or is entirely whitespace, then return a blank line
    if line_slice.starts_with(';') || line_slice.is_empty() {
        return Err(ParseError::Blank(lineno));
    }

    let mut operand: Option<u8> = None;

    //remove the comment from the line, if one exists
    if let Some(i) = line_slice.find(';') {
        line_slice = &line_slice[..i];
    }

    // labels are unimplemented for now
    // //get the label, if the line has one
    // if let Some(i) = line_slice.find(':') {
    //     //add the label to the symbol table
    //     table.insert(&line[0..i], lineno as u8);
    // }

    //just left with opcode operand at this point
    let mut split = line_slice.trim().split_whitespace();

    //get the opcode as the first item in the iterator, error if not possible
    let opcode = split.next().ok_or(ParseError::OpcodeParseError(lineno))?;

    //if theres a second item in the iterator and it can be parsed to a u8, store it as the operand
    if let Some(operand_str) = split.next() {
        operand = operand_str.parse::<u8>().ok()
    }

    //should be no opcode with clear inc stop or dec
    if (opcode == "CLEAR" || opcode == "DEC" || opcode == "INC" || opcode == "STOP")
        && operand != Option::None
    {
        return Err(ParseError::UnexpectedOperand(lineno));
    }

    //iterator should now be empty no matter what
    if split.next().is_some() {
        return Err(ParseError::UnexpectedOperand(lineno));
    }

    //match opcodes to instructions
    //if we need an opcode and we cant get one, an error is returned here
    let instruction = match opcode {
        "CLEAR" => Instruction::Clear(0),
        "STOP" => Instruction::Clear(1),
        "INC" => Instruction::Inc,
        "ADD" => Instruction::Add(operand.ok_or(ParseError::OperandParseError(lineno))?),
        "DEC" => Instruction::Dec,
        "JMP" => Instruction::Jump(operand.ok_or(ParseError::OperandParseError(lineno))?),
        "BUZ" | "BNZ" | "BZC" | "BNE" => {
            Instruction::Bnz(operand.ok_or(ParseError::OperandParseError(lineno))?)
        }
        "LOAD" => Instruction::Load(operand.ok_or(ParseError::OperandParseError(lineno))?),
        "STORE" => Instruction::Store(operand.ok_or(ParseError::OperandParseError(lineno))?),
        _ => return Err(ParseError::InvalidOpcode(lineno)),
    };

    Ok(instruction)
}

//the main parser function
//takes a large string (the file) and returns a vec of instructions
pub fn parse_file(file: &str) -> Result<Vec<Instruction>, ParseError> {
    //keeps track of symbols and their names/locations
    let mut program: Vec<Instruction> = Vec::new();

    for (lineno, line) in file.lines().enumerate() {
        match parse_line(line, lineno) {
            Err(ParseError::Blank(_)) => (),
            Err(e) => return Err(e),
            Ok(instruction) => program.push(instruction),
        }
    }
    Ok(program)
}

#[cfg(test)]
mod test_files;

#[cfg(test)]
mod test_lines;
