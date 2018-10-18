// For command line arguments
use clap::ArgMatches;

// Use Error type and config_file() path method
use ::{Result, config_file, ext};

// For file writing
use std::fs;
use serde_json;


// The function we're exporting
// Given the current matches, open, read, or modify the config file as necessary
pub fn run(matches : &ArgMatches) -> Result<()> {

    // Get and print config file
    let cfg = config_file().unwrap();
    println!("{}", &cfg.to_string_lossy());

    if !cfg.exists() { // The config file doesn't exist, create one
        eprintln!("File not found, creating...");
        let dat = json!({
            "links": {
                cfg.file_name().unwrap().to_string_lossy() : "~/.dot.json"
            }
        });

        let file = fs::File::create(&cfg)?;
        serde_json::to_writer_pretty(file,&dat)?;
    }

    // If the 'edit' option is present, we open the file to edit
    if matches.is_present("edit") {
        ext::editor(&cfg)?;
    }
    Ok(())
}

// Include tests
#[cfg(test)]
mod test;
