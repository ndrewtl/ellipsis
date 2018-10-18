use std::{env,ffi,process};
use ::{home_dir, Result};

pub fn git<I, T>(itr : I) -> Result<()>
where
I : IntoIterator<Item = T>,
T : AsRef<ffi::OsStr> + Clone {

    process::Command::new("git")
        .args(vec!["-C", &home_dir().unwrap().to_string_lossy()])
        .args(itr)
        .status()?;

    Ok(())

}

// Invoke the editor
pub fn editor<P: AsRef<ffi::OsStr>>(path : P) -> Result<()> {
    process::Command::new(
        env::var("EDITOR").unwrap_or(String::from("vi")))
        .arg(path)
        .status()?;
    Ok(())
}
