use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::Read;

pub fn backup_folders() -> Result<Vec<String>, std::io::Error> {
    dotenv().ok();
    
    let backup_file_paths = env::var("BACKUP_PATHS_FILE").unwrap();
    let mut str = String::new();
    let mut file = File::open(backup_file_paths).expect("Error in reading file");
    file.read_to_string(&mut str).expect("Unable to read line");
    
    let paths: Vec<String> = str.split('\n').map(|x| x.to_string()).collect();

    Ok(paths)
}
