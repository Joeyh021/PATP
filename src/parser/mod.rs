use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, space0, space1, u8},
    combinator::{flat_map, opt, rest, success},
    error::{convert_error, VerboseError},
    sequence::{pair, preceded, separated_pair, terminated},
    Finish, IResult,
};
use thiserror::Error;

use crate::instruction::Instruction;

mod test;

// lines have format label: opcode operand; comment
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Line {
    number: usize,
    label: Option<String>,
    opcode: String,
    operand: Option<Operand>,
}

//operand is either a label or a number
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Operand {
    Number(u8),
    Label(String),
}

//various errors that may occur at different stages of parsing
#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Unrecognised input on line {0}. Parser trace:\n{1}")]
    BadInput(usize, String),

    #[error("Could not parse operand on line {0}")]
    MissingOperandError(usize),

    #[error("Unexpected symbol found on line {0}")]
    #[allow(dead_code)]
    UnexpectedSymbol(usize),

    #[error("Invalid opcode on line {0}")]
    InvalidOpcode(usize),

    #[error("Unknown parse error on line {0}")]
    #[allow(dead_code)]
    Unknown(usize),

    #[error("Program is too long: contains {0} instructions (max 32)")]
    ProgramTooLong(usize),

    #[error("Unknown symbol {0} on line {1}")]
    InvalidSymbol(String, usize),

    #[error("Invalid operand on line {0}: operand is greater than 32")]
    OperandOverflow(usize),
}

//parse a file, retunrning a vec of all the instructions
pub fn parse_file(file: &str) -> Result<Vec<Instruction>, ParseError> {
    let lines = file
        .lines()
        .map(|l| l.trim()) // remove whitespace from each line
        .filter(|l| !(l.starts_with(';') || l.is_empty())) //remove empty or comment-only lines
        .enumerate() //get line numbers
        .map(|(line_no, line)| {
            //map the parser over every line
            instruction(line, line_no)
                .finish() //convert the errors, TODO: add more context to parsers and make this less basic
                .map_err(|error| ParseError::BadInput(line_no, convert_error(line, error))) //map nom errors into our errors
                .map(|x| x.1) //drop the remaining input, we only want the line
        });
    let lines: Vec<Line> = lines.collect::<Result<_, _>>()?; //collect into result

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

//parse a single line (with symbol table, converting any symbols), returning the parsed instruction
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

    if let Some(operand) = operand {
        if operand >= 32 {
            return Err(ParseError::OperandOverflow(line.number));
        }
    }

    let instruction = match line.opcode.as_str() {
        "CLEAR" => Instruction::Clear(0),
        "STOP" => Instruction::Clear(1),
        "INC" => Instruction::Inc,
        "ADD" => Instruction::Add(operand.ok_or(ParseError::MissingOperandError(line.number))?),
        "DEC" => Instruction::Dec,
        "JMP" | "JUMP" => {
            Instruction::Jump(operand.ok_or(ParseError::MissingOperandError(line.number))?)
        }
        "BUZ" | "BNZ" | "BZC" | "BNE" => {
            Instruction::Bnz(operand.ok_or(ParseError::MissingOperandError(line.number))?)
        }
        "LOAD" => Instruction::Load(operand.ok_or(ParseError::MissingOperandError(line.number))?),
        "STORE" => Instruction::Store(operand.ok_or(ParseError::MissingOperandError(line.number))?),
        _ => return Err(ParseError::InvalidOpcode(line.number)),
    };
    Ok(instruction)
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
