mod cpu;
mod error;

pub use crate::instruction::Instruction;
use anyhow::Result;
use cpu::Cpu;
pub use error::CPUError;
use std::{fs, path::Path};

pub fn execute_program(program: &[u8]) -> Result<Cpu> {
    let mut state = Cpu::new().load(program)?;
    loop {
        let instruction = state.memory[state.pc as usize];
        let new_state = state.execute(instruction);
        match new_state {
            None => return Ok(state),
            Some(new_state) => state = new_state,
        }
    }
}

pub fn execute_file(path: &Path) -> Result<()> {
    let file = fs::read(path)?;
    let final_state = execute_program(&file)?;
    println!("Final CPU State: \n{}", final_state);
    Ok(())
}
