// For command line arguments
use clap;

// for running external command
use std::vec;

// Import our result and home dir
use ::{Error, ErrorKind, ext, Result, home_dir};

///
/// ellipsis git -> forward all given arguments to the git command
///
/// This function takes all given arguments in the variable `matches`, and forwards them to the
/// `git(1)` command. This command is executed in the context of the ellipsis home directory.
///
/// # Errors
///
/// This function returns an error if the git command returns an error code. It also errors if the
/// home directory does not exist or cannot be found.
///
/// # Panics
/// This function panics if the ellipsis home directory cannot be located, that is if
/// ['home_dir()'] returns `None`.
pub fn run(matches : &clap::ArgMatches) -> Result<()> {

    let home = home_dir().expect("ellipsis home can't be found");

    if home.exists() {

        let args : vec::Vec<&str> = matches.values_of("gitargs").unwrap().collect();

        if ext::git(args).is_ok() {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Git, String::from("git command failed")))
        }

    } else {

        let msg = format!("Directory {} does not exist",
                 home.to_string_lossy());

        Err(Error::new(ErrorKind::IO, msg))
    }


}

#[cfg(test)]
mod test;
