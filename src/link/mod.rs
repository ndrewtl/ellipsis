// For command line arguments
use clap::ArgMatches;

// Import our result
use ::{config_file, home_dir, Result, Scope};

// For reading files
use std::fs;

// For paths
use std::path;

// For json
use serde_json;

// For getting our paths straightened out
use shellexpand;

// For symlinks
use std::os::unix;

// The function we're exporting
// Link the files
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
    for (dest, source) in links {

        // Parse the file into a PathBuf
        let source = path::PathBuf::from(home_dir().unwrap())
            .join(source.as_str().unwrap());

        // Tilde-expand the destination file
        let dest = path::PathBuf::from(
            shellexpand::tilde(dest.as_str()).into_owned());

        // Make sure the parent directory exists
        if let Some(dest_dir) = dest.parent() {
            // If the directory we're going to link into doesn't exist...
            if ! dest_dir.exists() {
                // Create it
                eprintln!("{} {}",
                          "MKDIRP ",
                          dest_dir.to_string_lossy());
                fs::create_dir_all(&dest_dir)?;
            }
        }


        eprintln!("{} {} -> {}",
                  "SYMLINK",
                  source.to_string_lossy(),
                  dest.to_string_lossy());
        unix::fs::symlink(source,dest)?;

    }


    Ok(())
}

#[cfg(test)]
mod test;
