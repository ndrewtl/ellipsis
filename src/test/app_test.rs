use app;
// Test that these commands are ok
#[test]
fn test_valid_commands() {
    let valid = vec![
        "ellipsis config --edit",
        "ellipsis config -e",
        "ellipsis destroy --force",
        "ellipsis destroy -f",
        "ellipsis destroy --dry-run",
        "ellipsis destroy -n",
        "ellipsis home",
        "ellipsis init https://github.com/ndrewtl/dotfiles.git",
        "ellipsis link",
        "ellipsis unlink",
    ];

    for arg in valid {
        let matches = app()
            .get_matches_from_safe(arg.split_whitespace());
        assert!(matches.is_ok());
    }
}

// Test that these commands are not ok
#[test]
fn test_invalid_commands() {
    let invalid = vec![
        "very obviously not a command",
        "ellipsis config --edit --edit",
        "ellipsis config --invalidarg",
        "ellipsis config -short",
        "ellipsis destroy", // Can't run destroy without either -n or -f
        "ellipsis destroy --invalidarg",
        "ellipsis destroy -fn",
        "ellipsis home --longarg",
        "ellipsis home -x",
        "ellipsis init",
        "ellipsis link -x -a",
        "ellipsis unlink --longarg",
        "ellipsis invalid_command"
    ];

    for arg in invalid {
        let matches = app()
            .get_matches_from_safe(arg.split_whitespace());
        assert!(matches.is_err());
    }
}
