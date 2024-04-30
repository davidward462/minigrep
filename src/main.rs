// Bring module into scope for getting command line arguments
use std::env;
// For reading file
use std::fs;

fn main() {

    // Get arguments and convert to collection (vector of strings)
    let args: Vec<String> = env::args().collect();

    let search_string = &args[1];
    let file_path = &args[2];

    println!("Search term: {}", search_string);
    println!("File: {}", file_path);

    // Read file
    let text = fs::read_to_string(file_path).expect("Should have been able to read the file.");

    // Print text from file
    println!("Text:\n{}", text);

    // Debug macro
    dbg!(args);
}
