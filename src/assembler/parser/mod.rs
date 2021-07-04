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
// opcode operand ; comment
// returns Some(Instruction), or None if there's some parse error
fn parse_line(line: &str) -> Result<Instruction, ParseError> {
    //copy the instruction so we have ownership of it
    let mut line_slice: &str = line.trim();

    //if the line starts with a comment (;) or is entirely whitespace, then return a blank line
    if line_slice.starts_with(';') || line_slice == "" {
        return Err(ParseError::Blank);
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
    let opcode = split
        .next()
        .ok_or(ParseError::err("Could not parse opcode"))?;

    //if theres a second item in the iterator and it can be parsed to a u8 , store it as the operand
    if let Some(operand_str) = split.next() {
        operand = operand_str.parse::<u8>().ok()
    }

    //should be no opcode with clear inc stop or dec
    if (opcode == "CLEAR" || opcode == "DEC" || opcode == "INC" || opcode == "STOP")
        && operand != Option::None
    {
        return Err(ParseError::err("No operand expected here"));
    }

    //iterator should now be empty no matter what
    if let Some(_) = split.next() {
        return Err(ParseError::err("Too many operands"));
    }

    //match opcodes to instructions
    //if we need an opcode and we cant get one, an error is returned here
    let instruction = match opcode {
        "CLEAR" => Instruction::CLEAR(0),
        "STOP" => Instruction::CLEAR(1),
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
    let mut binary: Vec<u8> = Vec::new();

    let mut instructions = file.lines().enumerate();

    while let Some((lineno, line)) = instructions.next() {
        match parse_line(&line).map(|i| i.assemble()) {
            Err(ParseError::Blank) => (),
            Err(ParseError::Error(str)) => panic!("Error on line {}: {}", lineno, str),
            Ok(byte) => binary.push(byte),
        }
    }
    return binary;
}

#[cfg(test)]
mod test_files;

#[cfg(test)]
mod test_lines;
