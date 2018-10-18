// For command line arguments
use clap::{ArgMatches};

use std::fs;

// Import our result
use ::{Result,home_dir};

// The function we're exporting
// Delete the ellipsis home directory
pub fn run(matches : &ArgMatches) -> Result<()> {
    let home = home_dir().expect("Couldn't find the ellipsis home dir");

    // Figure out which option we were given...
    if matches.is_present("force") { // If we're given --force | -f ...
        // Remove our directory
        eprintln!("Removing {}", home.to_string_lossy());
        fs::remove_dir_all(home)?;

    } else if matches.is_present("dry-run") { // If we're given --dry-run | -n ...
        // Tell the user what would happen
        println!("Would remove {}", home.to_string_lossy());

    } else { // We should never get here
        // So panic
        panic!("Must provide one of: --force, --dry-run")
    }

    Ok(())
}

#[cfg(test)]
mod test;
