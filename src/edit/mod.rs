// For command line arguments
use clap::ArgMatches;

use ::{ext, Error, ErrorKind, home_dir, Result};

///
/// Edit the argument. Given the value `matches->filepath`, resolve filepath relative to the
/// ellipsis home directory, then invoke the user's `$EDITOR` on the result. If the user doesn't
/// have an `$EDITOR` set, use `vi`.
///
/// # Errors
/// Return `Err` if the caller's does not exist or is not a directory.
///
/// # Panics
/// This function panics if [`home_dir()`] returns `None`, i.e. the ellipsis home directory cannot
/// be found
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
