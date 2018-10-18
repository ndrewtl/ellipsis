// Argument parsing
#[macro_use]
extern crate clap;
// Colors in the terminal
extern crate colored;
// Dirs crate
extern crate dirs;
// Git stuff
extern crate git2;
// Get this machine's hostname
extern crate hostname;
// Working with JSON
#[macro_use]
extern crate serde_json;
// For getting full paths
extern crate shellexpand;

use std::{env, path};

// Use external commands
pub mod ext;

// Use error types
pub mod error;
pub use ::error::Error;
pub use ::error::ErrorKind;

// Use Result type
pub mod result;
pub use ::result::Result;

// Use app for arg parsing
pub mod app;
pub use ::app::app;

// Include commands
pub mod config;
pub mod destroy;
pub mod edit;
pub mod git;
pub mod home;
pub mod init;
pub mod link;
pub mod unlink;

pub fn run(matches : &clap::ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("config",  Some(sub_matches)) => config::run(&sub_matches),
        ("destroy", Some(sub_matches)) => destroy::run(&sub_matches),
        ("edit",    Some(sub_matches)) => edit::run(&sub_matches),
        ("git",     Some(sub_matches)) => git::run(&sub_matches),
        ("home",    Some(sub_matches)) => home::run(&sub_matches),
        ("init",    Some(sub_matches)) => init::run(&sub_matches),
        ("link",    Some(sub_matches)) => link::run(&sub_matches),
        ("unlink",  Some(sub_matches)) => unlink::run(&sub_matches),
        _ => Ok(())
    }
}

// Return the "home directory" where the config files will be stored
pub fn home_dir() -> Option<path::PathBuf> {
    if let Ok(str) = env::var("XDG_DATA_HOME") {
        Some(path::PathBuf::from(str).join("ellipsis"))
    } else {

        if let Some(myhome) = dirs::home_dir() {
            Some(myhome.join(".ellipsis"))
        } else {
            None
        }
    }
}

// Return the location of this device's .dot.json
pub fn config_file() -> Option<path::PathBuf> {
    if let Some(home) = home_dir() {
        if let Some(hname) = hostname::get_hostname() {
            Some(home.join(format!("{}.dot.json", hname)))
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod test;
