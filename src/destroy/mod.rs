// For command line arguments
use clap;

use std::fs;

// Import our result
use ::{Result,home_dir};

/// Delete the ellipsis home directory
///
/// This function requires `matches` to contain exactly one of the `--force` and `dry-run` flags,
/// which can also be abbreviated `-f` and `-n`, respectively. If `--force` is passed, recursively
/// delete [`home_dir()`]. If `--dry-run` is passed, print to stdout each file that would be
/// removed.  In either case, it returns `Ok(())`.
///
/// # Errors
/// This function returns an error if rust cannot remove [`home_dir()`]
///
/// # Panics
/// This function panics if [`home_dir()`] returns [`None`] or neither the `--force` or `--dry-run`
/// flags are passed.
pub fn run(matches : &clap::ArgMatches) -> Result<()> {
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
