// For command line arguments
use clap::ArgMatches;

// Import our result
use ::{Result,home_dir};
// Import our landing

// The function we're exporting
// Print the ellipsis home directory
pub fn run(_matches : &ArgMatches) -> Result<()> {

    println!("{}", home_dir()
             .expect("Couldn't find home dir!")
             .to_string_lossy());
    Ok(())
}

// Include tests
#[cfg(test)]
mod test;
