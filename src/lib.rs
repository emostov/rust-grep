use std::{error::Error, fs};

// Box<dyn Error> indicates the fn will return a type that implements the Error trait
// dyn keyword is short for dynamic. It gives us the ability to return different
// error types
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Opens file and either gives back the string of the file or
    // returns a Result with an error
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // Here, clone is not the most efficient means, but it keeps the code simple
        // and we only do it once so its const is constant
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let querry = "duct";
        let contents = "\
Rust:
safe, fast, productive
Pick three.";

        assert_eq!(vec!["safe, fast, productive"], search(querry, contents));
    }
}
