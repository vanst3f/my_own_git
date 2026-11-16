use std::fs::{self, File};
use std::path::PathBuf;
use sha1::{Sha1, Digest};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;

pub fn execute(file: PathBuf, w: bool) -> Result<(), std::io::Error> {
    let content = fs::read_to_string(&file)?;
    let object = format!("blob {}\0{}", content.len(), content).into_bytes();

    let mut hasher = Sha1::new();
    hasher.update(&object);
    let result = hasher.finalize();
    let hex_string = hex::encode(result);

    if w {
        let dir = &hex_string[..2];
        let file_name = &hex_string[hex_string.len() - 38..];

        let mut path = PathBuf::from(dir);
        fs::create_dir(&path)?;
        path.push(file_name);
        File::create(&path)?;

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&object)?;
        let compressed_bytes = encoder.finish()?;

        fs::write(&path, compressed_bytes)?;
    }
    
    println!("{}", hex_string);
    Ok(())
}