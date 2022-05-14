use anyhow::Result;
use clap::{ArgEnum, Parser};

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Assemble => patp::execute_file(args.file)?,
        Command::Emulate => patp::assemble_file(args.file)?,
        Command::Run => patp::run_file(args.file)?,
    }
    Ok(())
}

//cli/clap stuff

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
