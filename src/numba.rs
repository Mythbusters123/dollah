use std::env;
use std::process::{Command, exit, Child};
fn main() {
    let args: Vec<String> = env::args().collect();

    let sudo_bin: String;

    match env::var("SUDO") {
        Ok(val) => {sudo_bin = val},
        Err(_) => {sudo_bin = "/usr/bin/sudo".to_string()}
    }

    if args.iter().nth(1).unwrap_or(&" ".to_owned()) == &" ".to_owned() {
        eprintln!("Not enough arguments... Exiting.");
        exit(1);
    }

    let sudo_bin = sudo_bin.to_owned();

    let mut program: Child = Command::new(sudo_bin).args(args.iter().skip(1)).spawn().expect("Could not start process.");

    let ecode = program.wait().expect("Failed to wait on process.");

    exit(ecode.code().expect("Error getting code."))
}
