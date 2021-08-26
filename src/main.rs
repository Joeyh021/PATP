use anyhow::Result;
use std::path::Path;

mod assembler;
mod cli;
mod emulator;
mod instruction;
mod run;

fn main() -> Result<()> {
    let matches = cli::args().get_matches();

    if matches.is_present("emulate") {
        emulator::execute_file(Path::new(matches.value_of("emulate").unwrap()))?
    } else if matches.is_present("assemble") {
        assembler::assemble_file(Path::new(matches.value_of("assemble").unwrap()))?
    } else if matches.is_present("run") {
        run::run_file(Path::new(matches.value_of("run").unwrap()))?
    } else {
        //shouldn't happen if cli parser does its job
        //unwraps should never error either
        panic!()
    }
    Ok(())
}
