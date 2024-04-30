// Bring module into scope for getting command line arguments
use std::env;

fn main() {

    // Get arguments and convert to collection (vector of strings)
    let args: Vec<String> = env::args().collect();

    let search_string = &args[1];
    let file_path = &args[2];

    println!("Search term: {}", search_string);
    println!("File: {}", file_path);

    // Debug macro
    dbg!(args);
}
