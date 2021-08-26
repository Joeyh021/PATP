use anyhow::Result;
pub use parser::parse_file;
use std::{fs, path::Path};
mod parser;

pub fn assemble_file(path: &Path) -> Result<()> {
    let file = fs::read_to_string(path)?;

    let binary: Vec<u8> = parser::parse_file(&file)?;

    let outfile = path.file_stem().unwrap();

    fs::write(outfile, &binary)?;
    Ok(())
}
