// For command line arguments
use clap::ArgMatches;

// Import our result
use ::{config_file, Result, Scope};


// For paths
use std::{fs,path};

// For json
use serde_json;

// For expanding ~
use shellexpand;

// The function we're exporting
// Delete links created by the `link` command
pub fn run(_matches : &ArgMatches) -> Result<()> {

    let cfg = config_file(Scope::Local)
        .expect("Couldn't find config file");


    let file = fs::File::open(&cfg)?;

    // Read json into dat
    let dat : serde_json::Value = serde_json::from_reader(file)?;

    // Export links into a map
    let links = dat["links"]
        .as_object()
        .expect("Problem parsing JSON");

    // Each pair in this map is of the format
    // File -> symlink destination
    for filename in links.values() {

        let filename = path::PathBuf::from(
            shellexpand::tilde(filename.as_str().unwrap()).into_owned());

        // Panic if wee're about to remove a non-symlink
        if ! filename.symlink_metadata()?.file_type().is_symlink() {
            panic!("File {} is not a symlink", &filename.to_string_lossy());
        }

        eprintln!("{} {}",
                 "RM     ",
                 filename.to_string_lossy());
        fs::remove_file(&filename)?;

    }


    Ok(())
}

#[cfg(test)]
mod test;
