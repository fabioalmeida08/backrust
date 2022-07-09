use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::{self, metadata, File};

pub fn compression() -> Result<String, std::io::Error> {
    let home = env::var("HOME").unwrap();

    let tar_gz = File::create("backup.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);

    tar.append_dir_all("backup", format!("{}/backuptst", home))?;
    fs::create_dir_all(format!("{}/backup", home))?;
    fs::rename("backup.tar.gz", format!("{}/backup/backup.tar.gz", home))?;

    let size = metadata(format!("{}/backup/backup.tar.gz", home))?.len() as f64;
    let size = format!("{:.2} MB", (size / 1000000.0));

    Ok(size)
}
