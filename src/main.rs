use std::env::args;
use std::fs;
use std::iter::Iterator;

mod instruction;
mod parser;

fn main() {
    //get the args from the command line as a vector of Strings

    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("please enter a filename")
    }

    let filename: &str = &args[1];

    let file: String = fs::read_to_string(filename).expect("Could not open file!");

    let binary: Vec<u8> = parser::parse_file(&file);

    fs::write("output", &binary).expect("Error writing to file");
}
