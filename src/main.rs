use anyhow::Result;
use std::{env, path::Path};
mod assembler;
mod emulator;
mod instruction;

fn main() -> Result<()> {
    let mut args = env::args();
    args.next();
    let cmd = args.next().unwrap();

    let file = args.next().unwrap();

    match cmd.as_str() {
        "assemble" => assembler::assemble(Path::new(&file)),
        "emulate" => emulator::emulate(Path::new(&file)),
        _ => panic!("Command {} not recognised.", cmd),
    }
}
