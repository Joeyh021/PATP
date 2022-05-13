use clap::{App, Arg, ArgGroup};

pub fn args() -> clap::App<'static, 'static> {
    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_AUTHORS");
    let description = env!("CARGO_PKG_DESCRIPTION");
    App::new("PATP Toolkit")
        .version(version)
        .author(author)
        .about(description)
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
