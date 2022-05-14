use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, space0, space1, u8},
    combinator::{flat_map, opt, rest, success},
    error::VerboseError,
    sequence::{pair, preceded, separated_pair, terminated},
    IResult,
};

use crate::instruction::Instruction;

mod test;

// lines have format label: opcode operand; comment
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line {
    number: usize,
    label: Option<String>,
    opcode: String,
    operand: Option<Operand>,
}

//operand is either a label or a number
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operand {
    Number(u8),
    Label(String),
}

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Could not parse opcode on line {0}")]
    #[allow(dead_code)]
    OpcodeError(usize),

    #[error("Could not parse operand on line {0}")]
    OperandError(usize),

    #[error("Unexpected operand found on line {0}")]
    #[allow(dead_code)]
    UnexpectedOperand(usize),

    #[error("Invalid opcode on line {0}")]
    InvalidOpcode(usize),

    #[error("Unknown parse error on line {0}")]
    #[allow(dead_code)]
    Unknown(usize),

    #[error("Program is too long: contains {0} instructions (max 32)")]
    ProgramTooLong(usize),

    #[error("Unknown symbol {0} on line {1}")]
    InvalidSymbol(String, usize),

    #[error("Invalid operand on line {0}: operand overflows 5-bit limit")]
    OperandOverflow(usize),
}

//parse opcode-operand
fn ops(i: &str) -> IResult<&str, (&str, Option<Operand>), VerboseError<&str>> {
    //no operand
    let clear = pair(tag("CLEAR"), success(None));
    let inc = pair(tag("INC"), success(None));
    let dec = pair(tag("DEC"), success(None));
    let stop = pair(tag("STOP"), success(None));

    let add = separated_pair(
        tag("ADD"),
        space1,
        flat_map(u8, |o| success(Some(Operand::Number(o)))),
    );

    let load = separated_pair(
        tag("LOAD"),
        space1,
        flat_map(u8, |o| success(Some(Operand::Number(o)))),
    );

    let store = separated_pair(
        tag("STORE"),
        space1,
        flat_map(u8, |o| success(Some(Operand::Number(o)))),
    );

    let jump = separated_pair(
        tag("JUMP"),
        space1,
        flat_map(alphanumeric1, |l: &str| {
            success(Some(Operand::Label(l.to_owned())))
        }),
    );
    let branch = separated_pair(
        alt((tag("BNZ"), tag("BUZ"), tag("BZC"), tag("BNE"))),
        space1,
        flat_map(alphanumeric1, |l: &str| {
            success(Some(Operand::Label(l.to_owned())))
        }),
    );

    alt((add, load, store, jump, clear, branch, inc, dec, stop))(i)
}

//parse the label off the front of the instruction
fn label(i: &str) -> IResult<&str, Option<&str>, VerboseError<&str>> {
    opt(terminated(terminated(alphanumeric1, tag(":")), space0))(i)
}

//parse an entire instruction
//top-level nom parser
//should return result with no input left
fn instruction(i: &str, line_no: usize) -> IResult<&str, Line, VerboseError<&str>> {
    //trim whitespace and newlines
    let i = i.trim();

    //get the label
    let (i, label) = label(i)?;

    //get the operation
    let (i, (opcode, operand)) = ops(i)?;

    //make sure the rest of the input is either comment or whitespace
    let (i, _) = alt((preceded(preceded(space0, tag(";")), rest), space0))(i)?;

    Ok((
        i,
        Line {
            label: label.map(str::to_owned),
            opcode: opcode.to_owned(),
            number: line_no,
            operand,
        },
    ))
}

pub fn parse_file(file: &str) -> Result<Vec<Instruction>, ParseError> {
    let lines = file.lines().enumerate().map(|(line_no, line)| {
        instruction(line, line_no)
            .map_err(|error| {
                dbg!(&error);
                ParseError::Unknown(line_no)
            })
            .map(|x| x.1)
    });
    let lines: Vec<Line> = lines.collect::<Result<_, _>>()?;

    //if program too long, then yeet
    if lines.len() >= 32 {
        return Err(ParseError::ProgramTooLong(lines.len()));
    }

    //build symbol table
    let symbols: HashMap<String, usize> = lines
        .iter()
        .flat_map(|line| line.label.clone().map(|l| (l, line.number)))
        .collect();

    //process lines into instruction
    let instructions = lines.into_iter().map(|line| parse_line(line, &symbols));
    instructions.collect()
}

fn parse_line(line: Line, symbols: &HashMap<String, usize>) -> Result<Instruction, ParseError> {
    let operand = line
        .operand
        .map(|operand| match operand {
            Operand::Number(x) => Ok(x),
            Operand::Label(l) => {
                let n = symbols
                    .get(&l)
                    .ok_or(ParseError::InvalidSymbol(l, line.number))?;

                (*n).try_into()
                    .map_err(|_| ParseError::OperandOverflow(line.number))
            }
        })
        .transpose()?;

    let instruction = match line.opcode.as_str() {
        "CLEAR" => Instruction::Clear(0),
        "STOP" => Instruction::Clear(1),
        "INC" => Instruction::Inc,
        "ADD" => Instruction::Add(operand.ok_or(ParseError::OperandError(line.number))?),
        "DEC" => Instruction::Dec,
        "JMP" => Instruction::Jump(operand.ok_or(ParseError::OperandError(line.number))?),
        "BUZ" | "BNZ" | "BZC" | "BNE" => {
            Instruction::Bnz(operand.ok_or(ParseError::OperandError(line.number))?)
        }
        "LOAD" => Instruction::Load(operand.ok_or(ParseError::OperandError(line.number))?),
        "STORE" => Instruction::Store(operand.ok_or(ParseError::OperandError(line.number))?),
        _ => return Err(ParseError::InvalidOpcode(line.number)),
    };
    Ok(instruction)
}
