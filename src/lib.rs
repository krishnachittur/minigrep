use std::fs;
use std::env;
use std::error::Error;

#[cfg(test)]
mod tests;

pub struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing query argument")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing filename argument")
        };
        Ok(Config{query, filename})
    }
}

pub fn search(query: &str, contents: &str) -> Vec<String> {
    contents.lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(num, line)| format!("{}: {}", num, line))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

