use ::test::rand;
use std::{env, fs, path};

// A struct to create and destroy fake directories for testing
// This struct creates new throwaway directories for $HOME and $XDG_DATA_HOME
// It implements the Drop trait to delete these directories afterwards
// ~Always~ create using the ::new method
pub struct TestDirs {
    data_original : Option<String>,
    home_original : Option<String>,
    data : path::PathBuf, // The XDG_DATA_DIR
    home : path::PathBuf,
}

impl TestDirs {

    pub fn new() -> Self {

        let tmp = env::temp_dir();

        let rn = rand::random::<u32>();

        // Name our new fake directories
        let tmp_home = tmp.join(
            format!("ellipsis-test-home-{}", rn));
        let tmp_data = tmp.join(
            format!("ellipsis-test-data-{}", rn));

        // Create them
        fs::create_dir(&tmp_home).unwrap();
        fs::create_dir(&tmp_data).unwrap();

        // Set our environment variables to point at the new environment variables
        // Save original env variables, then set the new ones
        let home_original = env::var("HOME").ok();
        let data_original = env::var("XDG_DATA_HOME").ok();
        env::set_var("HOME", tmp_home.to_str().unwrap());
        env::set_var("XDG_DATA_HOME", tmp_data.to_str().unwrap());

        Self {
            home_original,
            data_original,
            home: tmp_home,
            data: tmp_data
        }
    }

    pub fn data_dir(&self) -> &path::Path {
        self.data.as_path()
    }

    pub fn home_dir(&self) -> &path::Path {
        self.home.as_path()
    }
}

impl Drop for TestDirs {
    fn drop(&mut self) {
        // When this falls out of scope, delete the test directories
        fs::remove_dir_all(&self.home).unwrap();
        fs::remove_dir_all(&self.data).unwrap();

        if let Some(dirname) = &self.data_original {
            env::set_var("XDG_DATA_HOME", dirname);
        }

        if let Some(dirname) = &self.home_original {
            env::set_var("HOME", dirname);
        }
    }
}
