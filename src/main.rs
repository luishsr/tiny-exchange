use std::{process, str::Bytes};
use tinyexchange::Command;

// The entry point of a CLI-based exchange
fn main() {
    // Load arguments from the program call into a variable
    let args: Vec<String> = std::env::args().collect();

    let command = Command::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Calls the config run()
    if let Err(e) = tinyexchange::run(command) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
