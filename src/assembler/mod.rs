use anyhow::Result;
use std::{fs, path::Path};

mod parser;

pub fn assemble(path: &Path) -> Result<()> {
    let file = fs::read_to_string(path)?;

    let binary: Vec<u8> = parser::parse_file(&file)?;

    let outfile = path.file_stem().unwrap();

    fs::write(outfile, &binary)?;
    Ok(())
}
