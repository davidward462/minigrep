use std::env; // command line arguments
use std::process; // exit process

// struct
use minigrep::Config;

fn main() {

    // Get arguments and convert to collection (vector of strings)
    let args: Vec<String> = env::args().collect();

    // Call build() function implemented on the Config struct
    // The unwrap_or_else calls anonymous function on error
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application erorr: {e}");
        process::exit(1);
    }
}











