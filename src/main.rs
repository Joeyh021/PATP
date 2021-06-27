use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let cmd = args.next().expect("please specify a command");
    let file = args.next().expect("please specify a file name");
    println!("perform operation {} on file {}", cmd, file)
}
