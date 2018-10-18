// For command line arguments
use clap;

// for running external command
use std::vec;

// Import our result and home dir
use ::{Error, ErrorKind, ext, Result, home_dir};

// The function we're exporting
// Print the ellipsis home directory
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
