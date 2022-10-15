use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::Read;

pub fn backup_folders() -> Result<Vec<String>, std::io::Error> {
    dotenv().ok();

    let backup_dir = env::var("DIRS_TO_BACKUP_FILE").unwrap();
    let mut string = String::new();
    let mut file = File::open(backup_dir).expect("Error when reading file");

    file.read_to_string(&mut string)
        .expect("unable to read line");

    let mut paths: Vec<String> = string.split('\n').map(|x| x.to_string()).collect();
    paths.pop();

    Ok(paths)
}
