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
    let query = &args[1];
    let file_path = &args[2];

    // return the config struct
    Config {query, file_path}
}

fn main() {

    // Get arguments and convert to collection (vector of strings)
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    println!("Search term: {}", query);
    println!("File: {}", file_path);

    // Read file
    let text = fs::read_to_string(file_path).expect("Should have been able to read the file.");

    // Print text from file
    println!("Text:\n{}", text);

    // Debug macro
    dbg!(args);
}
