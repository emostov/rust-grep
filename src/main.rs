use std::{env, process};
use rustgrep::Config;

// Responsible for parsing logic, setting up configuration, calling a
// run function in lib.rs, and handling errors from lib.rs
// main.rs concern is running the program.
fn main() {
    // Gather and creat vector cl args pass to program, will not accept invlid unicode
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!(
            "There wasv an ISSUE with how you entered arguments :( : {}",
            err
        );
        // Non-zero exit status code to indicate to calling process that the
        // program exited with an error
        process::exit(1);
    });

    // dont use unwrap_or_else here because we dont care about ret val when it works
    if let Err(e) = rustgrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}




