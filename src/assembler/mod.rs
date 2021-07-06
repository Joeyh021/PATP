use std::{fs, io::Error, path::Path};

mod parser;

pub fn assemble(path: &Path) -> Result<(), Error> {
    let file = fs::read_to_string(path)?;

    let binary: Vec<u8> = parser::parse_file(&file);

    let outfile = path.file_stem().unwrap();

    fs::write(outfile, &binary)?;
    Ok(())
}
