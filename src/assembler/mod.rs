use anyhow::Result;
pub use parser::parse_file;
use std::{fs, path::Path};

use crate::emulator::{CPUError, Instruction};
mod parser;

pub fn assemble_file(path: &Path) -> Result<()> {
    let file = fs::read_to_string(path)?;

    let binary = assemble(parser::parse_file(&file)?)?;

    //todo - come up with something to return this error instead of unwrapping
    let outfile = path.file_stem().unwrap();

    fs::write(outfile, &binary)?;
    Ok(())
}

pub fn assemble(prog: Vec<Instruction>) -> Result<Vec<u8>, CPUError> {
    prog.into_iter()
        .map(|i| i.assemble())
        .collect::<Result<Vec<u8>, _>>()
}
