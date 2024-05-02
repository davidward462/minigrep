// Bring module into scope for getting command line arguments
use std::env;
// For reading file
use std::fs;

// Hold program configuration data
struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args:&[String]) -> Config
{
    let query = args[1].clone();
    let file_path = args[2].clone();

    // return the config struct
    Config {query, file_path}
}

fn main() {

    // Get arguments and convert to collection (vector of strings)
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Search term: {}", config.query);
    println!("File: {}", config.file_path);

    // Read file
    let text = fs::read_to_string(config.file_path).expect("Should have been able to read the file.");

    // Print text from file
    println!("Text:\n{}", text);

    // Debug macro
    dbg!(args);
}
