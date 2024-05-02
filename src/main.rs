use std::env; // command line arguments
use std::fs; // read file
use std::process; // exit process

// Hold program configuration data
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // Return a result with a Config instance on success
    // The error value is a string literal with the 'static lifetime
    fn build(args:&[String]) -> Result<Config, &'static str>
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

fn main() {

    // Get arguments and convert to collection (vector of strings)
    let args: Vec<String> = env::args().collect();

    // Call build() function implemented on the Config struct
    // The unwrap_or_else calls anonymous function on error
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    println!("Search term: {}", config.query);
    println!("File: {}", config.file_path);

    // Read file
    let text = fs::read_to_string(config.file_path).expect("Should have been able to read the file.");

    // Print text from file
    println!("Text:\n{}", text);

    // Debug macro
    dbg!(args);
}
