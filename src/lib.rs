use std::error::Error;
use std::fs; // read file
use std::env; // environment variables

// Hold program configuration data
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
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

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case
        })
    }
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
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

    // make query lowercase and store as string slice
    let query = query.to_lowercase();

    // iterate through
    for line in contents.lines() {

        // make line lowercase before checking
        if line.to_lowercase().contains(&query) {
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
    let found_lines = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    }
    else {
        search_case_sensitive(&config.query, &content)
    };

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
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}










