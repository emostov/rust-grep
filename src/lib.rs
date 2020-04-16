use std::{env, error::Error, fs};

// Box<dyn Error> indicates the fn will return a type that implements the Error trait
// dyn keyword is short for dynamic. It gives us the ability to return different
// error types
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Opens file and either gives back the string of the file or
    // returns a Result with an error
    let contents = fs::read_to_string(config.filename)?;

    let search_results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in search_results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // emv::Args returns an iterator
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // skip the first value since it is the program name
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't give a filename"),
        };

        // We dont care about the value of the variable, just wether or not it is
        // set
        // Returns a result. If Case insensitive is set, is_err will return false
        // If it is unset, that is technically an error and will return true
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut matches = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            matches.push(line);
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let querry = "duct";
        let contents = "\
Rust:
safe, fast, productive
Pick three.
Duct";

        assert_eq!(vec!["safe, fast, productive"], search(querry, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
