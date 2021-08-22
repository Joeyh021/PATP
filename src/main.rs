use anyhow::Result;
use std::path::Path;

mod assembler;
mod cli;
mod emulator;
mod instruction;

fn main() -> Result<()> {
    let matches = cli::args().get_matches();

    if matches.is_present("emulate") {
        emulator::emulate(Path::new(matches.value_of("emulate").unwrap()))?
    } else if matches.is_present("assemble") {
        assembler::assemble(Path::new(matches.value_of("assemble").unwrap()))?
    } else if matches.is_present("run") {
        todo!()
    } else {
        panic!()
    }
    Ok(())
}
