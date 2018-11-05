// For command line arguments
use clap::ArgMatches;

// Import our result
use ::{Result,home_dir};
// Import our landing

///
/// Print the ellipsis home directory
/// This function takes no arguments, and prints the value of the ellipsis home directory.
/// It returns `Ok(())` in all non-panicking situations.
///
/// # Panics
/// This function panics if the program cannot locate the ellipsis home directory
pub fn run(_matches : &ArgMatches) -> Result<()> {

    println!("{}", home_dir()
             .expect("Couldn't find home dir!")
             .to_string_lossy());
    Ok(())
}

// Include tests
#[cfg(test)]
mod test;
