use std::{fs,path,os};
use serde_json;
use shellexpand;
use ::{config_file, home_dir, test, Scope};

#[test]
fn test_should_remove_symlinks() {
    let _tdirs = test::TestDirs::new();

    // Create home dir
    let home = home_dir().unwrap();
    let cfg  = config_file(Scope::Local).unwrap();
    fs::create_dir(&home).unwrap();


    // Create test files
    fs::write(&home.join("test.txt"), b"file contents").unwrap();
    fs::write(&home.join("test2.js"), b"module.exports = 5").unwrap();
    fs::create_dir(&home.join("folder")).unwrap();
    fs::write(&home.join("folder/test3.py"), b"print('hello')").unwrap();

    // Initialize config file
    let data = json!({
        "links" : {
            "test.txt" : "~/link1",
            "test2.js": "~/link2.js",
            "folder/test3.py" : "~/yo.py"
        }
    });
    let cfgfile = fs::File::create(&cfg).unwrap();
    serde_json::to_writer_pretty(cfgfile, &data).unwrap();



    // Link all the test files
    for (source, dest) in data["links"].as_object().unwrap() {

        let source = path::PathBuf::from(home_dir().unwrap())
            .join(source.as_str());

        let dest = path::PathBuf::from(
            shellexpand::tilde(dest.as_str().unwrap()).into_owned());

        println!("{:?}", source);

        os::unix::fs::symlink(source ,dest).unwrap();
    }

    // Assert that we've created all these files
    for file in data["links"].as_object().unwrap().values() {
        let file = path::PathBuf::from(
            shellexpand::tilde(file.as_str().unwrap()).into_owned());
        assert!(file.exists());
        assert!(file
                .symlink_metadata()
                .unwrap()
                .file_type()
                .is_symlink());
    }

    // Run the command
    assert!(test::run_cmd(vec!["ellipsis", "unlink"]).is_ok());


    // Assert that we've deleted all the files
    for file in data["links"].as_object().unwrap().values() {
        let file = path::PathBuf::from(
            shellexpand::tilde(file.as_str().unwrap()).into_owned());
        assert!(! file.exists());
    }
}


#[test]
#[should_panic]
fn test_should_not_remove_files() {
    let tdirs = test::TestDirs::new();

    // Create home dir
    let home = home_dir().unwrap();
    let cfg  = config_file(Scope::Local).unwrap();
    fs::create_dir(&home).unwrap();


    // Create test file
    fs::write(&tdirs.home_dir().join("test.txt"), b"file contents").unwrap();

    // Initialize config file
    let data = json!({
        "links" : {
            "test.txt" : "~/test.txt"
        }
    });
    let cfgfile = fs::File::create(&cfg).unwrap();
    serde_json::to_writer_pretty(cfgfile, &data).unwrap();

    // Run the command
    test::run_cmd(vec!["ellipsis", "unlink"]).is_ok();

}



#[test]
#[should_panic]
fn test_should_not_remove_directories() {
    let tdirs = test::TestDirs::new();

    // Create home dir
    let home = home_dir().unwrap();
    let cfg  = config_file(Scope::Local).unwrap();
    fs::create_dir(&home).unwrap();


    let test_dir = "test_dir";

    // Create test dir
    fs::create_dir(tdirs.home_dir().join(test_dir)).unwrap();

    // Initialize config file
    let data = json!({
        "links" : {
            "test.txt" : format!("~/{}", test_dir)
        }
    });
    let cfgfile = fs::File::create(&cfg).unwrap();
    serde_json::to_writer_pretty(cfgfile, &data).unwrap();

    // Run the command
    test::run_cmd(vec!["ellipsis", "unlink"]).is_ok();

}
