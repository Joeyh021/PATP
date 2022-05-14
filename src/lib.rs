#![allow(clippy::enum_variant_names)]

use std::fs;
use std::path::Path;

use anyhow::Result;
//expose some bits that may be useful
pub use cpu::Cpu;
pub use instruction::Instruction;
pub use parser::parse_file;

mod cpu;
mod instruction;
mod parser;

//reads a text file from disk, assembles the instructions and writes a binary file to disk with the assembled program
pub fn assemble_file(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    let file = fs::read_to_string(path)?;

    let binary = assemble_instructions(&parse_file(&file)?)?;

    //todo - come up with something to return this error instead of unwrapping
    let outfile = path.file_stem().unwrap();

    fs::write(outfile, &binary)?;
    Ok(())
}

//executes a binary file on disk, printing the final state to stdout
pub fn execute_file(path: impl AsRef<Path>) -> Result<()> {
    let file = fs::read(path)?;
    let final_state = execute_program(&file)?;
    println!("Final CPU State: \n{}", final_state);
    Ok(())
}

//reads a text file from disk, assembles it, and then runs it, printing the final state to stdout
pub fn run_file(path: impl AsRef<Path>) -> Result<()> {
    let file = fs::read_to_string(path)?;

    let instructions = parser::parse_file(&file)?;

    let binary: Result<Vec<u8>, _> = instructions.into_iter().map(|i| i.assemble()).collect();

    let final_state = execute_program(&binary?)?;

    println!("Final CPU State: \n{}", final_state);

    Ok(())
}

//takes a list of instructions and assembles them, returning a vec of bytes
pub fn assemble_instructions(instructions: &[Instruction]) -> Result<Vec<u8>, cpu::CPUError> {
    instructions
        .iter()
        .map(|i| (*i).assemble())
        .collect::<Result<Vec<u8>, _>>()
}

//takes a program as a list of bytes and executes it, returning the final CPU state
pub fn execute_program(program: &[u8]) -> Result<Cpu> {
    let mut state = Cpu::new().load(program)?;
    loop {
        let instruction = state.fetch();
        let new_state = state.execute(instruction);
        match new_state {
            Ok(new_state) => state = new_state,
            Err(cpu::CPUError::Stop(end_state)) => return Ok(end_state),
            _ => unreachable!(),
        }
    }
}
