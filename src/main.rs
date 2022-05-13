use anyhow::Result;
use std::{fs, path::Path};

mod assembler;
mod cli;
mod emulator;
mod parser;

fn main() -> Result<()> {
    let matches = cli::args().get_matches();

    if matches.is_present("emulate") {
        emulator::execute_file(Path::new(matches.value_of("emulate").unwrap()))?
    } else if matches.is_present("assemble") {
        assembler::assemble_file(Path::new(matches.value_of("assemble").unwrap()))?
    } else if matches.is_present("run") {
        run_file(Path::new(matches.value_of("run").unwrap()))?
    } else {
        //shouldn't happen if cli parser does its job
        //unwraps should never error either
        panic!()
    }
    Ok(())
}

pub fn run_file(path: &Path) -> Result<()> {
    let file = fs::read_to_string(path)?;

    let binary = assembler::assemble(parser::parse_file(&file)?)?;

    let final_state = emulator::execute_program(&binary)?;

    println!("Final CPU State: \n{}", final_state);

    Ok(())
}
