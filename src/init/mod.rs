// For command line arguments
use clap;

// For pulling the repository
use git2;

// Import our result
use ::{Result,home_dir};

pub fn run(matches : &clap::ArgMatches) -> Result<()> {

    // The directory to clone into
    let destination = home_dir()
        .expect("Couldn't find location where dotfiles should be stored");

    // The URI to clone from
    if let Some(uri) = matches.value_of("URI") {

        // If we have one, we'll clone it in here
        eprintln!("Cloning {} into {}", uri, destination.to_string_lossy());

        // Clone
        git2::Repository::clone(&uri, &destination)?;

    } else {

        // If not, we'll just initialize one here
        eprintln!("Creating new git repository in {}", destination.to_string_lossy());
        git2::Repository::init(&destination)?;
    }

    Ok(())
}

// Include tests
#[cfg(test)]
mod test;
