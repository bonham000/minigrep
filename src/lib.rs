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

    Ok(())
}