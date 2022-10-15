use crate::backup_paths::backup_folders;
use chrono::{DateTime, Local};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;

pub async fn compression() -> Result<String, Box<dyn std::error::Error>> {
    let filename: DateTime<Local> = Local::now();
    let filename: String = format!("{}.tar.gz", filename.format("%d_%m_%Y"));

    let tar_gz = File::create(&filename)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);

    let paths = backup_folders()?;

    for path in paths {
        let folders = path.split("/").collect::<Vec<_>>();
        let folder_name = folders.last().unwrap();

        tar.append_dir_all(format!("backup/{}", folder_name), path)?;
    }

    Ok(filename)
}
