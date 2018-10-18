// Unit tests for the config submodule
// These tests focus mainly on locating the config and editing it in the correct way

// For getting our app
use ::app;
// For creating and destroying fake test dirs
use ::test::TestDirs;
use ::home_dir;
// Module to test
use ::destroy;
// use fs: to test content of file
use std::fs;
extern crate rand;


#[test]
fn test_should_destroy_empty_dir() {
    let _tdirs = TestDirs::new();

    let home = home_dir().unwrap();

    // home doesn't exist
    assert!(!&home.exists());

    // Create it, now it should exist
    fs::create_dir(&home).unwrap();
    assert!(&home.exists());

    // Destroy it, now it should be gone again
    let matches = app()
        .get_matches_from( vec!["ellipsis", "destroy", "--force"] );
    let sub_matches = matches.subcommand_matches("destroy").unwrap();
    assert!(destroy::run(&sub_matches).is_ok());
    assert!(! &home.exists());

}

#[test]
fn test_should_destroy_dir() {
    let _tdirs = TestDirs::new();

    let home = home_dir().unwrap();

    // home doesn't exist
    assert!(!&home.exists());

    // Create it, now it should exist
    fs::create_dir(&home).unwrap();
    fs::write(&home.join("test.txt"), b"file contents").unwrap();
    assert!(&home.exists());

    // Destroy it, now it should be gone again
    let matches = app()
        .get_matches_from( vec!["ellipsis", "destroy", "--force"] );
    let sub_matches = matches.subcommand_matches("destroy").unwrap();
    assert!(destroy::run(&sub_matches).is_ok());
    assert!(! &home.exists());
}

#[test]
fn dry_run_should_not_destroy_dir() {
    let _tdirs = TestDirs::new();

    let home = home_dir().unwrap();

    // home doesn't exist
    assert!(!&home.exists());

    // Create directory
    fs::create_dir(&home).unwrap();
    assert!(&home.exists());

    // Initialize file with random value
    let testfile = home.join("text.txt");
    let random_string = rand::random::<u32>().to_string();
    fs::write(&testfile, &random_string).unwrap();

    // Run with dry run, make sure it still exists and has same value
    let matches = app()
        .get_matches_from( vec!["ellipsis", "destroy", "--dry-run"] );
    let sub_matches = matches.subcommand_matches("destroy").unwrap();
    assert!(destroy::run(&sub_matches).is_ok());
    assert!(&home.exists());

    let content = fs::read_to_string(&testfile).unwrap();
    assert_eq!(content, random_string);
}
