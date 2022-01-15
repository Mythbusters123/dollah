use std::env;
use std::process::{Command, exit, Child};
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.iter().nth(1).unwrap_or(&" ".to_owned()) == &" ".to_owned() {
        eprintln!("Not enough arguments... Exiting.");
        exit(1);
    }

    let mut program: Child = Command::new(args.iter().nth(1).unwrap_or(&"echo \"No Program supplied\"".to_owned()))
        .args(args.iter().skip(2)).spawn().expect("Could not start process.");

    let ecode = program.wait().expect("Failed to wait on process.");

    exit(ecode.code().expect("Error getting code."))
}
