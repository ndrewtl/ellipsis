// Unit tests for the config submodule
// These tests focus mainly on locating the config and editing it in the correct way

// For creating and destroying fake test dirs
use ::test::TestDirs;
use ::{config_file, home_dir}; // Test this
// Module to test
use ::config;
// For validating json
use serde_json;
// use fs: to test content of file
use std::fs;

// For getting our app
use ::app;


#[test]
fn test_should_create_config_file() {
    // Create this variable for RAII
    let _tdirs = TestDirs::new();

    // Create our home dir
    fs::create_dir(home_dir().unwrap()).unwrap();

    let matches = app()
        .get_matches_from( vec!["ellipsis", "config"] );
    let sub_matches = matches.subcommand_matches("config").unwrap();

    let cfgfile = config_file().unwrap();


    // Assert that the file doesn't exist yet
    assert!(!cfgfile.exists());
    assert!(config::run(&sub_matches).is_ok());

    // Assert that the file now exists (i.e. it was created)
    assert!(cfgfile.exists());

    // Get the content of the file
    let file = fs::File::open(&cfgfile).unwrap();
    let test_data : serde_json::Value =
        serde_json::from_reader(file).unwrap();

    let known_data = json!({
        "links": {
            cfgfile.file_name().unwrap().to_string_lossy() : "~/.dot.json"
        }
    });

    assert_eq!(known_data, test_data);
}
