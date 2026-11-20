use crate::utils::input::read_stdin;

use std::path::PathBuf;
use std::fs::{self};
use sha1::{Sha1, Digest};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;

pub fn execute(file: Option<PathBuf>, w: bool, stdin: bool) -> Result<(), std::io::Error> {
    let content = if stdin {
        read_stdin()?
    } else if let Some(path) = file {
        fs::read_to_string(path)?
    } else {
        unreachable!()
    };
    let object = format!("blob {}\0{}", content.len(), content).into_bytes();

    let mut hasher = Sha1::new();
    hasher.update(&object);
    let result = hasher.finalize();
    let hex_string = hex::encode(result);

    if w {
        let mut path = PathBuf::from(".my_own_git/objects");
        let subfolder = &hex_string[..2];
        path.push(subfolder);
        fs::create_dir_all(&path)?;

        let file_name = &hex_string[hex_string.len() - 38..];
        let file_path = path.join(file_name);

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&object)?;
        let compressed_bytes = encoder.finish()?;

        fs::write(&file_path, compressed_bytes)?;
    }
    
    println!("{}", hex_string);
    Ok(())
}