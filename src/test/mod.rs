extern crate rand;
use hostname;
use std::{ffi,env};

use ::{app, config_file, home_dir, run, Result, Scope}; // methods to test

// Include TestDirs struct
pub mod testdirs;
pub use self::testdirs::TestDirs;

// Run the given command
pub fn run_cmd<I, T>(itr : I) -> Result<()>
    where
    I : IntoIterator<Item = T>,
    T : Into<ffi::OsString> + Clone
{
    let matches = app()
        .get_matches_from(itr);

    run(&matches)

}

#[test]
fn test_config_file() {
    let tdirs = TestDirs::new();
    let hname = hostname::get_hostname().unwrap();

    assert_eq!(config_file(Scope::Local).unwrap(),
        tdirs.data_dir().join(format!("ellipsis/{}.dot.json", hname)));
}

#[test]
fn test_home_dir() {
    let tdirs = TestDirs::new();

    // First test that our home dir is in $XDG_DATA_HOME/ellipsis
    assert_eq!(home_dir().unwrap(),
        tdirs.data_dir().join("ellipsis"));

    // next test that our home dir is in $HOME/.ellipsis
    env::remove_var("XDG_DATA_HOME");

    assert_eq!(home_dir().unwrap(),
        tdirs.home_dir().join(".ellipsis"));
}

// Test valid and invalid commands
#[cfg(test)]
mod app_test;

// Test external commands
#[cfg(test)]
mod ext_test;
