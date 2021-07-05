use std::{fs, path::Path};

mod parser;

pub fn assemble(path: &Path) {
    let file = fs::read_to_string(path).expect("Could not open file!");

    let binary: Vec<u8> = parser::parse_file(&file);

    let outfile = path.file_stem().unwrap();

    fs::write(outfile, &binary).expect("Error writing to file");
}
