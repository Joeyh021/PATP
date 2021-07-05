use std::{env, path::Path};

mod assembler;
mod emulator;
mod instruction;

fn main() {
    let mut args = env::args();
    args.next();
    let cmd = args.next().expect("please specify a command");
    let file = args.next().expect("please specify a file name");
    match cmd.as_str() {
        "assemble" => assembler::assemble(Path::new(&file)),
        "emulate" => emulator::emulate(Path::new(&file)),
        _ => println!("command not recognised! exiting.."),
    }
}
