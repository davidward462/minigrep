// Bring module into scope for getting command line arguments
use std::env;
// For reading file
use std::fs;

// Hold program configuration data
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args:&[String]) -> Config
    {
        let query = args[1].clone();
        let file_path = args[2].clone();

        // return the config struct
        Config {query, file_path}
    }
}

fn main() {

    // Get arguments and convert to collection (vector of strings)
    let args: Vec<String> = env::args().collect();

    // Call new() function implemented on the Config struct
    let config = Config::new(&args);

    println!("Search term: {}", config.query);
    println!("File: {}", config.file_path);

    // Read file
    let text = fs::read_to_string(config.file_path).expect("Should have been able to read the file.");

    // Print text from file
    println!("Text:\n{}", text);

    // Debug macro
    dbg!(args);
}
