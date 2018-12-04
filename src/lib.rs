use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitivity_flag: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() != 3 {
        return Err("Invalid arguments: --filename and --query parameters are required!");
    }

    let filename = args[1].clone();
    let query = args[2].clone();
    let case_sensitivity_flag = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config { query, filename, case_sensitivity_flag })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  println!("Search for query: {:?}", config.query);
  println!("In file: {:?}", config.filename);

  let contents = fs::read_to_string(config.filename)?;

  println!("File contents: {:?}", contents);

  let result = if config.case_sensitivity_flag {
    println!("Running case sensitive search!");
    search(&config.query, &contents)
  } else {
    println!("Running case insensitive search!");
    search_case_insensitive(&config.query, &contents)
  };

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

fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }

  results
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result_case_sensitive() {
    let query = "Pick";
    let result = vec!["Pick three."];

    assert_eq!(
      result,
      search(query, get_contents())
    );
  }

  #[test]
  fn two_results_case_insensitive() {
    let query = "Rust";
    let result = vec!["Rust:", "Trust me, really -"];

    assert_eq!(
      result,
      search_case_insensitive(query, get_case_sensitive_contents())
    );
  }

  #[test]
  fn no_result_case_sensitive() {
    let badQuery = "rust";
    let emptyResult: Vec<&str> = Vec::new();

    assert_eq!(
      emptyResult,
      search(badQuery, get_contents())
    );
  }

  #[test]
  fn one_result_case_insensitive() {
    let query = "duct";
    let result = vec!["safe, productive, fast."];

    assert_eq!(
      result,
      search_case_insensitive(query, get_contents())
    );
  }

  fn get_contents<'a>() -> &'a str {
"Rust:
safe, productive, fast.
Pick three."
  }

  fn get_case_sensitive_contents<'a>() -> &'a str {
"Rust:
Trust me, really -
or not"
  }
}