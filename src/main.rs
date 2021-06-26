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
    let filename = args[1];

    let lines = fs::read_to_string(filename)
        .expect("Could not open file!")
        .lines()
        .collect();

    let binary: Vec<u8> = parser::parse_file(lines);

    fs::write("output", &binary).expect("Error writing to file");
}
