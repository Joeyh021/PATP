use std::{fs, path::Path};

mod parser;

pub fn assemble(path: &Path) -> Result<(), String> {
    let file = fs::read_to_string(path).map_err(|err| err.to_string())?;

    let binary: Vec<u8> = parser::parse_file(&file)?;

    let outfile = path.file_stem().unwrap();

    fs::write(outfile, &binary).map_err(|err| err.to_string())?;
    Ok(())
}
