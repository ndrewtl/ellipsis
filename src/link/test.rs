// Unit tests for the link submodule
use ::test::TestDirs;
use ::{app,config_file,home_dir,link};
use std::{fs,path};
use serde_json;
use shellexpand;

#[test]
fn test_links_successfully() {
    let _tdirs = TestDirs::new();


    let home = home_dir().unwrap();
    let cfg = config_file().unwrap();


    // Initialize some dummy files
    fs::create_dir(&home).unwrap();
    fs::write(&home.join("test.txt"), b"file contents").unwrap();
    fs::write(&home.join("test2.md"), b"# Title \n content").unwrap();
    fs::write(&home.join("test3.rb"), b"puts 'Hello World'").unwrap();

    // Initialize config file
    let data = json!({
        "links" : {
            "~/test.txt" : "test.txt" ,
            "~/yo/yo/yo/multilevel.md" : "test2.md",
            "~/a_long_filename_test3.rb" : "test3.rb" ,
        }
    });

    let cfgfile = fs::File::create(&cfg).unwrap();
    serde_json::to_writer_pretty(cfgfile, &data).unwrap();

    let matches = app()
        .get_matches_from( vec!["ellipsis", "link"] );
    let sub_matches = matches.subcommand_matches("link").unwrap();

    assert!(link::run(&sub_matches).is_ok());

    // Assert that all selected files exist and are symlinks
    for fname in data["links"].as_object().unwrap().keys() {

        let file = path::PathBuf::from(
            shellexpand::tilde(fname.as_str()).into_owned());


        assert!(file.exists());
        assert!(file
                .symlink_metadata()
                .unwrap()
                .file_type()
                .is_symlink());

    }
}
