use crate::assembler::{assemble, parse_file};
use crate::emulator::execute_program;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn run_file(path: &Path) -> Result<()> {
    let file = fs::read_to_string(path)?;

    let binary = assemble(parse_file(&file)?)?;

    let final_state = execute_program(&binary)?;

    println!("Final CPU State: \n{}", final_state);

    Ok(())
}
