use std::io::Error;
use std::path::PathBuf;
use std::fs::{self};
use std::env::{self};

pub fn execute(dir_name: Option<String>) -> Result<(), Error> {
    let path = if let Some(dir) = dir_name {
        PathBuf::from(dir)
    } else {
        env::current_dir()?
    };
    let root = path.join(".my_own_git");
    let objects = root.join("objects");
    fs::create_dir_all(objects.join("info"))?;
    fs::create_dir_all(objects.join("pack"))?;

    Ok(())
}