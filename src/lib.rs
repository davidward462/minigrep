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

// Main logic of program
pub fn run(config: Config) -> Result<(), Box<dyn Error>>
{
    // Read file
    let _text = fs::read_to_string(config.file_path)?;

    Ok(())
}
