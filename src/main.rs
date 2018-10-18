// Import lib.rs
extern crate ellipsis;
// For colors
extern crate colored;
use colored::Colorize;

// For exit codes
use std::process;


fn main() {
    let matches = ellipsis::app().get_matches();

    if let Err(e) = ellipsis::run(&matches) {
        // Print error kind and message
        eprintln!("{}: {}",
                  e.kind().to_string().red(),
                  e.message());
        // Failure
        process::exit(1);
    } else {
        // Success
        process::exit(0);
    }
}
