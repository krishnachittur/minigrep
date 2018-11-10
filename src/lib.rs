use std::fs;
use std::error::Error;

#[cfg(test)]
mod tests;

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

pub fn search<'a>(query: &str, contents: &str) -> Vec<String> {
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

