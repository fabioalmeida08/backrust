use std::{env, fs::DirEntry, path::PathBuf, io::Error};
use dotenv::dotenv;
use std::fs;

use chrono::{Local, NaiveDate, Datelike};

fn compare_dates(file_name: &str) -> i64 {
    let now = Local::now();
    let file_date = NaiveDate::parse_from_str(&file_name, "%d_%m_%Y").unwrap();
    let naive_now = NaiveDate::from_ymd(
        now.year().try_into().unwrap(),
        now.month().try_into().unwrap(),
        now.day().try_into().unwrap(),
    );

    let diff = naive_now - file_date;

    diff.num_days()
}

fn get_files_names() -> Vec<DirEntry> {
    dotenv().ok();
    let backup_path = env::var("BACKUP_PATH").unwrap();
    fs::read_dir(&backup_path)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn check_files_and_delete(files: Vec<DirEntry>) {
    for entry in files {
        let file_path = entry.path();
        let file_name_date = entry.file_name().into_string().unwrap();
        let file_name_date = file_name_date.replace(".tar.gz", "");

        let days = compare_dates(&file_name_date);
        
        if days >= 7 {
            delete_file(file_path).unwrap();
        }
    }
}

fn delete_file(file_dir: PathBuf) -> Result<(), Error> {
    fs::remove_file(file_dir)?;
    Ok(())
}

pub fn check_delete() {
    let files = get_files_names();
    check_files_and_delete(files);
}
