use std::env;
use std::fs;

// Responsible for parsing logic, setting up configuration, calling a
// run function in lib.rs, and handling errors from lib.rs
// main.rs concern is running the program.
fn main() {
    // Gather and creat vector cl args pass to program, will not accept invlid unicode
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    // Opens file and returns a Result<String> of the contents
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        // Here, clone is not the most efficient means, but it keeps the code simple
        // and we only do it once so its const is constant
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
