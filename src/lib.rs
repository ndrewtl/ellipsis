//! # ellipsis
//! a peace-of-mind configuration manager.
//!
//! This crate contains the internals of the ellipsis configuration manager, which is exported as a
//! command-line tool. In order to design a tool with nested subcommands and argument-parsing,
//! ellipsis depends on [`clap`]. It exports a [`run`] function that takes a `&`[`clap::ArgMatches`]
//! reference as its single input and returns a [`Result`]`<()>`. Each subcommand is a submodule,
//! which similarly exports a `run` function with the same signature. The global `run` function
//! delegates control to the submodule `run` functions to decouple functionality.
//!
//! Additionally, ellipsis provides a few modules as utilities. The [`ext`] module handles
//! 'external' commands-- i.e. using `git` or the `$EDITOR.`
//!
//! ellipsis re-exports [`app()`] function that returns the [`clap::App`] used for parsing
//! arguments. It also re-exports the crate's [`Error`] struct, and its [`Result`] type.
//!
//!
//!

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

// use scope for local vs. global scope
pub mod scope;
pub use ::scope::Scope;

// Include commands
pub mod config;
pub mod destroy;
pub mod edit;
pub mod git;
pub mod home;
pub mod init;
pub mod link;
pub mod unlink;

/// The top-level run command.
///
/// This command calls the function corresponding to `matches.subcommand_name()`. So if
/// `subcommand_name() == "config"`, this command will call [`config::run`], passing along
/// `matches.subcommand_matches()`.
///
/// # Errors
/// If this command calls a `run()` function that returns an [`Error`], this function returns that
/// error. Otherwise it returns `Ok(())`.
///
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

/// Return the location of the ellipsis "home directory," the git repository where all (actual)
/// configuration files are stored.
///
/// If the `$XDG_DATA_HOME` environment variable is set, this function will return
/// `Some($XDG_DATA_HOME/ellipsis)`. If not, it will return `Some(dirs::home_dir()/.ellipsis`).
/// If [`dirs::home_dir()`] returns [`None`], this function will return `None`.
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

/// The location of this device's config file.
///
/// This function returns [`home_dir()`]`/[hostname].dot.json`, if [`home_dir()`] is valid.  If
/// `home_dir()` returns [`None`], or if the hostname can't be found, this function returns
/// `None`.
pub fn config_file(cfg_t : Scope) -> Option<path::PathBuf> {
    if let Some(home) = home_dir() {
        match cfg_t {
            Scope::Local => {
                if let Some(hname) = hostname::get_hostname() {
                    Some(home.join(format!("{}.dot.json", hname)))
                } else {
                    None
                }
            },
            Scope::Global => {
                Some(home.join(String::from(".dot.json")))
            }
        }
    } else {
        None
    }
}

#[cfg(test)]
mod test;
