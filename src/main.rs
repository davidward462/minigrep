// Bring module into scope for getting command line arguments
use std::env;

fn main() {

    // Get arguments and convert to collection (vector of strings)
    let args: Vec<String> = env::args().collect();

    // Debug macro
    dbg!(args);
}
