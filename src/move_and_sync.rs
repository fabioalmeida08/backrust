extern crate fs_extra;
use dotenv::dotenv;
use std::env;
use std::fs::rename;
use std::process::Command;
pub fn move_and_sync(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let backup_destination = env::var("BACKUP_DESTINATION")?;
    let backup_mirror_destination = env::var("BACKUP_MIRROR_DESTINATION")?;
    
    
    rename(&filename, format!("{}{}", backup_destination, &filename))?;

    _ = Command::new("rsync")
        .args([
            "-r",
            "--delete",
            &backup_destination,
            &backup_mirror_destination,
        ]).output()?;
        
    Ok(())
}
