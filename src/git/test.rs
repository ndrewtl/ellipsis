use std::fs;
use ::{home_dir,test};

#[test]
#[ignore]
// This subcommand makes heavily reliance on an external binary,
// which appears to be particularly problematic for remote coverage reports.
// Don't test unless explicitly specified
pub fn test() {
    let _tdirs = test::TestDirs::new();

    let home = home_dir().unwrap();

    fs::create_dir(&home).unwrap();

    // git status should fail
    assert!(test::run_cmd(vec!["ellipsis", "git", "status"]).is_err());

    // The .git directory shouldn't exist
    assert!(! &home.join(".git").exists() );

    // Init a repository
    assert!(test::run_cmd(vec!["ellipsis", "git", "init"]).is_ok());

    // Now it should exist
    assert!( &home.join(".git").exists() );

    // Config
    assert!(test::run_cmd(vec!["ellipsis", "git",
                          "config", "user.email", "you@example.com"]).is_ok());
    assert!(test::run_cmd(vec!["ellipsis", "git",
                          "config", "user.name", "Your Name"]).is_ok());

    // git status should succeed now
    assert!(test::run_cmd(vec!["ellipsis", "git" ,"status"]).is_ok());

    // This isn't a command, it should fail
    assert!(test::run_cmd(vec!["ellipsis", "git", "sxkla"]).is_err());

    let filename = "test.txt";

    // Try to add a file, should fail
    assert!(test::run_cmd(vec!["ellipsis", "git", "add", filename]).is_err());

    // Test commit should fail
    assert!(test::run_cmd(vec!["ellipsis",
                          "git", "commit", "-m'Test commit'"]).is_err());

    // Write the file
    fs::write(&home.join(filename), b"file contents").unwrap();

    // Try to add the file again, should succeed
    assert!(test::run_cmd(vec!["ellipsis", "git", "add", filename]).is_ok());

    // Test commit should succeed now
    assert!(test::run_cmd(vec!["ellipsis",
                          "git", "commit", "-m'Test commit'"]).is_ok());
}
