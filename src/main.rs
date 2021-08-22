use std::{env, format, path::Path};

mod assembler;
mod emulator;
mod instruction;

fn main() -> Result<(), String> {
    let mut args = env::args();
    args.next();
    let cmd = args
        .next()
        .ok_or("Please specify either assemble or emulate")?;

    let file = args.next().ok_or("Please specify a file")?;

    match cmd.as_str() {
        "assemble" => assembler::assemble(Path::new(&file)),
        "emulate" => emulator::emulate(Path::new(&file)),
        _ => Err(format!("Command {} not recognised.", cmd)),
    }
}
