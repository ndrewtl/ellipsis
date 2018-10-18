// For command line arguments
use clap::ArgMatches;

use ::{ext, Error, ErrorKind, home_dir, Result};

// Edit the given file
pub fn run(matches : &ArgMatches) -> Result<()> {

    let home = home_dir().expect("Couldn't find home dir");

    if home.exists() {

        if home.is_dir() {

            let fpath = matches
                .value_of("filepath")
                .unwrap_or("");

            ext::editor(home_dir()
                   .expect("Couldn't find home dir")
                   .join(&fpath))?;

            Ok(())

        } else {
            Err(Error::new(ErrorKind::IO,
                           format!("{} is not a directory",
                                   home.to_string_lossy())))
        }


    } else {
        Err(Error::new(ErrorKind::IO,
                       format!("Home directory {} does not exist",
                               home.to_string_lossy())))
    }

}

#[cfg(test)]
mod test;
