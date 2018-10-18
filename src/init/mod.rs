// For command line arguments
use clap;

// For pulling the repository
use git2;

// Import our result
use ::{Result,home_dir};

// The function we're exporting
// Clone the given URI as the ellipsis home directory
pub fn run(matches : &clap::ArgMatches) -> Result<()> {

    // The URI to clone from
    let uri = matches.value_of("URI")
        .expect("URI to clone from must be provided!");

    // The directory to clone into
    let destination = home_dir()
        .expect("Couldn't find location where dotfiles should be stored");

    eprintln!("Cloning {} into {}", uri, destination.to_string_lossy());

    // Clone
    git2::Repository::clone(&uri, &destination)
        .expect("Failed to clone repository");

    Ok(())
}

// Include tests
#[cfg(test)]
mod test;
