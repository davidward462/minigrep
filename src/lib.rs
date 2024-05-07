use std::error::Error;
use std::fs; // read file

// Hold program configuration data
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // Return a result with a Config instance on success
    // The error value is a string literal with the 'static lifetime
    pub fn build(args:&[String]) -> Result<Config, &'static str>
    {
        // check args length
        if args.len() < 3 {
            return Err("Usage: cargo run -- <query> <filepath>");
        }

        // save arguments
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    let mut results = Vec::new();

    // iterate through 
    for line in contents.lines() {
        if line.contains(query) {
        results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    let mut results = Vec::new();

    // iterate through 
    for line in contents.lines() {
        if line.contains(query) {
        results.push(line);
        }
    }
    results
}

// Main logic of program
pub fn run(config: Config) -> Result<(), Box<dyn Error>>
{
    // Read file
    let content = fs::read_to_string(config.file_path)?;
    
    // Get lines that contain query
    let found_lines = search(&config.query, &content);

    // Print found lines
    for line in found_lines {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn case_sensitive()
    {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}










