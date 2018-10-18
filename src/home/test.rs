// Unit tests for the home submodule
use ::{app,home};

// There's honestly not a huge amount to test in this function.
// We're content to know it succeeds.
#[test]
fn test_command_should_succeed() {

    let matches = app()
        .get_matches_from( vec!["ellipsis", "home"] );
    let sub_matches = matches.subcommand_matches("home").unwrap();

    assert!(home::run(&sub_matches).is_ok());
}

