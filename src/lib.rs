use std::fs;
use std::error::Error;

pub struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            Err("Usage: <program> <query> <filename>")
        } else {
            Ok(Config{query: args[1].clone(), filename: args[2].clone()})
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {} in file {}", config.query, config.filename);
    let contents = fs::read_to_string(&config.filename)?;
    println!("{}", contents);
    Ok(())
}

