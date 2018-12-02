extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    println!("Initializing Program...\n");

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("A problem occurred validating the arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("The program failed: {}", e);
        process::exit(1);
    }
}