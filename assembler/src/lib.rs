use std::{fs, path::Path};

mod instruction;
mod parser;

pub fn assemble(path: &Path) {
    let file = fs::read_to_string(path).expect("Could not open file!");

    let binary: Vec<u8> = parser::parse_file(&file);

    fs::write("output", &binary).expect("Error writing to file");
}
