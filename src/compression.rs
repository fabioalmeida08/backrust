use crate::backup_paths::backup_folders;
use chrono::{DateTime, Local};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::{self, File};
use fs_extra::dir;

pub fn compression() -> Result<String, std::io::Error> {
    let home = env::var("HOME").unwrap();
    let mut log = String::new();

    let filename: DateTime<Local> = Local::now();
    let filename: String = format!("{}.tar.gz", filename.format("%d_%m_%Y"));

    let tar_gz = File::create(&filename)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);

    let paths = backup_folders().unwrap();

    for path in paths {
        let folders = path.split("/").collect::<Vec<_>>();
        let folder_name = folders.last().unwrap();
        
        let path_size = dir::get_size(&path).unwrap();
        let path_size_in_mb = format!("{} with {:.2}MB \n", &path, path_size as f32 / 1048576.0);
        
        log.push_str(&path_size_in_mb);
        
        tar.append_dir_all(format!("backup/{}", folder_name), format!("{}", path))?;
    }

    fs::create_dir_all(format!("{}/backup", home))?;
    fs::rename(&filename, format!("{}/backup/{}", home, filename))?;

    let total_size = dir::get_size(format!("{}/backup/{}", home, filename)).unwrap();
    let total_size = format!("Total Backup Size: {:.2} MB", (total_size as f64 / 1048576.0));

    log.push_str(&total_size);

    Ok(log)
}
