use clap::{App, Arg, ArgGroup};

pub fn args() -> clap::App<'static, 'static> {
    App::new("PATP Toolkit")
        .version("0.1.2")
        .author("Joey Harrison")
        .about("An emulator and assembler for the Pedagogically Advanced Teaching Processor")
        .arg(
            Arg::with_name("emulate")
                .takes_value(true)
                .short("e")
                .long("emulate")
                .value_name("FILE")
                .help("Emulate the program in the given file"),
        )
        .arg(
            Arg::with_name("assemble")
                .takes_value(true)
                .short("a")
                .long("assemble")
                .value_name("FILE")
                .help("Assemble the code in the given file"),
        )
        .arg(
            Arg::with_name("run")
                .takes_value(true)
                .short("r")
                .long("run")
                .value_name("FILE")
                .help("Assemble and then emulate the code in the given file"),
        )
        .group(
            ArgGroup::with_name("operation")
                .args(&["assemble", "emulate", "run"])
                .multiple(false)
                .required(true),
        )
}
