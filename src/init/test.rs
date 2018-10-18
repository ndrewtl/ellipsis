use git2;
use ::test::TestDirs;
use ::{app,home_dir, init};

#[test]
fn test_init_local_repo() {
    let tdirs = TestDirs::new();

    // Create a dummy repo in our data dir
    let dummy_repo = tdirs.data_dir().join("dummy_repo");
    git2::Repository::init(&dummy_repo).unwrap();

    // Assert that we do not yet have a home dir
    assert!(! home_dir().unwrap().exists() );

    let matches = app()
        .get_matches_from( vec!["ellipsis", "init", &dummy_repo.to_string_lossy()] );

    let sub_matches = matches
        .subcommand_matches("init").unwrap();

    // Run the command
    assert!(init::run(&sub_matches).is_ok());

    // Assert that we now have a home_dir
    assert!( home_dir().unwrap().exists() );

}
