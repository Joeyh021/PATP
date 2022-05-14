use anyhow::Result;
use clap::{ArgEnum, Parser};
use std::{fs, path::Path};

mod assembler;
mod emulator;
mod instruction;
mod parser;

fn main() -> Result<()> {
    let args = Cli::parse();
    let file: &Path = args.file.as_ref();

    match args.command {
        Command::Assemble => emulator::execute_file(file)?,
        Command::Emulate => assembler::assemble_file(Path::new(file))?,
        Command::Run => run_file(Path::new(file))?,
    }
    Ok(())
}

pub fn run_file(path: &Path) -> Result<()> {
    let file = fs::read_to_string(path)?;

    let binary = assembler::assemble(parser::parse_file(&file)?)?;

    let final_state = emulator::execute_program(&binary)?;

    println!("Final CPU State: \n{}", final_state);

    Ok(())
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(arg_enum)]
    command: Command,
    #[clap(validator = file_exists)]
    file: String,
}

#[derive(Copy, Clone, Debug, ArgEnum)]
enum Command {
    Assemble,
    Emulate,
    Run,
}

fn file_exists(f: &str) -> Result<(), &'static str> {
    let p = std::path::Path::new(f);
    if !p.is_file() {
        Err("File does not exist.")
    } else {
        Ok(())
    }
}
