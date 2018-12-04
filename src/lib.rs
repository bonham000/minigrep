use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() != 3 {
            return Err("Invalid arguments: --filename and --query parameters are required!");
        }

        let filename = args[1].clone();
        let query = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Search for query: {:?}", config.query);
    println!("In file: {:?}", config.filename);

    let contents = fs::read_to_string(config.filename)?;

    println!("File contents: {:?}", contents);

    let result = search(&config.query, &contents);

    println!("{:#?}", result);

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let result = vec!["safe, productive, fast."];

        assert_eq!(
            result,
            search(query, get_contents())
        );
    }

    fn no_result() {
        let badQuery = "non";
        let emptyResult: Vec<&str> = Vec::new();

        assert_eq!(
            emptyResult,
            search(badQuery, get_contents())
        );
    }

    fn get_contents<'a>() -> &'a str {
"Rust:
safe, productive, fast.
Pick three."
    }
}